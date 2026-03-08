use crate::domain::conversation::{ConversationArchived, ConversationEvent};
use cqrs_es::{EventEnvelope, View};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageView {
    pub role: String,
    pub content: String,
    pub timestamp: String,
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversationHistoryView {
    pub id: String,
    pub title: String,
    pub user_id: Option<String>,
    pub archived: bool,
    pub updated_at: String,
    pub messages: Vec<MessageView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversationListEntry {
    pub id: String,
    pub user_id: Option<String>,
    pub title: String,
    pub last_message_preview: String,
    pub updated_at: String,
    pub archived: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversationMemoryView {
    pub id: String,
    pub user_id: Option<String>,
    pub archived: bool,
    pub recent_messages: Vec<MessageView>,
    pub summary: Option<String>,
}

impl ConversationHistoryView {
    pub fn from_events(id: &str, events: &[ConversationEvent]) -> Self {
        let mut view = Self {
            id: id.to_string(),
            ..Self::default()
        };

        for event in events {
            match event {
                ConversationEvent::Started(evt) => {
                    view.title = evt.title.clone();
                    view.user_id = Some(evt.user_id.clone());
                    view.updated_at = evt.timestamp.clone();
                }
                ConversationEvent::UserMessageSent(evt) => {
                    view.messages.push(MessageView {
                        role: "user".to_string(),
                        content: evt.content.clone(),
                        timestamp: evt.timestamp.clone(),
                        model_id: Some(evt.model_id.clone()),
                    });
                    view.updated_at = evt.timestamp.clone();
                }
                ConversationEvent::AssistantMessageSent(evt) => {
                    view.messages.push(MessageView {
                        role: "assistant".to_string(),
                        content: evt.content.clone(),
                        timestamp: evt.timestamp.clone(),
                        model_id: Some(evt.model_id.clone()),
                    });
                    view.updated_at = evt.timestamp.clone();
                }
                ConversationEvent::TitleChanged(evt) => {
                    view.title = evt.title.clone();
                }
                ConversationEvent::ConversationArchived(ConversationArchived { timestamp }) => {
                    view.archived = true;
                    view.updated_at = timestamp.clone();
                }
                ConversationEvent::ConversationSummarized(_) => {}
            }
        }

        view
    }
}

impl ConversationListEntry {
    pub fn from_events(id: &str, events: &[ConversationEvent]) -> Self {
        let history = ConversationHistoryView::from_events(id, events);
        let last_message_preview = history
            .messages
            .last()
            .map(|message| truncate(&message.content, 80))
            .unwrap_or_default();

        Self {
            id: history.id,
            user_id: history.user_id,
            title: history.title,
            last_message_preview,
            updated_at: history.updated_at,
            archived: history.archived,
        }
    }
}

impl ConversationMemoryView {
    pub fn from_events(id: &str, events: &[ConversationEvent], window: usize) -> Self {
        let history = ConversationHistoryView::from_events(id, events);
        let len = history.messages.len();
        let start = len.saturating_sub(window);
        let recent_messages = history.messages[start..].to_vec();
        let summary = recent_messages.last().map(|message| {
            format!(
                "Conversation '{}' most recently discussed: {}",
                history.title,
                truncate(&message.content, 120)
            )
        });

        Self {
            id: history.id,
            user_id: history.user_id,
            archived: history.archived,
            recent_messages,
            summary,
        }
    }
}

impl View<crate::domain::conversation::Conversation> for ConversationHistoryView {
    fn update(&mut self, event: &EventEnvelope<crate::domain::conversation::Conversation>) {
        match &event.payload {
            ConversationEvent::Started(evt) => {
                self.id = event.aggregate_id.clone();
                self.user_id = Some(evt.user_id.clone());
                self.title = evt.title.clone();
                self.updated_at = evt.timestamp.clone();
            }
            ConversationEvent::UserMessageSent(evt) => {
                self.messages.push(MessageView {
                    role: "user".to_string(),
                    content: evt.content.clone(),
                    timestamp: evt.timestamp.clone(),
                    model_id: Some(evt.model_id.clone()),
                });
                self.updated_at = evt.timestamp.clone();
            }
            ConversationEvent::AssistantMessageSent(evt) => {
                self.messages.push(MessageView {
                    role: "assistant".to_string(),
                    content: evt.content.clone(),
                    timestamp: evt.timestamp.clone(),
                    model_id: Some(evt.model_id.clone()),
                });
                self.updated_at = evt.timestamp.clone();
            }
            ConversationEvent::TitleChanged(evt) => {
                self.title = evt.title.clone();
            }
            ConversationEvent::ConversationArchived(ConversationArchived { timestamp }) => {
                self.archived = true;
                self.updated_at = timestamp.clone();
            }
            ConversationEvent::ConversationSummarized(_) => {}
        }
    }
}

impl View<crate::domain::conversation::Conversation> for ConversationListEntry {
    fn update(&mut self, event: &EventEnvelope<crate::domain::conversation::Conversation>) {
        match &event.payload {
            ConversationEvent::Started(evt) => {
                self.id = event.aggregate_id.clone();
                self.user_id = Some(evt.user_id.clone());
                self.title = evt.title.clone();
                self.updated_at = evt.timestamp.clone();
            }
            ConversationEvent::UserMessageSent(evt) => {
                self.last_message_preview = truncate(&evt.content, 80);
                self.updated_at = evt.timestamp.clone();
            }
            ConversationEvent::AssistantMessageSent(evt) => {
                self.last_message_preview = truncate(&evt.content, 80);
                self.updated_at = evt.timestamp.clone();
            }
            ConversationEvent::TitleChanged(evt) => {
                self.title = evt.title.clone();
            }
            ConversationEvent::ConversationArchived(_) => {
                self.archived = true;
            }
            ConversationEvent::ConversationSummarized(_) => {}
        }
    }
}

impl View<crate::domain::conversation::Conversation> for ConversationMemoryView {
    fn update(&mut self, event: &EventEnvelope<crate::domain::conversation::Conversation>) {
        match &event.payload {
            ConversationEvent::Started(evt) => {
                self.id = event.aggregate_id.clone();
                self.user_id = Some(evt.user_id.clone());
            }
            ConversationEvent::UserMessageSent(evt) => {
                self.recent_messages.push(MessageView {
                    role: "user".to_string(),
                    content: evt.content.clone(),
                    timestamp: evt.timestamp.clone(),
                    model_id: Some(evt.model_id.clone()),
                });
            }
            ConversationEvent::AssistantMessageSent(evt) => {
                self.recent_messages.push(MessageView {
                    role: "assistant".to_string(),
                    content: evt.content.clone(),
                    timestamp: evt.timestamp.clone(),
                    model_id: Some(evt.model_id.clone()),
                });
            }
            ConversationEvent::TitleChanged(_) => {}
            ConversationEvent::ConversationArchived(_) => {
                self.archived = true;
            }
            ConversationEvent::ConversationSummarized(summary) => {
                self.summary = Some(summary.summary.clone());
            }
        }

        if self.recent_messages.len() > 4 {
            let keep_from = self.recent_messages.len() - 4;
            self.recent_messages = self.recent_messages.split_off(keep_from);
        }

        if self.summary.is_none() {
            self.summary = self.recent_messages.last().map(|message| {
                format!(
                    "Most recently discussed: {}",
                    truncate(&message.content, 120)
                )
            });
        }
    }
}

fn truncate(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        value.to_string()
    } else {
        value.chars().take(max_len).collect::<String>() + "..."
    }
}
