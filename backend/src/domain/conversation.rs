//! Conversation Aggregate
//!
//! Implements the Conversation aggregate using cqrs-es 0.5 for event sourcing.
//! Pure domain logic with no I/O.

use cqrs_es::event_sink::EventSink;
use cqrs_es::{Aggregate, DomainEvent};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// ERROR TYPE
// ============================================================================

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Error)]
pub enum ConversationError {
    #[error("conversation is archived")]
    ConversationArchived,
    #[error("already archived")]
    AlreadyArchived,
    #[error("message content cannot be empty")]
    EmptyMessageContent,
    #[error("{0}")]
    Other(String),
}

impl From<&str> for ConversationError {
    fn from(s: &str) -> Self {
        ConversationError::Other(s.to_string())
    }
}

// ============================================================================
// COMMANDS
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConversationCommand {
    Start(StartConversation),
    SendUserMessage(SendUserMessage),
    CompleteAssistantResponse(CompleteAssistantResponse),
    ChangeTitle(ChangeTitle),
    ArchiveConversation(ArchiveConversation),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartConversation {
    pub conversation_id: String,
    pub user_id: String,
    pub title: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendUserMessage {
    pub content: String,
    pub user_id: String,
    pub model_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteAssistantResponse {
    pub content: String,
    pub model_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeTitle {
    pub title: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchiveConversation {
    pub timestamp: String,
}

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConversationEvent {
    Started(ConversationStarted),
    UserMessageSent(UserMessageSent),
    AssistantMessageSent(AssistantMessageSent),
    TitleChanged(TitleChanged),
    ConversationArchived(ConversationArchived),
    ConversationSummarized(ConversationSummarized),
}

impl DomainEvent for ConversationEvent {
    fn event_type(&self) -> String {
        match self {
            ConversationEvent::Started(_) => "ConversationStarted".to_string(),
            ConversationEvent::UserMessageSent(_) => "UserMessageSent".to_string(),
            ConversationEvent::AssistantMessageSent(_) => "AssistantMessageSent".to_string(),
            ConversationEvent::TitleChanged(_) => "TitleChanged".to_string(),
            ConversationEvent::ConversationArchived(_) => "ConversationArchived".to_string(),
            ConversationEvent::ConversationSummarized(_) => "ConversationSummarized".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationStarted {
    pub conversation_id: String,
    pub user_id: String,
    pub title: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMessageSent {
    pub content: String,
    pub user_id: String,
    pub model_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantMessageSent {
    pub content: String,
    pub model_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TitleChanged {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationArchived {
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationSummarized {
    pub summary: String,
}

// ============================================================================
// AGGREGATE STATE
// ============================================================================

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub started: bool,
    pub archived: bool,
    pub message_count: usize,
    pub last_model_id: Option<String>,
    pub title: Option<String>,
    pub user_id: Option<String>,
}

// ============================================================================
// SERVICES (for dependency injection)
// ============================================================================

pub struct ConversationServices;

// ============================================================================
// AGGREGATE TRAIT IMPLEMENTATION
// ============================================================================

impl Aggregate for Conversation {
    const TYPE: &'static str = "conversation";

    type Command = ConversationCommand;
    type Event = ConversationEvent;
    type Error = ConversationError;
    type Services = ConversationServices;

    async fn handle(
        &mut self,
        command: Self::Command,
        _services: &Self::Services,
        sink: &EventSink<Self>,
    ) -> Result<(), Self::Error> {
        match command {
            ConversationCommand::Start(cmd) => {
                sink.write(
                    ConversationEvent::Started(ConversationStarted {
                        conversation_id: cmd.conversation_id,
                        user_id: cmd.user_id,
                        title: cmd.title,
                        timestamp: cmd.timestamp,
                    }),
                    self,
                )
                .await;
            }
            ConversationCommand::SendUserMessage(cmd) => {
                if self.archived {
                    return Err(ConversationError::ConversationArchived);
                }
                if cmd.content.trim().is_empty() {
                    return Err(ConversationError::EmptyMessageContent);
                }
                sink.write(
                    ConversationEvent::UserMessageSent(UserMessageSent {
                        content: cmd.content,
                        user_id: cmd.user_id,
                        model_id: cmd.model_id,
                        timestamp: cmd.timestamp,
                    }),
                    self,
                )
                .await;
            }
            ConversationCommand::CompleteAssistantResponse(cmd) => {
                sink.write(
                    ConversationEvent::AssistantMessageSent(AssistantMessageSent {
                        content: cmd.content,
                        model_id: cmd.model_id,
                        timestamp: cmd.timestamp,
                    }),
                    self,
                )
                .await;
            }
            ConversationCommand::ChangeTitle(cmd) => {
                sink.write(
                    ConversationEvent::TitleChanged(TitleChanged { title: cmd.title }),
                    self,
                )
                .await;
            }
            ConversationCommand::ArchiveConversation(cmd) => {
                if self.archived {
                    return Err(ConversationError::AlreadyArchived);
                }
                sink.write(
                    ConversationEvent::ConversationArchived(ConversationArchived {
                        timestamp: cmd.timestamp,
                    }),
                    self,
                )
                .await;
            }
        }
        Ok(())
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ConversationEvent::Started(evt) => {
                self.id = evt.conversation_id;
                self.started = true;
                self.user_id = Some(evt.user_id);
                self.title = Some(evt.title);
                self.message_count = 0;
            }
            ConversationEvent::UserMessageSent(evt) => {
                self.message_count += 1;
                self.last_model_id = Some(evt.model_id);
            }
            ConversationEvent::AssistantMessageSent(evt) => {
                self.message_count += 1;
                self.last_model_id = Some(evt.model_id);
            }
            ConversationEvent::TitleChanged(evt) => {
                self.title = Some(evt.title);
            }
            ConversationEvent::ConversationArchived(_) => {
                self.archived = true;
            }
            ConversationEvent::ConversationSummarized(_) => {
                // No state change needed
            }
        }
    }
}
