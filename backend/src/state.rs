use crate::ai::{ChatMessage, ModelConfig, OpenAiCompatBackend, ProviderRegistry, ProviderRouter};
use crate::config::AppConfig;
use crate::domain::conversation::{
    ArchiveConversation, ChangeTitle, CompleteAssistantResponse, Conversation,
    ConversationArchived, ConversationCommand, ConversationError, ConversationEvent,
    ConversationServices, ConversationStarted, SendUserMessage, StartConversation, TitleChanged,
    UserMessageSent,
};
use crate::domain::{ConversationHistoryView, ConversationListEntry, ConversationMemoryView};
use crate::query::conversation_queries;
use anyhow::Context;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::{Aggregate, CqrsFramework, DomainEvent};
use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime};
use password_auth::{generate_hash, verify_password};
use postgres_es::PostgresEventRepository;
use redis::AsyncCommands;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Owner,
    User,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicUser {
    pub id: String,
    pub username: String,
    pub role: UserRole,
}

#[derive(Debug, Clone)]
pub struct UserRecord {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
}

#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub user_id: String,
}

#[derive(Debug, Clone)]
pub struct ConversationRecord {
    pub user_id: String,
    pub events: Vec<ConversationEvent>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub users: Arc<RwLock<HashMap<String, UserRecord>>>,
    pub sessions: Arc<RwLock<HashMap<String, SessionRecord>>>,
    pub conversations: Arc<RwLock<HashMap<String, ConversationRecord>>>,
    pub registry: Arc<ProviderRegistry>,
    pub router: Arc<ProviderRouter>,
    pub db: Option<PgPool>,
    pub redis: Option<RedisPool>,
    pub cqrs: Option<Arc<CqrsFramework<Conversation, PersistedEventStore<PostgresEventRepository, Conversation>>>>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let mut registry = ProviderRegistry::new();
        registry.register(ModelConfig::new(
            config.ai_model.clone(),
            config.ai_provider.clone(),
            config.ai_base_url.clone(),
        ));

        let mut router = ProviderRouter::new();
        if !config.ai_api_key.is_empty() {
            router.register(
                config.ai_model.clone(),
                Box::new(OpenAiCompatBackend::new(
                    config.ai_base_url.clone(),
                    config.ai_api_key.clone(),
                    30,
                )),
            );
        }

        let db = if let Some(database_url) = &config.database_url {
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await
                .with_context(|| format!("failed connecting to postgres at {database_url}"))?;
            sqlx::migrate!("./migrations").run(&pool).await?;
            Some(pool)
        } else {
            None
        };

        let redis = if let Some(redis_url) = &config.redis_url {
            let pool = RedisConfig::from_url(redis_url.clone())
                .create_pool(Some(Runtime::Tokio1))
                .context("failed creating redis pool")?;
            Some(pool)
        } else {
            None
        };

        let cqrs = if let Some(pool) = db.clone() {
            let repo = PostgresEventRepository::new(pool.clone()).with_tables("cqrs_events", "cqrs_snapshots");
            let store = PersistedEventStore::new_snapshot_store(repo, 50);
            Some(Arc::new(CqrsFramework::new(
                store,
                conversation_queries(pool),
                ConversationServices,
            )))
        } else {
            None
        };

        let state = Self {
            config: config.clone(),
            users: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            conversations: Arc::new(RwLock::new(HashMap::new())),
            registry: Arc::new(registry),
            router: Arc::new(router),
            db,
            redis,
            cqrs,
        };

        state
            .seed_user(&config.owner_username, &config.owner_password, UserRole::Owner)
            .await?;

        Ok(state)
    }

    pub async fn seed_user(
        &self,
        username: &str,
        password: &str,
        role: UserRole,
    ) -> anyhow::Result<()> {
        if username.trim().is_empty() || password.trim().is_empty() {
            anyhow::bail!("username and password are required");
        }

        if let Some(db) = &self.db {
            let existing = sqlx::query("SELECT id FROM users WHERE username = $1")
                .bind(username)
                .fetch_optional(db)
                .await?;

            if existing.is_some() {
                if matches!(role, UserRole::Owner) {
                    return Ok(());
                }
                anyhow::bail!("user already exists");
            }

            sqlx::query(
                "INSERT INTO users (username, password_hash, system_prompt, is_owner, role) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(username)
            .bind(generate_hash(password))
            .bind(match role {
                UserRole::Owner => self.config.owner_system_prompt.clone(),
                UserRole::User => self.config.user_system_prompt.clone(),
            })
            .bind(matches!(role, UserRole::Owner))
            .bind(match role {
                UserRole::Owner => "owner",
                UserRole::User => "user",
            })
            .execute(db)
            .await?;

            return Ok(());
        }

        let Ok(mut users) = self.users.try_write() else {
            return Ok(());
        };
        if users.values().any(|user| user.username == username) {
            if matches!(role, UserRole::Owner) {
                return Ok(());
            }
            anyhow::bail!("user already exists");
        }

        let id = Uuid::new_v4().to_string();
        users.insert(
            id.clone(),
            UserRecord {
                id,
                username: username.to_string(),
                password_hash: generate_hash(password),
                role,
            },
        );

        Ok(())
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Option<PublicUser> {
        if let Some(db) = &self.db {
            let row = sqlx::query(
                "SELECT id, username, password_hash, is_owner FROM users WHERE username = $1 AND is_active = TRUE",
            )
            .bind(username)
            .fetch_optional(db)
            .await
            .ok()??;

            let password_hash: String = row.get("password_hash");
            verify_password(password, &password_hash).ok()?;

            return Some(PublicUser {
                id: row.get::<Uuid, _>("id").to_string(),
                username: row.get("username"),
                role: if row.get::<bool, _>("is_owner") {
                    UserRole::Owner
                } else {
                    UserRole::User
                },
            });
        }

        let users = self.users.read().await;
        users
            .values()
            .find(|user| user.username == username)
            .and_then(|user| {
                verify_password(password, &user.password_hash)
                    .ok()
                    .map(|_| self.to_public_user(user))
            })
    }

    pub async fn create_session(&self, user: &PublicUser) -> String {
        let token = Uuid::new_v4().to_string();

        if let Some(db) = &self.db {
            let _ = sqlx::query(
                "INSERT INTO sessions (user_id, token_hash, expires_at) VALUES ($1, $2, NOW() + INTERVAL '30 days')",
            )
            .bind(Uuid::parse_str(&user.id).expect("valid user uuid"))
            .bind(&token)
            .execute(db)
            .await;

            return token;
        }

        self.sessions.write().await.insert(
            token.clone(),
            SessionRecord {
                user_id: user.id.clone(),
            },
        );
        token
    }

    pub async fn get_user_by_session(&self, token: &str) -> Option<PublicUser> {
        if let Some(db) = &self.db {
            let row = sqlx::query(
                "SELECT users.id, users.username, users.is_owner
                 FROM sessions
                 JOIN users ON users.id = sessions.user_id
                 WHERE sessions.token_hash = $1 AND sessions.expires_at > NOW()",
            )
            .bind(token)
            .fetch_optional(db)
            .await
            .ok()??;

            return Some(PublicUser {
                id: row.get::<Uuid, _>("id").to_string(),
                username: row.get("username"),
                role: if row.get::<bool, _>("is_owner") {
                    UserRole::Owner
                } else {
                    UserRole::User
                },
            });
        }

        let sessions = self.sessions.read().await;
        let session = sessions.get(token)?;
        let users = self.users.read().await;
        users
            .get(&session.user_id)
            .map(|user| self.to_public_user(user))
    }

    pub async fn destroy_session(&self, token: &str) {
        if let Some(db) = &self.db {
            let _ = sqlx::query("DELETE FROM sessions WHERE token_hash = $1")
                .bind(token)
                .execute(db)
                .await;
            return;
        }

        self.sessions.write().await.remove(token);
    }

    pub async fn create_conversation(&self, user: &PublicUser, title: Option<String>) -> String {
        let conversation_id = Uuid::new_v4().to_string();
        let title = title.unwrap_or_else(|| chrono::Utc::now().format("Chat %Y-%m-%d %H:%M").to_string());
        let event = ConversationEvent::Started(ConversationStarted {
            conversation_id: conversation_id.clone(),
            user_id: user.id.clone(),
            title,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });

        if let Some(cqrs) = &self.cqrs {
            let command = ConversationCommand::Start(StartConversation {
                conversation_id: conversation_id.clone(),
                user_id: user.id.clone(),
                title: match &event {
                    ConversationEvent::Started(started) => started.title.clone(),
                    _ => unreachable!(),
                },
                timestamp: match &event {
                    ConversationEvent::Started(started) => started.timestamp.clone(),
                    _ => unreachable!(),
                },
            });
            let _ = cqrs.execute(&conversation_id, command).await;
            return conversation_id;
        }

        self.conversations.write().await.insert(
            conversation_id.clone(),
            ConversationRecord {
                user_id: user.id.clone(),
                events: vec![event],
            },
        );

        conversation_id
    }

    pub async fn rename_conversation(
        &self,
        user: &PublicUser,
        conversation_id: &str,
        title: String,
    ) -> Result<(), ApiStateError> {
        if let Some(cqrs) = &self.cqrs {
            let history = self.get_history(user, conversation_id).await?;
            if history.user_id.as_deref() != Some(user.id.as_str()) {
                return Err(ApiStateError::Forbidden);
            }
            cqrs.execute(
                conversation_id,
                ConversationCommand::ChangeTitle(ChangeTitle {
                    title,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }),
            )
            .await
            .map_err(map_cqrs_error)?;
            return Ok(());
        }

        let mut conversations = self.conversations.write().await;
        let record = conversations
            .get_mut(conversation_id)
            .ok_or(ApiStateError::NotFound)?;
        self.ensure_owner(user, record)?;
        record
            .events
            .push(ConversationEvent::TitleChanged(TitleChanged { title }));
        Ok(())
    }

    pub async fn archive_conversation(
        &self,
        user: &PublicUser,
        conversation_id: &str,
    ) -> Result<(), ApiStateError> {
        if let Some(cqrs) = &self.cqrs {
            let history = self.get_history(user, conversation_id).await?;
            if history.archived {
                return Err(ApiStateError::Conflict("conversation already archived".to_string()));
            }
            cqrs.execute(
                conversation_id,
                ConversationCommand::ArchiveConversation(ArchiveConversation {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }),
            )
            .await
            .map_err(map_cqrs_error)?;
            return Ok(());
        }

        let record = self
            .load_conversation_record(conversation_id)
            .await?
            .ok_or(ApiStateError::NotFound)?;
        self.ensure_owner(user, &record)?;
        if self.aggregate_from_events(&record.events).archived {
            return Err(ApiStateError::Conflict("conversation already archived".to_string()));
        }

        if self.db.is_some() {
            self.append_event(
                conversation_id,
                user,
                ConversationEvent::ConversationArchived(ConversationArchived {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }),
            )
            .await?;
            return Ok(());
        }

        let mut conversations = self.conversations.write().await;
        let record = conversations
            .get_mut(conversation_id)
            .ok_or(ApiStateError::NotFound)?;
        record
            .events
            .push(ConversationEvent::ConversationArchived(ConversationArchived {
                timestamp: chrono::Utc::now().to_rfc3339(),
            }));
        Ok(())
    }

    pub async fn get_history(
        &self,
        user: &PublicUser,
        conversation_id: &str,
    ) -> Result<ConversationHistoryView, ApiStateError> {
        if let Some(history) = self.load_history_view(conversation_id).await? {
            if history.user_id.as_deref() == Some(user.id.as_str()) {
                return Ok(history);
            }
        }

        let record = self
            .load_conversation_record(conversation_id)
            .await?
            .ok_or(ApiStateError::NotFound)?;
        self.ensure_owner(user, &record)?;
        Ok(ConversationHistoryView::from_events(conversation_id, &record.events))
    }

    pub async fn list_conversations(&self, user: &PublicUser) -> Vec<ConversationListEntry> {
        if let Ok(entries) = self.load_list_entries(user).await {
            if !entries.is_empty() {
                return entries;
            }
        }

        let records = self.load_all_conversations_for_user(user).await.unwrap_or_default();
        let mut entries = records
            .iter()
            .map(|(id, record)| ConversationListEntry::from_events(id, &record.events))
            .filter(|entry| !entry.archived)
            .collect::<Vec<_>>();
        entries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        entries
    }

    pub async fn add_user_message(
        &self,
        user: &PublicUser,
        conversation_id: &str,
        content: String,
        model_id: Option<String>,
    ) -> Result<String, ApiStateError> {
        if let Some(cqrs) = &self.cqrs {
            let history = self.get_history(user, conversation_id).await?;
            if history.archived {
                return Err(ApiStateError::Conflict("conversation is archived".to_string()));
            }
            if content.trim().is_empty() {
                return Err(ApiStateError::BadRequest("message content cannot be empty".to_string()));
            }
            let model_id = model_id.unwrap_or_else(|| {
                history
                    .messages
                    .last()
                    .and_then(|message| message.model_id.clone())
                    .unwrap_or_else(|| self.config.ai_model.clone())
            });
            cqrs.execute(
                conversation_id,
                ConversationCommand::SendUserMessage(SendUserMessage {
                    content,
                    user_id: user.id.clone(),
                    model_id: model_id.clone(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }),
            )
            .await
            .map_err(map_cqrs_error)?;
            return Ok(model_id);
        }

        let record = self
            .load_conversation_record(conversation_id)
            .await?
            .ok_or(ApiStateError::NotFound)?;
        self.ensure_owner(user, &record)?;

        let aggregate = self.aggregate_from_events(&record.events);
        if aggregate.archived {
            return Err(ApiStateError::Conflict("conversation is archived".to_string()));
        }
        if content.trim().is_empty() {
            return Err(ApiStateError::BadRequest("message content cannot be empty".to_string()));
        }

        let model_id = model_id.unwrap_or_else(|| {
            aggregate
                .last_model_id
                .unwrap_or_else(|| self.config.ai_model.clone())
        });

        let event = ConversationEvent::UserMessageSent(UserMessageSent {
            content,
            user_id: user.id.clone(),
            model_id: model_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        });

        if self.db.is_some() {
            self.append_event(conversation_id, user, event).await?;
        } else {
            let mut conversations = self.conversations.write().await;
            let record = conversations
                .get_mut(conversation_id)
                .ok_or(ApiStateError::NotFound)?;
            record.events.push(event);
        }

        Ok(model_id)
    }

    pub async fn add_assistant_message(
        &self,
        user: &PublicUser,
        conversation_id: &str,
        content: String,
        model_id: String,
    ) -> Result<(), ApiStateError> {
        if let Some(cqrs) = &self.cqrs {
            let history = self.get_history(user, conversation_id).await?;
            if history.user_id.as_deref() != Some(user.id.as_str()) {
                return Err(ApiStateError::Forbidden);
            }
            cqrs.execute(
                conversation_id,
                ConversationCommand::CompleteAssistantResponse(CompleteAssistantResponse {
                    content,
                    model_id,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }),
            )
            .await
            .map_err(map_cqrs_error)?;
        } else {
            let event = crate::domain::conversation::ConversationEvent::AssistantMessageSent(
                crate::domain::conversation::AssistantMessageSent {
                    content,
                    model_id,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
            );
            let mut conversations = self.conversations.write().await;
            let record = conversations
                .get_mut(conversation_id)
                .ok_or(ApiStateError::NotFound)?;
            self.ensure_owner(user, record)?;
            record.events.push(event);
        }

        Ok(())
    }

    pub async fn maybe_generate_title(
        &self,
        user: &PublicUser,
        conversation_id: &str,
    ) -> Result<(), ApiStateError> {
        let history = self.get_history(user, conversation_id).await?;
        let placeholder_title = history.title.starts_with("Chat ") || history.title.starts_with("Conversation ");
        if !placeholder_title || history.messages.len() < 2 {
            return Ok(());
        }

        let first_user_message = history
            .messages
            .iter()
            .find(|message| message.role == "user")
            .map(|message| message.content.clone())
            .unwrap_or_default();
        if first_user_message.is_empty() {
            return Ok(());
        }

        let title = self.generate_conversation_title(&first_user_message).await;
        if title.trim().is_empty() {
            return Ok(());
        }

        self.rename_conversation(user, conversation_id, title).await
    }

    pub async fn conversation_memory_for_user(
        &self,
        user: &PublicUser,
        current_conversation_id: &str,
    ) -> Vec<ConversationMemoryView> {
        if let Ok(entries) = self.load_memory_views(user, current_conversation_id).await {
            if !entries.is_empty() {
                return entries;
            }
        }

        let records = self.load_all_conversations_for_user(user).await.unwrap_or_default();
        records
            .iter()
            .filter(|(id, _)| id.as_str() != current_conversation_id)
            .map(|(id, record)| ConversationMemoryView::from_events(id, &record.events, 4))
            .filter(|memory| !memory.archived && !memory.recent_messages.is_empty())
            .collect()
    }

    pub async fn generate_assistant_reply(
        &self,
        user: &PublicUser,
        conversation_id: &str,
        content: &str,
        model_id: &str,
    ) -> String {
        let memory = self.conversation_memory_for_user(user, conversation_id).await;
        let system_prompt = match user.role {
            UserRole::Owner => &self.config.owner_system_prompt,
            UserRole::User => &self.config.user_system_prompt,
        };

        let mut messages = vec![ChatMessage::new("system", system_prompt)];
        for entry in memory.iter().take(2) {
            if let Some(summary) = &entry.summary {
                messages.push(ChatMessage::new("system", format!("Memory: {summary}")));
            }
        }
        messages.push(ChatMessage::new("user", content.to_string()));

        match self.router.complete(model_id, messages).await {
            Ok(reply) => reply,
            Err(_) => {
                let memory_hint = memory
                    .first()
                    .and_then(|item| item.summary.clone())
                    .map(|summary| format!(" I also remember: {summary}"))
                    .unwrap_or_default();
                format!("You said: {content}.{memory_hint}")
            }
        }
    }

    async fn generate_conversation_title(&self, first_user_message: &str) -> String {
        let prompt = format!(
            "Write a short conversation title of 2 to 5 words for this request. Return only the title. Request: {}",
            first_user_message
        );

        match self
            .router
            .complete(
                &self.config.ai_model,
                vec![
                    ChatMessage::new("system", "You write concise chat titles."),
                    ChatMessage::new("user", prompt),
                ],
            )
            .await
        {
            Ok(title) => sanitize_title(&title),
            Err(_) => fallback_title(first_user_message),
        }
    }

    pub async fn health_report(&self) -> (String, String) {
        let database = if let Some(db) = &self.db {
            match sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(db).await {
                Ok(_) => "ok".to_string(),
                Err(_) => "error".to_string(),
            }
        } else {
            "in_memory".to_string()
        };

        let redis = if let Some(pool) = &self.redis {
            match pool.get().await {
                Ok(mut conn) => match conn.set::<_, _, ()>("orchid:health", "ok").await {
                    Ok(_) => "ok".to_string(),
                    Err(_) => "error".to_string(),
                },
                Err(_) => "error".to_string(),
            }
        } else {
            "disabled".to_string()
        };

        (database, redis)
    }

    async fn append_event(
        &self,
        conversation_id: &str,
        user: &PublicUser,
        event: ConversationEvent,
    ) -> Result<(), ApiStateError> {
        let Some(db) = &self.db else {
            return Err(ApiStateError::Conflict("database unavailable".to_string()));
        };

        let aggregate_id = Uuid::parse_str(conversation_id)
            .map_err(|_| ApiStateError::BadRequest("invalid conversation id".to_string()))?;
        let user_id = Uuid::parse_str(&user.id)
            .map_err(|_| ApiStateError::BadRequest("invalid user id".to_string()))?;

        let current_version = sqlx::query_scalar::<_, Option<i64>>(
            "SELECT MAX(version) FROM events WHERE aggregate_id = $1 AND aggregate_type = 'conversation'",
        )
        .bind(aggregate_id)
        .fetch_one(db)
        .await
        .map_err(|error| ApiStateError::Conflict(error.to_string()))?
        .unwrap_or(0);

        sqlx::query(
            "INSERT INTO events (aggregate_id, aggregate_type, event_type, version, timestamp, user_id, data, metadata)
             VALUES ($1, 'conversation', $2, $3, NOW(), $4, $5, $6)",
        )
        .bind(aggregate_id)
        .bind(event.event_type())
        .bind(current_version + 1)
        .bind(user_id)
        .bind(serde_json::to_value(&event).map_err(|error| ApiStateError::Conflict(error.to_string()))?)
        .bind(serde_json::json!({ "event_version": event.event_version() }))
        .execute(db)
        .await
        .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

        self.refresh_views(conversation_id, user).await?;

        Ok(())
    }

    async fn refresh_views(
        &self,
        conversation_id: &str,
        user: &PublicUser,
    ) -> Result<(), ApiStateError> {
        let Some(db) = &self.db else {
            return Ok(());
        };

        let record = self
            .load_conversation_record(conversation_id)
            .await?
            .ok_or(ApiStateError::NotFound)?;
        let history = ConversationHistoryView::from_events(conversation_id, &record.events);
        let list_entry = ConversationListEntry::from_events(conversation_id, &record.events);
        let memory = ConversationMemoryView::from_events(conversation_id, &record.events, 4);
        let conversation_uuid = Uuid::parse_str(conversation_id)
            .map_err(|_| ApiStateError::BadRequest("invalid conversation id".to_string()))?;
        let user_uuid = Uuid::parse_str(&user.id)
            .map_err(|_| ApiStateError::BadRequest("invalid user id".to_string()))?;

        sqlx::query(
            "INSERT INTO conversation_history_views (conversation_id, user_id, title, archived, updated_at, payload)
             VALUES ($1, $2, $3, $4, $5::timestamptz, $6)
             ON CONFLICT (conversation_id)
             DO UPDATE SET title = EXCLUDED.title, archived = EXCLUDED.archived, updated_at = EXCLUDED.updated_at, payload = EXCLUDED.payload",
        )
        .bind(conversation_uuid)
        .bind(user_uuid)
        .bind(&history.title)
        .bind(history.archived)
        .bind(&history.updated_at)
        .bind(serde_json::to_value(&history).map_err(|error| ApiStateError::Conflict(error.to_string()))?)
        .execute(db)
        .await
        .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

        sqlx::query(
            "INSERT INTO conversation_list_entries (conversation_id, user_id, title, last_message_preview, archived, updated_at, payload)
             VALUES ($1, $2, $3, $4, $5, $6::timestamptz, $7)
             ON CONFLICT (conversation_id)
             DO UPDATE SET title = EXCLUDED.title, last_message_preview = EXCLUDED.last_message_preview, archived = EXCLUDED.archived, updated_at = EXCLUDED.updated_at, payload = EXCLUDED.payload",
        )
        .bind(conversation_uuid)
        .bind(user_uuid)
        .bind(&list_entry.title)
        .bind(&list_entry.last_message_preview)
        .bind(list_entry.archived)
        .bind(&list_entry.updated_at)
        .bind(serde_json::to_value(&list_entry).map_err(|error| ApiStateError::Conflict(error.to_string()))?)
        .execute(db)
        .await
        .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

        sqlx::query(
            "INSERT INTO conversation_memory_views (conversation_id, user_id, archived, summary, updated_at, payload)
             VALUES ($1, $2, $3, $4, NOW(), $5)
             ON CONFLICT (conversation_id)
             DO UPDATE SET archived = EXCLUDED.archived, summary = EXCLUDED.summary, updated_at = EXCLUDED.updated_at, payload = EXCLUDED.payload",
        )
        .bind(conversation_uuid)
        .bind(user_uuid)
        .bind(memory.archived)
        .bind(memory.summary.clone())
        .bind(serde_json::to_value(&memory).map_err(|error| ApiStateError::Conflict(error.to_string()))?)
        .execute(db)
        .await
        .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

        Ok(())
    }

    async fn load_history_view(
        &self,
        conversation_id: &str,
    ) -> Result<Option<ConversationHistoryView>, ApiStateError> {
        let Some(db) = &self.db else {
            return Ok(None);
        };
        let conversation_uuid = match Uuid::parse_str(conversation_id) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };

        let row = sqlx::query("SELECT payload FROM conversation_history_views WHERE conversation_id = $1")
            .bind(conversation_uuid)
            .fetch_optional(db)
            .await
            .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

        row.map(|row| {
            let payload: serde_json::Value = row.get("payload");
            serde_json::from_value(payload).map_err(|error| ApiStateError::Conflict(error.to_string()))
        })
        .transpose()
    }

    async fn load_list_entries(
        &self,
        user: &PublicUser,
    ) -> Result<Vec<ConversationListEntry>, ApiStateError> {
        let Some(db) = &self.db else {
            return Ok(Vec::new());
        };
        let user_uuid = Uuid::parse_str(&user.id)
            .map_err(|_| ApiStateError::BadRequest("invalid user id".to_string()))?;
        let rows = sqlx::query(
            "SELECT payload FROM conversation_list_entries WHERE user_id = $1 AND archived = FALSE ORDER BY updated_at DESC",
        )
        .bind(user_uuid)
        .fetch_all(db)
        .await
        .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

        rows.into_iter()
            .map(|row| {
                let payload: serde_json::Value = row.get("payload");
                serde_json::from_value(payload).map_err(|error| ApiStateError::Conflict(error.to_string()))
            })
            .collect()
    }

    async fn load_memory_views(
        &self,
        user: &PublicUser,
        current_conversation_id: &str,
    ) -> Result<Vec<ConversationMemoryView>, ApiStateError> {
        let Some(db) = &self.db else {
            return Ok(Vec::new());
        };
        let user_uuid = Uuid::parse_str(&user.id)
            .map_err(|_| ApiStateError::BadRequest("invalid user id".to_string()))?;
        let current_uuid = Uuid::parse_str(current_conversation_id).ok();
        let rows = sqlx::query(
            "SELECT conversation_id, payload FROM conversation_memory_views WHERE user_id = $1 AND archived = FALSE ORDER BY updated_at DESC",
        )
        .bind(user_uuid)
        .fetch_all(db)
        .await
        .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

        rows.into_iter()
            .filter(|row| Some(row.get::<Uuid, _>("conversation_id")) != current_uuid)
            .map(|row| {
                let payload: serde_json::Value = row.get("payload");
                serde_json::from_value(payload).map_err(|error| ApiStateError::Conflict(error.to_string()))
            })
            .collect()
    }

    async fn load_all_conversations_for_user(
        &self,
        user: &PublicUser,
    ) -> Result<Vec<(String, ConversationRecord)>, ApiStateError> {
        if let Some(db) = &self.db {
            let user_id = Uuid::parse_str(&user.id)
                .map_err(|_| ApiStateError::BadRequest("invalid user id".to_string()))?;
            let rows = sqlx::query(
                "SELECT DISTINCT aggregate_id FROM events WHERE aggregate_type = 'conversation' AND user_id = $1",
            )
            .bind(user_id)
            .fetch_all(db)
            .await
            .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

            let mut items = Vec::new();
            for row in rows {
                let aggregate_id = row.get::<Uuid, _>("aggregate_id").to_string();
                if let Some(record) = self.load_conversation_record(&aggregate_id).await? {
                    items.push((aggregate_id, record));
                }
            }
            return Ok(items);
        }

        let conversations = self.conversations.read().await;
        Ok(conversations
            .iter()
            .filter(|(_, record)| record.user_id == user.id)
            .map(|(id, record)| (id.clone(), record.clone()))
            .collect())
    }

    async fn load_conversation_record(
        &self,
        conversation_id: &str,
    ) -> Result<Option<ConversationRecord>, ApiStateError> {
        if let Some(db) = &self.db {
            let aggregate_id = match Uuid::parse_str(conversation_id) {
                Ok(id) => id,
                Err(_) => return Ok(None),
            };

            let rows = sqlx::query(
                "SELECT user_id, data FROM events
                 WHERE aggregate_id = $1 AND aggregate_type = 'conversation'
                 ORDER BY version ASC",
            )
            .bind(aggregate_id)
            .fetch_all(db)
            .await
            .map_err(|error| ApiStateError::Conflict(error.to_string()))?;

            if rows.is_empty() {
                return Ok(None);
            }

            let user_id = rows[0].get::<Option<Uuid>, _>("user_id").map(|id| id.to_string()).unwrap_or_default();
            let mut events = Vec::with_capacity(rows.len());
            for row in rows {
                let value: serde_json::Value = row.get("data");
                let event: ConversationEvent = serde_json::from_value(value)
                    .map_err(|error| ApiStateError::Conflict(error.to_string()))?;
                events.push(event);
            }

            return Ok(Some(ConversationRecord { user_id, events }));
        }

        Ok(self.conversations.read().await.get(conversation_id).cloned())
    }

    fn aggregate_from_events(&self, events: &[ConversationEvent]) -> Conversation {
        let mut aggregate = Conversation::default();
        for event in events {
            aggregate.apply(event.clone());
        }
        aggregate
    }

    fn ensure_owner(&self, user: &PublicUser, record: &ConversationRecord) -> Result<(), ApiStateError> {
        if record.user_id == user.id {
            Ok(())
        } else {
            Err(ApiStateError::Forbidden)
        }
    }

    fn to_public_user(&self, user: &UserRecord) -> PublicUser {
        PublicUser {
            id: user.id.clone(),
            username: user.username.clone(),
            role: user.role.clone(),
        }
    }
}

fn fallback_title(message: &str) -> String {
    let words = message
        .split_whitespace()
        .take(5)
        .collect::<Vec<_>>()
        .join(" ");
    sanitize_title(&words)
}

fn sanitize_title(title: &str) -> String {
    title
        .trim()
        .trim_matches('"')
        .trim_matches('`')
        .chars()
        .take(60)
        .collect::<String>()
}

#[derive(Debug)]
pub enum ApiStateError {
    NotFound,
    Forbidden,
    Conflict(String),
    BadRequest(String),
}

impl From<ConversationError> for ApiStateError {
    fn from(value: ConversationError) -> Self {
        Self::Conflict(value.to_string())
    }
}

fn map_cqrs_error(error: cqrs_es::AggregateError<ConversationError>) -> ApiStateError {
    match error {
        cqrs_es::AggregateError::UserError(inner) => inner.into(),
        cqrs_es::AggregateError::AggregateConflict => {
            ApiStateError::Conflict("aggregate conflict".to_string())
        }
        cqrs_es::AggregateError::DatabaseConnectionError(inner)
        | cqrs_es::AggregateError::DeserializationError(inner)
        | cqrs_es::AggregateError::UnexpectedError(inner) => {
            ApiStateError::Conflict(inner.to_string())
        }
    }
}
