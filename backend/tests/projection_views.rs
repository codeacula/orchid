use orchid::domain::{
    AssistantMessageSent, ConversationArchived, ConversationEvent, ConversationHistoryView,
    ConversationListEntry, ConversationMemoryView, ConversationStarted, TitleChanged,
    UserMessageSent,
};

fn sample_events() -> Vec<ConversationEvent> {
    vec![
        ConversationEvent::Started(ConversationStarted {
            conversation_id: "conv-1".to_string(),
            user_id: "user-1".to_string(),
            title: "Original Title".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }),
        ConversationEvent::UserMessageSent(UserMessageSent {
            content: "Hello from user".to_string(),
            user_id: "user-1".to_string(),
            model_id: "gpt-4o".to_string(),
            timestamp: "2026-01-01T00:01:00Z".to_string(),
        }),
        ConversationEvent::AssistantMessageSent(AssistantMessageSent {
            content: "Hello from assistant".to_string(),
            model_id: "gpt-4o".to_string(),
            timestamp: "2026-01-01T00:02:00Z".to_string(),
        }),
        ConversationEvent::TitleChanged(TitleChanged {
            title: "Updated Title".to_string(),
        }),
    ]
}

#[test]
fn history_view_tracks_messages_and_models() {
    let history = ConversationHistoryView::from_events("conv-1", &sample_events());
    assert_eq!(history.title, "Updated Title");
    assert_eq!(history.messages.len(), 2);
    assert_eq!(history.messages[0].role, "user");
    assert_eq!(history.messages[0].model_id.as_deref(), Some("gpt-4o"));
    assert_eq!(history.messages[1].role, "assistant");
}

#[test]
fn list_entry_uses_last_message_preview_and_archive_flag() {
    let mut events = sample_events();
    events.push(ConversationEvent::ConversationArchived(
        ConversationArchived {
            timestamp: "2026-01-01T00:03:00Z".to_string(),
        },
    ));

    let entry = ConversationListEntry::from_events("conv-1", &events);
    assert_eq!(entry.title, "Updated Title");
    assert!(entry.last_message_preview.contains("assistant"));
    assert!(entry.archived);
}

#[test]
fn memory_view_keeps_recent_window() {
    let mut events = vec![ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Windowed Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    })];

    for index in 0..6 {
        events.push(ConversationEvent::UserMessageSent(UserMessageSent {
            content: format!("message {index}"),
            user_id: "user-1".to_string(),
            model_id: "gpt-4o".to_string(),
            timestamp: format!("2026-01-01T00:0{index}:00Z"),
        }));
    }

    let memory = ConversationMemoryView::from_events("conv-1", &events, 4);
    assert_eq!(memory.recent_messages.len(), 4);
    assert_eq!(memory.recent_messages[0].content, "message 2");
    assert!(memory
        .summary
        .as_deref()
        .unwrap_or_default()
        .contains("message 5"));
}
