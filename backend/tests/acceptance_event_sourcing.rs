//! Acceptance Tests: Event Sourcing (Conversation Aggregate)
//!
//! These tests verify the core event sourcing behavior of the Conversation aggregate.
//! They use the cqrs-es TestFramework (no database required) to validate business rules.
//!
//! LOCKED — Do not modify, delete, rename, or skip any test in this file.

// These tests will compile and run once the domain types are implemented.
// Until then, each test body contains `todo!()` to fail immediately.

#[test]
fn starting_a_conversation_emits_conversation_started_event() {
    // Verifies: When a StartConversation command is issued with no prior events,
    // a ConversationStarted event is emitted containing the conversation ID,
    // the user ID who started it, and a timestamp.
    todo!("not yet implemented");
}

#[test]
fn sending_a_user_message_emits_user_message_sent_event() {
    // Verifies: Given a started conversation, when a SendUserMessage command is issued,
    // a UserMessageSent event is emitted with the message content, user ID,
    // and the selected model identifier.
    todo!("not yet implemented");
}

#[test]
fn completing_assistant_response_emits_assistant_message_sent_event() {
    // Verifies: Given a conversation with a user message, when a CompleteAssistantResponse
    // command is issued (after streaming finishes), an AssistantMessageSent event is emitted
    // with the full assembled content and the model that generated it.
    todo!("not yet implemented");
}

#[test]
fn changing_conversation_title_emits_title_changed_event() {
    // Verifies: Given a started conversation, when a ChangeTitle command is issued,
    // a TitleChanged event is emitted with the new title.
    todo!("not yet implemented");
}

#[test]
fn archiving_a_conversation_emits_conversation_archived_event() {
    // Verifies: Given a started conversation, when an ArchiveConversation command is issued,
    // a ConversationArchived event is emitted.
    todo!("not yet implemented");
}

#[test]
fn cannot_send_message_to_archived_conversation() {
    // Verifies: Given a conversation that has been archived, when a SendUserMessage command
    // is issued, an error is returned indicating the conversation is archived.
    todo!("not yet implemented");
}

#[test]
fn cannot_archive_an_already_archived_conversation() {
    // Verifies: Given an already-archived conversation, when ArchiveConversation is issued again,
    // an error is returned.
    todo!("not yet implemented");
}

#[test]
fn empty_message_content_is_rejected() {
    // Verifies: Given a started conversation, when a SendUserMessage command is issued
    // with empty or whitespace-only content, an error is returned.
    todo!("not yet implemented");
}

#[test]
fn conversation_state_is_correct_after_many_events() {
    // Verifies: Given a conversation with 50+ messages (alternating user/assistant),
    // the aggregate state correctly reflects the full message count and last message.
    // This validates event replay correctness for long conversations.
    todo!("not yet implemented");
}

#[test]
fn model_selection_is_recorded_per_message() {
    // Verifies: Given a started conversation, when two messages are sent with different
    // model identifiers (e.g., "ollama/llama3" then "openai/gpt-4"), each resulting
    // UserMessageSent event records the correct model used for that message.
    todo!("not yet implemented");
}

#[test]
fn apply_replays_events_to_reconstruct_aggregate_state() {
    // Verifies: The aggregate's apply() method correctly updates internal state
    // for each event type (ConversationStarted, UserMessageSent, AssistantMessageSent,
    // TitleChanged, ConversationArchived). After replaying a known sequence of events,
    // the aggregate state matches expected values.
    todo!("not yet implemented");
}
