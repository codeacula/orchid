//! Acceptance Tests: Event Sourcing Projections (Views)
//!
//! These tests verify that event projections correctly transform events
//! into queryable views. They test the View::update() logic directly.
//!
//! LOCKED — Do not modify, delete, rename, or skip any test in this file.

#[test]
fn conversation_history_view_builds_full_message_list() {
    // Verifies: After applying ConversationStarted, UserMessageSent, and
    // AssistantMessageSent events, the ConversationHistoryView contains
    // all messages in order with correct roles, content, and timestamps.
    todo!("not yet implemented");
}

#[test]
fn conversation_list_entry_shows_title_and_last_message_preview() {
    // Verifies: After applying events, the ConversationListEntry view contains
    // the conversation title, a preview of the last message (truncated),
    // and the updated_at timestamp.
    todo!("not yet implemented");
}

#[test]
fn conversation_list_entry_updates_on_title_change() {
    // Verifies: After a TitleChanged event, the ConversationListEntry view
    // reflects the new title.
    todo!("not yet implemented");
}

#[test]
fn conversation_memory_view_maintains_sliding_window() {
    // Verifies: The ConversationMemoryView only retains the last N messages
    // (configured window size) for use as AI context, rather than the
    // full history.
    todo!("not yet implemented");
}

#[test]
fn archived_conversation_is_excluded_from_active_list() {
    // Verifies: After a ConversationArchived event, the conversation no longer
    // appears in the active conversation list projection.
    todo!("not yet implemented");
}

#[test]
fn conversation_history_view_records_model_per_message() {
    // Verifies: Each message in the ConversationHistoryView includes the
    // model_id that was used, reflecting the per-message model selection.
    todo!("not yet implemented");
}
