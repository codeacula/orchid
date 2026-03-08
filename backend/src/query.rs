use crate::domain::{ConversationHistoryView, ConversationListEntry, ConversationMemoryView};
use crate::domain::Conversation;
use cqrs_es::{EventEnvelope, Query, View};
use sqlx::{PgPool, Row};

pub fn conversation_queries(pool: PgPool) -> Vec<Box<dyn Query<Conversation>>> {
    vec![
        Box::new(HistoryProjection::new(pool.clone())),
        Box::new(ListProjection::new(pool.clone())),
        Box::new(MemoryProjection::new(pool)),
    ]
}

struct HistoryProjection {
    pool: PgPool,
}

struct ListProjection {
    pool: PgPool,
}

struct MemoryProjection {
    pool: PgPool,
}

impl HistoryProjection {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ListProjection {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MemoryProjection {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Query<Conversation> for HistoryProjection {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Conversation>]) {
        let Some(view) = load_view::<ConversationHistoryView>(&self.pool, "conversation_history_views", aggregate_id).await else {
            let mut view = ConversationHistoryView::default();
            for event in events {
                view.update(event);
            }
            let _ = save_history(&self.pool, aggregate_id, &view).await;
            return;
        };

        let mut next = view;
        for event in events {
            next.update(event);
        }
        let _ = save_history(&self.pool, aggregate_id, &next).await;
    }
}

#[async_trait::async_trait]
impl Query<Conversation> for ListProjection {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Conversation>]) {
        let current = load_view::<ConversationListEntry>(&self.pool, "conversation_list_entries", aggregate_id)
            .await
            .unwrap_or_default();
        let mut next = current;
        for event in events {
            next.update(event);
        }
        let _ = save_list(&self.pool, aggregate_id, &next).await;
    }
}

#[async_trait::async_trait]
impl Query<Conversation> for MemoryProjection {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Conversation>]) {
        let current = load_view::<ConversationMemoryView>(&self.pool, "conversation_memory_views", aggregate_id)
            .await
            .unwrap_or_default();
        let mut next = current;
        for event in events {
            next.update(event);
        }
        let _ = save_memory(&self.pool, aggregate_id, &next).await;
    }
}

async fn load_view<V>(pool: &PgPool, table: &str, conversation_id: &str) -> Option<V>
where
    V: serde::de::DeserializeOwned,
{
    let query = format!("SELECT payload FROM {table} WHERE conversation_id = $1");
    let row = sqlx::query(&query)
        .bind(uuid::Uuid::parse_str(conversation_id).ok()?)
        .fetch_optional(pool)
        .await
        .ok()??;
    let payload: serde_json::Value = row.get("payload");
    serde_json::from_value(payload).ok()
}

async fn save_history(pool: &PgPool, conversation_id: &str, view: &ConversationHistoryView) -> anyhow::Result<()> {
    let query = "INSERT INTO conversation_history_views (conversation_id, user_id, title, archived, updated_at, payload)
                 VALUES ($1, $2, $3, $4, $5::timestamptz, $6)
                 ON CONFLICT (conversation_id)
                 DO UPDATE SET user_id = EXCLUDED.user_id, title = EXCLUDED.title, archived = EXCLUDED.archived, updated_at = EXCLUDED.updated_at, payload = EXCLUDED.payload";
    sqlx::query(query)
        .bind(uuid::Uuid::parse_str(conversation_id)?)
        .bind(parse_user_id(view.user_id.as_deref())?)
        .bind(&view.title)
        .bind(view.archived)
        .bind(&view.updated_at)
        .bind(serde_json::to_value(view)?)
        .execute(pool)
        .await?;
    Ok(())
}

async fn save_list(pool: &PgPool, conversation_id: &str, view: &ConversationListEntry) -> anyhow::Result<()> {
    let query = "INSERT INTO conversation_list_entries (conversation_id, user_id, title, last_message_preview, archived, updated_at, payload)
                 VALUES ($1, $2, $3, $4, $5, $6::timestamptz, $7)
                 ON CONFLICT (conversation_id)
                 DO UPDATE SET user_id = EXCLUDED.user_id, title = EXCLUDED.title, last_message_preview = EXCLUDED.last_message_preview, archived = EXCLUDED.archived, updated_at = EXCLUDED.updated_at, payload = EXCLUDED.payload";
    sqlx::query(query)
        .bind(uuid::Uuid::parse_str(conversation_id)?)
        .bind(parse_user_id(view.user_id.as_deref())?)
        .bind(&view.title)
        .bind(&view.last_message_preview)
        .bind(view.archived)
        .bind(&view.updated_at)
        .bind(serde_json::to_value(view)?)
        .execute(pool)
        .await?;
    Ok(())
}

async fn save_memory(pool: &PgPool, conversation_id: &str, view: &ConversationMemoryView) -> anyhow::Result<()> {
    let query = "INSERT INTO conversation_memory_views (conversation_id, user_id, archived, summary, updated_at, payload)
                 VALUES ($1, $2, $3, $4, NOW(), $5)
                 ON CONFLICT (conversation_id)
                 DO UPDATE SET user_id = EXCLUDED.user_id, archived = EXCLUDED.archived, summary = EXCLUDED.summary, updated_at = EXCLUDED.updated_at, payload = EXCLUDED.payload";
    sqlx::query(query)
        .bind(uuid::Uuid::parse_str(conversation_id)?)
        .bind(parse_user_id(view.user_id.as_deref())?)
        .bind(view.archived)
        .bind(view.summary.clone())
        .bind(serde_json::to_value(view)?)
        .execute(pool)
        .await?;
    Ok(())
}

fn parse_user_id(user_id: Option<&str>) -> anyhow::Result<uuid::Uuid> {
    let user_id = user_id.ok_or_else(|| anyhow::anyhow!("missing view user id"))?;
    Ok(uuid::Uuid::parse_str(user_id)?)
}
