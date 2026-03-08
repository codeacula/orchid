//! Acceptance Tests: Event Sourcing (Conversation Aggregate)
//!
//! These tests verify the core event sourcing behavior of the Conversation aggregate.
//! They use the cqrs-es TestFramework (no database required) to validate business rules.
//!
//! LOCKED — Do not modify, delete, rename, or skip any test in this file.

// These tests will compile and run once the domain types are implemented.
// Until then, each test body contains `todo!()` to fail immediately.

use cqrs_es::test::TestFramework;
use orchid::domain::conversation::*;

type ConversationTestFramework = TestFramework<Conversation>;

#[test]
fn starting_a_conversation_emits_conversation_started_event() {
    // Verifies: When a StartConversation command is issued with no prior events,
    // a ConversationStarted event is emitted containing the conversation ID,
    // the user ID who started it, and a timestamp.
    let framework = ConversationTestFramework::with(ConversationServices);
    let cmd = ConversationCommand::Start(StartConversation {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    framework
        .given_no_previous_events()
        .when(cmd)
        .then_expect_events(vec![ConversationEvent::Started(ConversationStarted {
            conversation_id: "conv-1".to_string(),
            user_id: "user-1".to_string(),
            title: "Test Chat".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        })]);
}

#[test]
fn sending_a_user_message_emits_user_message_sent_event() {
    // Verifies: Given a started conversation, when a SendUserMessage command is issued,
    // a UserMessageSent event is emitted with the message content, user ID,
    // and the selected model identifier.
    let framework = ConversationTestFramework::with(ConversationServices);
    let started_event = ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let cmd = ConversationCommand::SendUserMessage(SendUserMessage {
        content: "Hello, world!".to_string(),
        user_id: "user-1".to_string(),
        model_id: "ollama/llama3".to_string(),
        timestamp: "2026-01-01T00:01:00Z".to_string(),
    });

    framework
        .given(vec![started_event])
        .when(cmd)
        .then_expect_events(vec![ConversationEvent::UserMessageSent(UserMessageSent {
            content: "Hello, world!".to_string(),
            user_id: "user-1".to_string(),
            model_id: "ollama/llama3".to_string(),
            timestamp: "2026-01-01T00:01:00Z".to_string(),
        })]);
}

#[test]
fn completing_assistant_response_emits_assistant_message_sent_event() {
    // Verifies: Given a conversation with a user message, when a CompleteAssistantResponse
    // command is issued (after streaming finishes), an AssistantMessageSent event is emitted
    // with the full assembled content and the model that generated it.
    let framework = ConversationTestFramework::with(ConversationServices);
    let started_event = ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let user_message_event = ConversationEvent::UserMessageSent(UserMessageSent {
        content: "Hello, world!".to_string(),
        user_id: "user-1".to_string(),
        model_id: "ollama/llama3".to_string(),
        timestamp: "2026-01-01T00:01:00Z".to_string(),
    });

    let cmd = ConversationCommand::CompleteAssistantResponse(CompleteAssistantResponse {
        content: "This is the assistant response.".to_string(),
        model_id: "ollama/llama3".to_string(),
        timestamp: "2026-01-01T00:02:00Z".to_string(),
    });

    framework
        .given(vec![started_event, user_message_event])
        .when(cmd)
        .then_expect_events(vec![ConversationEvent::AssistantMessageSent(
            AssistantMessageSent {
                content: "This is the assistant response.".to_string(),
                model_id: "ollama/llama3".to_string(),
                timestamp: "2026-01-01T00:02:00Z".to_string(),
            },
        )]);
}

#[test]
fn changing_conversation_title_emits_title_changed_event() {
    // Verifies: Given a started conversation, when a ChangeTitle command is issued,
    // a TitleChanged event is emitted with the new title.
    let framework = ConversationTestFramework::with(ConversationServices);
    let started_event = ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Old Title".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let cmd = ConversationCommand::ChangeTitle(ChangeTitle {
        title: "New Title".to_string(),
        timestamp: "2026-01-01T00:01:00Z".to_string(),
    });

    framework
        .given(vec![started_event])
        .when(cmd)
        .then_expect_events(vec![ConversationEvent::TitleChanged(TitleChanged {
            title: "New Title".to_string(),
        })]);
}

#[test]
fn archiving_a_conversation_emits_conversation_archived_event() {
    // Verifies: Given a started conversation, when an ArchiveConversation command is issued,
    // a ConversationArchived event is emitted.
    let framework = ConversationTestFramework::with(ConversationServices);
    let started_event = ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let cmd = ConversationCommand::ArchiveConversation(ArchiveConversation {
        timestamp: "2026-01-01T00:01:00Z".to_string(),
    });

    framework
        .given(vec![started_event])
        .when(cmd)
        .then_expect_events(vec![ConversationEvent::ConversationArchived(
            ConversationArchived {
                timestamp: "2026-01-01T00:01:00Z".to_string(),
            },
        )]);
}

#[test]
fn cannot_send_message_to_archived_conversation() {
    // Verifies: Given a conversation that has been archived, when a SendUserMessage command
    // is issued, an error is returned indicating the conversation is archived.
    let framework = ConversationTestFramework::with(ConversationServices);
    let started_event = ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let archived_event = ConversationEvent::ConversationArchived(ConversationArchived {
        timestamp: "2026-01-01T00:01:00Z".to_string(),
    });

    let cmd = ConversationCommand::SendUserMessage(SendUserMessage {
        content: "Hello".to_string(),
        user_id: "user-1".to_string(),
        model_id: "ollama/llama3".to_string(),
        timestamp: "2026-01-01T00:02:00Z".to_string(),
    });

    framework
        .given(vec![started_event, archived_event])
        .when(cmd)
        .then_expect_error_message("conversation is archived");
}

#[test]
fn cannot_archive_an_already_archived_conversation() {
    // Verifies: Given an already-archived conversation, when ArchiveConversation is issued again,
    // an error is returned.
    let framework = ConversationTestFramework::with(ConversationServices);
    let started_event = ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let archived_event = ConversationEvent::ConversationArchived(ConversationArchived {
        timestamp: "2026-01-01T00:01:00Z".to_string(),
    });

    let cmd = ConversationCommand::ArchiveConversation(ArchiveConversation {
        timestamp: "2026-01-01T00:02:00Z".to_string(),
    });

    framework
        .given(vec![started_event, archived_event])
        .when(cmd)
        .then_expect_error_message("already archived");
}

#[test]
fn empty_message_content_is_rejected() {
    // Verifies: Given a started conversation, when a SendUserMessage command is issued
    // with empty or whitespace-only content, an error is returned.
    let framework = ConversationTestFramework::with(ConversationServices);
    let started_event = ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let cmd = ConversationCommand::SendUserMessage(SendUserMessage {
        content: "   ".to_string(), // whitespace-only
        user_id: "user-1".to_string(),
        model_id: "ollama/llama3".to_string(),
        timestamp: "2026-01-01T00:01:00Z".to_string(),
    });

    framework
        .given(vec![started_event])
        .when(cmd)
        .then_expect_error_message("message content cannot be empty");
}

#[test]
fn conversation_state_is_correct_after_many_events() {
    // Verifies: Given a conversation with 50+ messages (alternating user/assistant),
    // the aggregate state correctly reflects the full message count and last message.
    // This validates event replay correctness for long conversations.
    let framework = ConversationTestFramework::with(ConversationServices);
    let mut events = vec![ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    })];

    // Add 50 alternating user/assistant messages (25 pairs = 50 total)
    for i in 0..25 {
        events.push(ConversationEvent::UserMessageSent(UserMessageSent {
            content: format!("User message {}", i + 1),
            user_id: "user-1".to_string(),
            model_id: "ollama/llama3".to_string(),
            timestamp: format!("2026-01-01T00:{:02}:00Z", i * 2),
        }));

        events.push(ConversationEvent::AssistantMessageSent(
            AssistantMessageSent {
                content: format!("Assistant response {}", i + 1),
                model_id: "ollama/llama3".to_string(),
                timestamp: format!("2026-01-01T00:{:02}:30Z", i * 2),
            },
        ));
    }

    let cmd = ConversationCommand::SendUserMessage(SendUserMessage {
        content: "Final user message".to_string(),
        user_id: "user-1".to_string(),
        model_id: "openai/gpt-4".to_string(),
        timestamp: "2026-01-01T01:00:00Z".to_string(),
    });

    framework
        .given(events)
        .when(cmd)
        .then_expect_events(vec![ConversationEvent::UserMessageSent(UserMessageSent {
            content: "Final user message".to_string(),
            user_id: "user-1".to_string(),
            model_id: "openai/gpt-4".to_string(),
            timestamp: "2026-01-01T01:00:00Z".to_string(),
        })]);
}

#[test]
fn model_selection_is_recorded_per_message() {
    // Verifies: Given a started conversation, when two messages are sent with different
    // model identifiers (e.g., "ollama/llama3" then "openai/gpt-4"), each resulting
    // UserMessageSent event records the correct model used for that message.
    let framework = ConversationTestFramework::with(ConversationServices);
    let started_event = ConversationEvent::Started(ConversationStarted {
        conversation_id: "conv-1".to_string(),
        user_id: "user-1".to_string(),
        title: "Test Chat".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let first_message = ConversationEvent::UserMessageSent(UserMessageSent {
        content: "First message".to_string(),
        user_id: "user-1".to_string(),
        model_id: "ollama/llama3".to_string(),
        timestamp: "2026-01-01T00:01:00Z".to_string(),
    });

    let cmd = ConversationCommand::SendUserMessage(SendUserMessage {
        content: "Second message".to_string(),
        user_id: "user-1".to_string(),
        model_id: "openai/gpt-4".to_string(),
        timestamp: "2026-01-01T00:02:00Z".to_string(),
    });

    framework
        .given(vec![started_event, first_message])
        .when(cmd)
        .then_expect_events(vec![ConversationEvent::UserMessageSent(UserMessageSent {
            content: "Second message".to_string(),
            user_id: "user-1".to_string(),
            model_id: "openai/gpt-4".to_string(),
            timestamp: "2026-01-01T00:02:00Z".to_string(),
        })]);
}

#[test]
fn apply_replays_events_to_reconstruct_aggregate_state() {
    // Verifies: The aggregate's apply() method correctly updates internal state
    // for each event type (ConversationStarted, UserMessageSent, AssistantMessageSent,
    // TitleChanged, ConversationArchived). After replaying a known sequence of events,
    // the aggregate state matches expected values.
    let framework = ConversationTestFramework::with(ConversationServices);

    let events = vec![
        ConversationEvent::Started(ConversationStarted {
            conversation_id: "conv-1".to_string(),
            user_id: "user-1".to_string(),
            title: "Original Title".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }),
        ConversationEvent::UserMessageSent(UserMessageSent {
            content: "Hello".to_string(),
            user_id: "user-1".to_string(),
            model_id: "ollama/llama3".to_string(),
            timestamp: "2026-01-01T00:01:00Z".to_string(),
        }),
        ConversationEvent::AssistantMessageSent(AssistantMessageSent {
            content: "Hi there!".to_string(),
            model_id: "ollama/llama3".to_string(),
            timestamp: "2026-01-01T00:02:00Z".to_string(),
        }),
        ConversationEvent::TitleChanged(TitleChanged {
            title: "Updated Title".to_string(),
        }),
    ];

    let cmd = ConversationCommand::ArchiveConversation(ArchiveConversation {
        timestamp: "2026-01-01T00:03:00Z".to_string(),
    });

    framework.given(events).when(cmd).then_expect_events(vec![
        ConversationEvent::ConversationArchived(ConversationArchived {
            timestamp: "2026-01-01T00:03:00Z".to_string(),
        }),
    ]);
}
