# Conjure MVP

**Task ID:** 20260307095408

**Status:** pending

## Request

"I want to bootstrap a ChatGPT-like chatbot app. In the future it will be able to call coding agents for me, but for now I want to be able to have multiple conversations, be able to have it reference previous conversation memory, and to have a lovely VueJS client that has pretty animations and stuff. I want to use Caddy as a reverse proxy, and I want to be able to set it up so I can run it on docker or whatever is best. The backend to be written in Rust."

## Research

### Backend (Rust) Research

- **Framework**: Axum v0.8.8 (by Tokio team)
  - Multi-route structure with `Router`, `State` for shared DB pool
  - WebSocket support via `features = ["ws"]`
  
- **OpenAI Integration**: `async-openai` v0.33.0
  - Streaming chat completions via `create_stream`
  - Stateless API (must resend full message history each call)
  
- **Database**: SQLx v0.8.6 with PostgreSQL
  - Async, compile-time checked queries
  - SQL migrations stored in `backend/migrations/`
  
- **Core Dependencies** (with versions):
  - axum 0.8.8, tokio 1.50.0, tower-http 0.6.8
  - async-openai 0.33.0, sqlx 0.8.6
  - serde 1.0.228, serde_json 1.0.149
  - uuid 1.22.0, chrono 0.4.44
  - anyhow 1.0.102, thiserror 2.0.18
  - tracing 0.1.44, futures-util 0.3.32
  - dotenvy 0.15.7
  
- **Dev Dependencies**:
  - axum-test 19.1.1, mockall 0.14.0, wiremock 0.6.5
  - tower 0.5, http-body-util 0.1
  
- **Memory Strategy**:
  - Maintain a sliding window for in-conversation context
  - Maintain per-conversation summaries for longer-term recall
  - Use summary-based cross-conversation memory for the same authenticated user in MVP
  
- **Database Schema**:
  - Event store tables for conversations and snapshots
  - Projection/view tables for conversation history, sidebar entries, and memory summaries
  - Auth/session tables for users and browser sessions
  
- **Testing Strategy**:
  - Unit tests in `#[cfg(test)]` modules inline
  - Integration tests in `tests/` directory
  - Use `#[tokio::test]` for async tests
  - Test Axum handlers via `.oneshot()` on the Router (tower::ServiceExt)
  - Use isolated Postgres/Redis-backed test setups for infrastructure-sensitive integration tests
  
- **Streaming Implementation**:
  - WebSockets are the MVP transport for authenticated bidirectional chat streaming
  - Do not implement SSE in MVP
  
- **CORS Configuration**:
  - Use `tower-http` CorsLayer
  - Must include `CONTENT_TYPE` header for JSON POST bodies

### Frontend (Vue.js) Research

- **Stack**: Vue 3 + Vite 7.3.1 + TypeScript + Pinia 3.x
  
- **Scaffolding**: `npm create vue@latest`
  - Wraps Vite with prompts for TS, Router, Pinia, Vitest, ESLint
  
- **Project Structure**:
  - src/components/ — reusable Vue components
  - src/composables/ — reusable logic hooks
  - src/stores/ — Pinia state management
  - src/views/ — page-level components
  
- **Chat UI Implementation**:
  - TransitionGroup for message list animations
  - WebSocket-based chat composable for live token streaming
  - Auto-scroll composable for message list
  
- **Markdown Rendering**: `markdown-it` with Shiki v3.23.0
  - Syntax highlighting via VS Code engine
  
- **Animation Framework**:
  - Vue Transition/TransitionGroup (6 CSS classes)
  - `@vueuse/motion` for declarative animations
  - CSS cubic-bezier for message entry effects
  
- **State Management**: Pinia setup store pattern
  - conversations array
  - activeConversationId
  - computed activeConversation
  - actions for CRUD operations
  
- **Testing**: Vitest 4.0.17 + Vue Test Utils 2.x + @pinia/testing
  - Co-located `.spec.ts` file pattern
  - jsdom or happy-dom environment
  - `createTestingPinia()` for store mocking
  
- **Docker Deployment**:
  - Multi-stage Node→Nginx or Caddy-served static files
  - Build stage outputs to /dist
  
- **Security**:
  - Must use DOMPurify with v-html for AI-generated markdown

### Infrastructure (Caddy + Docker) Research

- **Caddy Version**: v2.11.2 (latest, released 2026-03-06)
  
- **Caddyfile Configuration Pattern**:
  - `handle /api/*` → reverse_proxy to backend
  - `handle` → try_files + file_server for SPA routing
  
- **Streaming Support**:
  - Caddy auto-upgrades WebSocket connections (no extra config needed)
  - SSE-specific proxy tuning is not needed for MVP
  
- **HTTPS Configuration**:
  - `auto_https off` for development
  - Automatic Let's Encrypt for production
  
- **Docker Compose Strategy**: Multi-file approach
  - Base docker-compose.yml
  - dev override (optional, if needed later)
  - prod override (optimized, TLS certs volume)
  - Named volumes for PostgreSQL persistence, Redis data, and Caddy TLS certs
  
- **Rust Docker Build Optimization**:
  - cargo-chef 3-stage build (planner → builder → runtime)
  - debian:bookworm-slim runtime (~30MB base)
  - Up to 5x speedup from layer caching
  
- **Development Workflow Options**:
  - **Option A (Full Docker)**: cargo-watch + systemfd for hot reload, Vite dev server with bind mounts
  - **Option B (Native Dev)**: Vite proxy to local Rust backend, faster iteration
  
- **Health Checks & Startup Ordering**:
  - `depends_on` with `condition: service_healthy` for proper service startup ordering
  - Health check endpoints for database and backend readiness

### Event Sourcing Architecture

- **Crates**: `cqrs-es` v0.5.0 + `postgres-es` v0.5.x
  - Event sourcing backed by PostgreSQL for MVP
  
- **Event Types**:
  - ConversationStarted
  - UserMessageSent
  - AssistantMessageSent
  - TitleChanged
  - ConversationSummarized
  - ConversationArchived
  - ConversationModelSelected (optional derived event if needed by implementation; otherwise derive from latest message)
  
- **Schema**:
  - Single `events` table with (aggregate_type, aggregate_id, sequence, event_type, event_version, payload, metadata) as PK-guarded optimistic concurrency
  - Separate view tables per projection storing JSON payloads
  
- **Three Projections**:
  - **ConversationHistoryView**: Full message list for display
  - **ConversationListEntry**: Sidebar conversation list with titles and last message preview
  - **ConversationMemoryView**: Sliding window + summary for AI context
  
- **Testing Strategy**:
  - cqrs-es TestFramework provides given/when/then synchronous tests with no database
  - Projection tests via direct View::update() calls
  - Integration tests use Postgres/Redis-backed test setups where persistence matters
  
- **Snapshotting**:
  - PersistedEventStore::new_snapshot_store(repo, 50) for long conversations
  - Reduces replay time for conversations with many messages
  
- **Streaming Implementation Decision**:
  - Buffer streaming chunks in memory
  - Emit single AssistantMessageSent event when stream completes
  - Don't store individual chunks as events

### Multi-Model AI Routing

- **Primary Approach**: `async-openai` v0.33.0 with custom base_url
  - Via OpenAIConfig::new().with_api_base()
  - Works with Ollama, vLLM, llama.cpp, Gemini OpenAI-compat, real OpenAI
  
- **LlmBackend Trait**: Custom abstraction layer
  - Methods: `complete()` and `complete_stream()`
  - Implementation: OpenAiCompatBackend (wraps async-openai)
  - Future implementation: CliAgentBackend (wraps CLI tools)
  
- **ProviderRouter**: Service layer
  - HashMap-based router keyed by "provider_id/model_id"
  - Resolves to Arc<dyn LlmBackend>
  
- **ProviderRegistry**: Configuration struct
  - providers, models, base_urls, api_keys
  - Loaded from TOML configuration and environment variables
  
- **Streaming Compatibility**:
  - All major OAI-compatible servers use same SSE format
  - Anthropic differs (different SSE dialect) — use CLI delegation or OpenRouter for Claude
  
- **Future Extensibility**:
  - CliAgentBackend for claude/gemini/codex CLI tools
  - OpenRouter as meta-provider shortcut

### Authentication & Authorization

- **Dependencies**:
  - `axum-login` v0.18.0
  - `tower-sessions` v0.15.0
  - `tower-sessions-sqlx-store` v0.15.0
  - `password-auth` v1.0.0 (Argon2id)
  
- **WebSocket Authentication**:
  - Cookie-based sessions work automatically
  - Browser sends session cookie during WebSocket upgrade HTTP request
  - AuthSession extractor resolves before socket opens
  
- **User Model**:
  - `users` table: id, username, password (Argon2 PHC hash), role ('owner' | 'user')
  
- **Role-Based Features**:
  - Owner gets personalized system prompt via OWNER_SYSTEM_PROMPT env var
  - Regular users get friendly default via USER_SYSTEM_PROMPT env var
  - Stored in environment variables for easy configuration
  
- **Session Configuration**:
  - 30-day expiry
  - SameSite=Lax, HttpOnly, Signed cookies
  - SESSION_SECRET env var for persistence across restarts
  
- **Frontend Login Flow**:
  - Simple username/password form
  - Pinia auth store for state management
  - Vue Router guard with /api/auth/me check on navigation
  
- **Owner Seeding**:
  - CLI subcommand or env var on first startup
  
- **CSRF Protection**:
  - SameSite=Lax sufficient since SPA and API share same origin via Caddy

### Updated Project Context & UX Requirements

- **Real-time Communication**: WebSockets for streaming (not SSE)
  - Bidirectional for future agent use
  
- **Conversation Storage**: Event Sourcing with swappable projections/views
  
- **Multi-User Support**:
  - Owner (personalized system prompt) vs regular user (friendly default)
  - Role-based system prompts
  
- **Development Workflow**: Native dev (no Docker for dev, Docker for deployment)
  - Faster iteration with local Rust and Vite
  
- **Multi-Model Support**:
  - OpenAI-compatible default (Ollama, vLLM, etc.)
  - Model selection from frontend dropdown
  
- **UI/UX Theme**:
  - Dark theme as primary
  - Purple (#8B5CF6 or similar orchid purple) as accent color
  - Sidebar for conversation list
  - Streaming responses with animated token-by-token display
  
- **Message Animations**:
  - Token-by-token streaming with character-level display animation
  - Conversation fade-in transitions
  - Smooth typing indicators

## Clarification Checklist

Use this section to resolve the remaining product and architecture choices in small, explicit decisions before implementation. Each item includes a recommended default so decisions can be made quickly.

### 1. Cross-Conversation Memory

- **Question:** For MVP, what does "reference previous conversation memory" mean in practice?
- **Why this matters:** This changes the backend memory model, the prompt-building logic, and whether the UI needs to expose retrieved memories.
- **Recommended default:** Automatic per-user memory recall from that user's prior conversations only, using a lightweight summary-based approach rather than vector search.
- **Decision:** Accepted. MVP uses automatic per-user memory recall from that user's prior conversations only, backed by summaries instead of vector search.
- **Decision details:**
  - Memory is automatic rather than manually invoked
  - Memory is scoped to the authenticated user only
  - Archived conversations may still be used as memory sources unless later excluded for UX reasons
  - Memory usage is not surfaced in the UI during MVP
- **Need to decide:**
  - None for MVP unless you want to change archived-conversation behavior later

### 2. Transport Direction

- **Question:** Confirm that WebSockets fully replace SSE for MVP streaming.
- **Why this matters:** Earlier research mentions SSE, but the later locked direction uses WebSockets.
- **Recommended default:** WebSockets only for streaming and future agent support; do not implement SSE in MVP.
- **Decision:** Accepted. WebSockets fully replace SSE for MVP streaming.
- **Decision details:**
  - Only WebSockets are implemented for live chat streaming
  - SSE is not implemented in the backend or frontend MVP path
  - Caddy and frontend behavior should assume WebSocket-first streaming

### 3. Persistence Stack

- **Question:** Confirm that PostgreSQL + Redis fully replace the earlier SQLite research notes.
- **Why this matters:** Migrations, local setup, Docker, session storage, and CQRS wiring all depend on this.
- **Recommended default:** PostgreSQL for events/views/sessions, Redis for streaming coordination and transient buffering, no SQLite in MVP.
- **Decision:** Accepted. PostgreSQL + Redis fully replace the earlier SQLite notes for MVP.
- **Decision details:**
  - PostgreSQL stores events, snapshots, projections, users, and sessions
  - Redis supports streaming coordination and transient runtime needs
  - SQLite is not part of the MVP runtime or migration plan

### 4. Authentication Scope

- **Question:** Is multi-user auth part of MVP, or should MVP start as a single-user local app?
- **Why this matters:** Auth is a large scope addition that affects API design, UI flow, and deployment complexity.
- **Recommended default:** Keep auth in MVP as currently planned, but make the initial UX very small: login form, owner seed flow, and basic user isolation only.
- **Decision:** Accepted. MVP includes auth using simple username/password combinations.
- **Decision details:**
  - Multi-user support stays in MVP
  - Authentication uses username/password only
  - Session-cookie auth remains the default browser flow
  - Account management stays minimal: seeded owner user plus basic regular users
  - No OAuth, magic links, password reset, or profile management in MVP

### 5. Canonical Message Send Flow

- **Question:** Should the frontend send chat messages via REST, via WebSocket, or use both with clearly different purposes?
- **Why this matters:** The current plan includes both `POST /api/conversations/:id/messages` and WebSocket `send_message`, which risks duplicate logic.
- **Recommended default:** Use REST to create conversations and perform CRUD, and use WebSocket as the canonical send/stream path for live chat messages.
- **Decision:** Accepted. REST handles conversation CRUD, and WebSocket is the canonical send/stream path for live chat messages.
- **Decision details:**
  - Conversation creation, listing, retrieval, rename, archive, auth, model listing, and health checks remain HTTP endpoints
  - Live message send and assistant token streaming happen over WebSocket
  - MVP avoids duplicating primary send logic across REST and WebSocket
- **Need to decide:**
  - Whether the REST message endpoint remains as compatibility/fallback scaffolding or is omitted from the initial frontend flow
  - Whether the frontend hard-blocks send until WebSocket is connected or queues locally for a moment

### 6. Conversation Title Behavior

- **Question:** How should conversation titles work in MVP?
- **Why this matters:** The sidebar UX and initial conversation creation flow depend on it.
- **Recommended default:** New conversations start as `New chat`; users may rename them manually; automatic title generation is deferred.
- **Decision:** Adjusted. New conversations start with a timestamp-based default title, users may rename them manually, and AI-generated titles are included in MVP.
- **Decision details:**
  - A newly created conversation gets a timestamp-based placeholder title immediately so the sidebar has a stable label
  - Users can edit titles manually at any time
  - MVP includes AI-generated titles as an enhancement over the placeholder title
  - The placeholder title should be replaced only after the AI-generated title is available or if the user manually renames first
- **Implementation note:** AI title generation should be lightweight and non-blocking so it does not delay the first chat response.

### 7. Model Selection Rules

- **Question:** What are the fallback and persistence rules for model selection?
- **Why this matters:** The backend records model per message, but the product behavior around defaults is still not explicit.
- **Recommended default:** Model selection is per message, the UI remembers the last selected model locally for convenience, and the backend uses a configured default model if none is supplied.
- **Decision:** Adjusted. The backend remembers the last selected model per conversation based on the most recent request.
- **Decision details:**
  - Each message still records its explicit model choice for event/history accuracy
  - Each conversation also tracks a current model preference derived from the latest message request
  - When the user sends a new message without manually changing the selector, the conversation's last-used model becomes the default
  - A global configured default model is used only when a conversation has no prior model history yet
  - For MVP, all authenticated users can see the same available model list unless later restricted
- **Need to decide:**
  - None for MVP
- **Configured default for MVP:**
  - Default OpenAI-compatible endpoint: `http://192.168.1.50:8080`
  - Default model ID: `Hermes-3-Llama-3.1-8B-Q6_K_L`

### 8. WebSocket Protocol Details

- **Question:** What exact MVP protocol details should be standardized for the WebSocket connection?
- **Why this matters:** Both frontend and backend need a stable contract before implementation.
- **Recommended default:**
  - Server sends an `ack` frame immediately after successful connection
  - Client sends `send_message`
  - Server sends `token`, then `done`, or `error`
  - Each frame includes `conversation_id`
  - Reconnect/resume is out of scope for MVP
- **Decision:** Accepted. MVP standardizes on the recommended lightweight WebSocket protocol.
- **Decision details:**
  - Server sends `ack` immediately after successful authenticated connection
  - Client sends `send_message` frames for live chat requests
  - Server streams `token` frames followed by `done`, or sends `error`
  - All chat-related frames include `conversation_id`
  - Reconnect/resume is out of scope for MVP
  - Idempotency guarantees for repeated `send_message` calls are out of scope for MVP
- **Need to decide:**
  - Whether frames should also include message IDs and timestamps in the first protocol version, or whether that can be added later without affecting MVP goals

### 9. MVP UI Scope Around Memory and Conversations

- **Question:** What must the UI expose in MVP beyond the basic chat surface?
- **Why this matters:** The backend could support memory internally without adding visible UI, which is much smaller scope.
- **Recommended default:** Keep the UI focused on login, sidebar, conversation history, model picker, and streaming chat. Do not add search, delete, or visible memory citations in MVP unless explicitly requested.
- **Decision:** Accepted. MVP UI stays focused on the core chat experience and defers non-essential management features.
- **Decision details:**
  - Include login, sidebar, conversation history, model picker, and streaming chat
  - Defer search, hard delete, visible memory citations, reconnect UI, and advanced settings
  - Archive remains sufficient for MVP conversation management
  - Memory usage stays invisible in the MVP UI even when used behind the scenes
- **Need to decide:**
  - None for MVP unless you later want visible memory references or stronger conversation management tools

### 10. Recommended Decision Order

To keep this ADHD-friendly, resolve the checklist in this order:

1. Cross-conversation memory behavior
2. Authentication scope
3. Canonical message send flow
4. Conversation title behavior
5. Model selection defaults
6. WebSocket protocol details
7. UI extras to include or defer

## Implementation-Ready Decisions

- **Transport:** WebSockets are the only MVP streaming transport. REST remains for auth, health, model listing, and conversation CRUD.
- **Persistence:** PostgreSQL + Redis are the only MVP persistence/runtime services. Earlier SQLite notes are superseded.
- **Memory:** Cross-conversation memory is automatic, per-user, summary-based, and hidden from the UI in MVP.
- **Auth:** MVP includes simple username/password auth with session cookies, one seeded owner, and regular users.
- **Chat flow:** The frontend creates and manages conversations over HTTP, then sends live chat messages over WebSocket.
- **Titles:** New conversations get timestamp-based titles immediately; users can rename them; AI-generated titles are part of MVP and should update non-blockingly.
- **Models:** Each message records its model explicitly, and each conversation remembers its last-used model as the default for the next message.
- **Default model config:** Brand-new conversations default to the OpenAI-compatible endpoint `http://192.168.1.50:8080` and model `Hermes-3-Llama-3.1-8B-Q6_K_L` until a different model is chosen.
- **UI scope:** Include login, sidebar, history, model picker, and streaming chat. Defer search, hard delete, visible memory citations, reconnect UX, and advanced settings.

## Acceptance Tests

**Status: LOCKED** — Confirmed by user. Do not modify, delete, rename, or skip any test. Implementation agents must make these pass, not change them.

### Backend Tests (55 tests across 7 files)

All backend acceptance tests are in `backend/tests/` as Rust integration test files. Each test body currently contains `todo!("not yet implemented")` and will panic until the implementation makes them pass.

#### File: `backend/tests/acceptance_event_sourcing.rs` (11 tests)
- `starting_a_conversation_emits_conversation_started_event` — StartConversation command → ConversationStarted event with conversation ID, user ID, timestamp
- `sending_a_user_message_emits_user_message_sent_event` — SendUserMessage → UserMessageSent with content, user_id, model_id
- `completing_assistant_response_emits_assistant_message_sent_event` — CompleteAssistantResponse → AssistantMessageSent with full content and model
- `changing_conversation_title_emits_title_changed_event` — ChangeTitle → TitleChanged with new title
- `archiving_a_conversation_emits_conversation_archived_event` — ArchiveConversation → ConversationArchived
- `cannot_send_message_to_archived_conversation` — Error when messaging archived conversation
- `cannot_archive_an_already_archived_conversation` — Error on double-archive
- `empty_message_content_is_rejected` — Error on blank/whitespace-only messages
- `conversation_state_is_correct_after_many_events` — Replay correctness for 50+ events
- `model_selection_is_recorded_per_message` — Different model IDs recorded per message event
- `apply_replays_events_to_reconstruct_aggregate_state` — All event types correctly update aggregate state

#### File: `backend/tests/acceptance_auth.rs` (8 tests)
- `login_with_valid_credentials_returns_success_and_session_cookie` — POST /api/auth/login → 200 + Set-Cookie
- `login_with_invalid_credentials_returns_unauthorized` — Bad password → 401, no cookie
- `protected_route_rejects_unauthenticated_request` — No cookie → 401
- `protected_route_allows_authenticated_request` — Valid cookie → 200
- `auth_me_returns_current_user_info` — GET /api/auth/me → user info (id, username, role)
- `logout_invalidates_session` — POST /api/auth/logout → session invalidated
- `user_can_only_see_own_conversations` — User A can't see User B's conversations
- `owner_role_gets_personalized_system_prompt` — Owner gets OWNER_SYSTEM_PROMPT

#### File: `backend/tests/acceptance_api.rs` (10 tests)
- `create_conversation_returns_201_with_conversation_id` — POST /api/conversations → 201 Created
- `list_conversations_returns_conversation_summaries` — GET /api/conversations → JSON array of summaries
- `get_conversation_returns_full_message_history` — GET /api/conversations/:id → full messages
- `get_nonexistent_conversation_returns_404` — Unknown ID → 404
- `update_conversation_title_returns_200` — PATCH /api/conversations/:id → 200
- `archive_conversation_returns_200` — DELETE /api/conversations/:id → 200
- `list_available_models_returns_configured_providers` — GET /api/models → model list
- `send_message_with_model_selection_returns_202` — POST messages with model_id → 202
- `accessing_another_users_conversation_returns_403` — Cross-user access → 403
- `health_check_endpoint_returns_200` — GET /api/health → 200 with status

#### File: `backend/tests/acceptance_websocket.rs` (6 tests)
- `websocket_connection_requires_authentication` — No auth → WS rejected
- `websocket_connection_succeeds_with_valid_session` — Auth → WS connected + ack
- `websocket_streams_assistant_response_tokens` — Token chunks as WS frames + done signal
- `websocket_sends_error_on_ai_service_failure` — AI failure → error frame
- `websocket_handles_concurrent_conversations` — Multi-conversation demux on single WS
- `websocket_graceful_close` — Clean close handshake

#### File: `backend/tests/acceptance_multi_model.rs` (6 tests)
- `provider_router_routes_to_correct_backend_by_model_id` — Model routing correctness
- `provider_router_returns_error_for_unknown_model` — Unknown model → error
- `llm_backend_streams_completion_via_openai_compatible_api` — Streaming via wiremock
- `llm_backend_sends_full_message_history_in_request` — Full context sent to API
- `llm_backend_handles_api_timeout_gracefully` — Timeout → error, not hang
- `model_list_reflects_provider_registry_configuration` — Config → available models

#### File: `backend/tests/acceptance_projections.rs` (6 tests)
- `conversation_history_view_builds_full_message_list` — Full message replay in view
- `conversation_list_entry_shows_title_and_last_message_preview` — Sidebar summary data
- `conversation_list_entry_updates_on_title_change` — Title change reflected in list
- `conversation_memory_view_maintains_sliding_window` — Sliding window for AI context
- `archived_conversation_is_excluded_from_active_list` — Archive removes from active list
- `conversation_history_view_records_model_per_message` — Model ID in history view

#### File: `backend/tests/acceptance_infrastructure.rs` (8 tests)
- `env_example_file_exists_with_required_variables` — .env.example with all required vars
- `justfile_exists_with_dev_commands` — justfile with dev/build/test/check/fmt/lint/db-migrate/db-reset/seed
- `docker_compose_file_defines_required_services` — docker-compose with postgres/redis/backend/frontend/caddy
- `caddyfile_routes_api_and_spa_correctly` — Caddy with /api/* routing + SPA fallback
- `gitignore_excludes_build_artifacts_and_secrets` — .gitignore with target/node_modules/.env/dist
- `backend_dockerfile_uses_cargo_chef_multi_stage_build` — Three-stage cargo-chef build
- `check_env_script_validates_required_tools` — scripts/check-env.sh validates toolchain
- `database_migrations_directory_contains_initial_migration` — Initial PostgreSQL migration exists

### Frontend Tests (37 test specifications)

Located at `frontend/acceptance-tests/frontend-acceptance-tests.spec.ts` as specifications. These will be written as real `.spec.ts` files in the proper co-located `__tests__/` directories when the Vue project is scaffolded.

**Components (20 tests):**
- MessageList (6): renders all messages, empty state, role styling, markdown rendering, XSS sanitization, accessibility
- MessageInput (4): submit emits send, clears after submit, rejects empty, disables during streaming
- Sidebar (4): renders conversations, highlights active, click calls setActive, new chat button
- ModelSelector (3): renders models, emits model-change, displays current selection
- StreamingIndicator (3): visible when streaming, displays partial content, aria-live

**Stores (6 tests):**
- ConversationStore (3): empty init, addMessage, setActive
- AuthStore (3): not authenticated init, login sets user, logout clears

**Composables (3 tests):**
- useChat (3): establishes WS connection, assembles streaming tokens, isStreaming lifecycle

**Animations (2 tests):**
- ChatMessage (2): shimmer class on streaming, skips animation with prefers-reduced-motion

**Login (3 tests):**
- LoginForm (3): submits credentials, shows error, disables button during attempt

**Theme (2 tests):**
- App (2): dark theme with orchid purple, sidebar present

**Testing conventions:** Vitest 4.x, Vue Test Utils 2.x, @pinia/testing with createTestingPinia(), jsdom environment, MSW 2.x for WebSocket mocking, vi.stubGlobal for matchMedia, data-test attributes for selectors.

## Units of Work

### Unit 1: Infrastructure & Dev Environment
**Status:** verified
**Description:** Create all project infrastructure files: .gitignore, .env.example, justfile, dev scripts, Docker configuration, Caddyfile, and initial PostgreSQL migration.
**Acceptance Tests:**
- `acceptance_infrastructure::env_example_file_exists_with_required_variables`
- `acceptance_infrastructure::justfile_exists_with_dev_commands`
- `acceptance_infrastructure::docker_compose_file_defines_required_services`
- `acceptance_infrastructure::caddyfile_routes_api_and_spa_correctly`
- `acceptance_infrastructure::gitignore_excludes_build_artifacts_and_secrets`
- `acceptance_infrastructure::backend_dockerfile_uses_cargo_chef_multi_stage_build`
- `acceptance_infrastructure::check_env_script_validates_required_tools`
- `acceptance_infrastructure::database_migrations_directory_contains_initial_migration`
**Files to create/modify:**
- `.gitignore`
- `.env.example`
- `justfile`
- `scripts/check-env.sh`, `scripts/dev-up.sh`, `scripts/dev-down.sh`, `scripts/dev-reset.sh`, `scripts/seed.sh`
- `docker/docker-compose.yml`
- `docker/Caddyfile`
- `docker/backend.Dockerfile`
- `docker/frontend.Dockerfile`
- `backend/migrations/` (initial migration)
**Dependencies:** None — this unit can be done first.
**Research context:** Caddy 2.11.2, cargo-chef 3-stage build, postgres:17-alpine + redis:7-alpine, Docker Compose profiles (infra/app/full), just 1.46.0. Migration must create: events table, snapshots table, users table (id, username, password, role), sessions table, and view tables.

### Unit 2: Conversation Aggregate (Domain Model)
**Status:** verified
**Description:** Implement the Conversation aggregate, commands, events, error types, and the `handle()`/`apply()` methods using cqrs-es. This is pure domain logic with no I/O.
**Acceptance Tests:**
- All 11 tests in `acceptance_event_sourcing.rs`
**Files to create/modify:**
- `backend/src/domain/mod.rs`
- `backend/src/domain/conversation.rs` (aggregate, commands, events, errors)
- `backend/src/lib.rs` (add domain module)
**Dependencies:** None — pure logic, no infrastructure needed.
**Research context:** cqrs-es 0.5 with `Aggregate` trait, `EventSink` for pushing events in `handle()`. Events: ConversationStarted, UserMessageSent, AssistantMessageSent, TitleChanged, ConversationSummarized, ConversationArchived. ConversationServices struct for injecting AI client. TestFramework for given/when/then tests. Event and Error types need `PartialEq, Debug, Clone, Serialize, Deserialize`. Error needs `From<&str>`.

### Unit 3: Event Sourcing Projections (Views)
**Status:** pending
**Description:** Implement the three projection views: ConversationHistoryView, ConversationListEntry, ConversationMemoryView. These transform events into queryable read models and support automatic summary-based memory recall.
**Acceptance Tests:**
- All 6 tests in `acceptance_projections.rs`
**Files to create/modify:**
- `backend/src/domain/views.rs` (or `backend/src/views/` directory)
- `backend/src/lib.rs` (add views module)
**Dependencies:** Unit 2 (needs event types defined).
**Research context:** cqrs-es `View` trait with `update()` method. ConversationHistoryView = full message list. ConversationListEntry = title + last_message_preview + updated_at for sidebar plus current effective model hint if useful. ConversationMemoryView = sliding window of recent messages plus summary data for same-user cross-conversation recall. Each view stores JSON payload. Views must track model_id per message.

### Unit 4: Multi-Model AI Routing
**Status:** in-progress
**Description:** Implement the LlmBackend trait, OpenAiCompatBackend (wrapping async-openai), ProviderRouter, and ProviderRegistry for configuring and routing to multiple AI backends.
**Acceptance Tests:**
- All 6 tests in `acceptance_multi_model.rs`
**Files to create/modify:**
- `backend/src/ai/mod.rs`
- `backend/src/ai/backend.rs` (LlmBackend trait + OpenAiCompatBackend)
- `backend/src/ai/router.rs` (ProviderRouter)
- `backend/src/ai/registry.rs` (ProviderRegistry)
- `backend/src/lib.rs` (add ai module)
**Dependencies:** None — this is independent of the domain model.
**Research context:** async-openai 0.33 with `OpenAIConfig::new().with_api_base()` for custom base URLs. LlmBackend trait with `complete()` and `complete_stream()` methods. ProviderRouter keyed by "provider_id/model_id" → Arc<dyn LlmBackend>. Use wiremock for testing. Handle timeouts gracefully. Streaming returns a `Stream<Item = Result<String>>` of token chunks.

### Unit 5: Authentication & Session Management
**Status:** pending
**Description:** Implement user authentication with axum-login, session management with tower-sessions (PostgresStore), login/logout/me endpoints, and route protection middleware.
**Acceptance Tests:**
- All 8 tests in `acceptance_auth.rs`
**Files to create/modify:**
- `backend/src/auth/mod.rs`
- `backend/src/auth/backend.rs` (AuthnBackend implementation)
- `backend/src/auth/models.rs` (User model)
- `backend/src/auth/handlers.rs` (login, logout, me handlers)
- `backend/src/lib.rs` (add auth module)
**Dependencies:** Unit 1 (needs migrations for users table and sessions table).
**Research context:** axum-login 0.18 with AuthManagerLayerBuilder. tower-sessions 0.15 with PostgresStore for session persistence. password-auth 1.0 for Argon2id hashing. User model: id, username, password (PHC hash), role ('owner'|'user'). Cookie-based sessions with SameSite=Lax, HttpOnly, Signed. MemoryStore for tests. Owner gets OWNER_SYSTEM_PROMPT env var, regular users get USER_SYSTEM_PROMPT.

### Unit 6: REST API Endpoints
**Status:** pending
**Description:** Implement the Axum HTTP handlers and router for conversation CRUD, optional compatibility message submission, model listing, and health check. Wire up authentication middleware and CQRS dispatching.
**Acceptance Tests:**
- All 10 tests in `acceptance_api.rs`
**Files to create/modify:**
- `backend/src/api/mod.rs`
- `backend/src/api/handlers.rs` (conversation CRUD, messages, models, health)
- `backend/src/api/router.rs` (route definitions)
- `backend/src/api/models.rs` (request/response DTOs)
- `backend/src/lib.rs` (add api module, create_router function)
**Dependencies:** Unit 2 (conversation aggregate), Unit 3 (projections for reads), Unit 4 (model list), Unit 5 (auth middleware).
**Research context:** Axum 0.8 with Router, State, extractors. Routes: POST /api/conversations, GET /api/conversations, GET /api/conversations/:id, PATCH /api/conversations/:id, DELETE /api/conversations/:id, POST /api/conversations/:id/messages, GET /api/models, GET /api/health. Use login_required! macro for protection. CorsLayer from tower-http. The frontend's canonical live send path is WebSocket; if the REST message endpoint remains, treat it as compatibility/fallback and still return 202. Health check pings DB + Redis.

### Unit 7: WebSocket Streaming
**Status:** pending
**Description:** Implement the WebSocket endpoint for real-time chat streaming. Handle authentication, message routing, token streaming from AI backends, error handling, graceful close, and the stable MVP protocol contract.
**Acceptance Tests:**
- All 6 tests in `acceptance_websocket.rs`
**Files to create/modify:**
- `backend/src/ws/mod.rs`
- `backend/src/ws/handler.rs` (WebSocket upgrade + message loop)
- `backend/src/ws/protocol.rs` (WS message types: ack, token, done, error, send_message)
- `backend/src/lib.rs` (add ws module, add WS route)
**Dependencies:** Unit 2 (conversation aggregate for commands), Unit 4 (AI backend for streaming), Unit 5 (auth for WS upgrade).
**Research context:** Axum WebSocket via `features = ["ws"]`. Cookie-based auth works on WS upgrade (browser sends cookie). Protocol: server sends `{ type: "ack" }` after successful connection; client sends `{ type: "send_message", conversation_id, content, model_id? }`; server streams `{ type: "token", conversation_id, content }` chunks then `{ type: "done", conversation_id }`, or `{ type: "error", conversation_id, message }`. Buffer chunks in memory/Redis, emit single AssistantMessageSent event when stream completes. Single WS connection handles multiple conversations via conversation_id tagging. Reconnect/resume and idempotency are out of scope for MVP.

### Unit 8: Backend Main & App Wiring
**Status:** pending
**Description:** Wire up main.rs with all components: database pool, Redis pool, CQRS framework with postgres-es, auth layer, router, and server startup. Implement the seed command for creating the owner user.
**Acceptance Tests:**
- Indirectly supports all integration tests that need a running app.
- Specifically enables the full test harness for acceptance_api, acceptance_auth, and acceptance_websocket tests.
**Files to create/modify:**
- `backend/src/main.rs`
- `backend/src/lib.rs` (create_app function that builds the full Router with state)
- `backend/src/config.rs` (environment variable loading)
**Dependencies:** All backend units (2-7).
**Research context:** postgres-es `postgres_cqrs()` function to build CqrsFramework. `default_postgress_pool()` for connection pool. deadpool-redis for Redis pool. dotenvy for .env loading. tracing-subscriber for logging. Bind to 0.0.0.0:3000. Create a `create_app()` function that returns Router (testable) and `main()` that just calls serve.

### Unit 9: Frontend Scaffolding & Core Components
**Status:** in-progress (retrying with bun-based tooling)
**Description:** Scaffold the Vue.js project, install dependencies, set up Vitest + testing infrastructure, and implement the MVP UI: login flow, sidebar, chat views, model selection, timestamp-backed conversation titles, and WebSocket-driven streaming components. Write all frontend acceptance tests as real .spec.ts files.
**Acceptance Tests:**
- All 37 frontend test specifications (MessageList 6, MessageInput 4, Sidebar 4, ModelSelector 3, StreamingIndicator 3, ConversationStore 3, AuthStore 3, useChat 3, ChatMessage animations 2, LoginForm 3, App theme 2, MarkdownContent implicit)
**Files to create/modify:**
- `frontend/` (full Vue project scaffold)
- `frontend/src/components/MessageList.vue`
- `frontend/src/components/MessageInput.vue`
- `frontend/src/components/Sidebar.vue`
- `frontend/src/components/ModelSelector.vue`
- `frontend/src/components/StreamingIndicator.vue`
- `frontend/src/components/ChatMessage.vue`
- `frontend/src/components/LoginForm.vue`
- `frontend/src/components/MarkdownContent.vue`
- `frontend/src/stores/conversation.ts`
- `frontend/src/stores/auth.ts`
- `frontend/src/composables/useChat.ts`
- `frontend/src/views/ChatView.vue`
- `frontend/src/views/LoginView.vue`
- `frontend/src/App.vue`
- `frontend/src/test/setup.ts` (MSW + matchMedia mock)
- `frontend/vitest.config.ts`
- All corresponding `__tests__/*.spec.ts` files
**Dependencies:** None for scaffolding. UI behavior depends on backend API existing (mocked in tests).
**Research context:** `npm create vue@latest` with TypeScript, Router, Pinia, Vitest, ESLint. Pinia 3.x with setup store pattern. Vitest 4.x + jsdom + Vue Test Utils 2.x. MSW 2.x for WebSocket mocking. @vueuse/motion for animations. markdown-it + Shiki for rendering. Dark theme with --color-primary: #8B5CF6. data-test attributes for test selectors. createTestingPinia({ createSpy: vi.fn }) for component tests.

### Unit 10: Frontend Theme, Animations & Polish
**Status:** pending
**Description:** Implement the dark theme with orchid purple accent, CSS shimmer animations for streaming text, prefers-reduced-motion handling, and conversation transitions.
**Acceptance Tests:**
- Frontend: App dark theme test, ChatMessage shimmer test, ChatMessage reduced-motion test
**Files to create/modify:**
- `frontend/src/assets/styles/theme.css` (CSS custom properties, dark theme)
- `frontend/src/assets/styles/animations.css` (shimmer, transitions)
- `frontend/src/composables/useReducedMotion.ts` (wrapper around @vueuse/motion)
- Component updates for animation classes
**Dependencies:** Unit 9 (needs components to exist).
**Research context:** CSS `background-clip: text` gradient shimmer sweep as primary effect. @vueuse/motion `useReducedMotion()` backed by `useMediaQuery('(prefers-reduced-motion: reduce)')`. animejs 4.3.6 for per-character stagger. MotionPlugin in app setup. vi.stubGlobal('matchMedia', ...) for testing.

## Context and Notes

- **Task Title**: Orchid: AI Chatbot with Rust Backend, Vue.js Frontend, and Docker Deployment
- **Task ID**: 20260307095408
- **Created**: 2026-03-07
- **Architecture**: Event-sourced backend (cqrs-es + postgres-es), Axum REST + WebSocket, Vue 3 SPA, Caddy reverse proxy
- **Tech Stack**: Rust (Axum 0.8, cqrs-es 0.5, postgres-es 0.5, async-openai 0.33), Vue.js 3 (Vite 7, Pinia 3, Vitest 4), Caddy 2.11, PostgreSQL 17, Redis 7, Docker
- **Key Decisions**:
  - WebSockets (not SSE) for streaming — bidirectional for future agent support
  - Event Sourcing for conversation storage with swappable projections
  - PostgreSQL + Redis (not SQLite) — Postgres for events/views/sessions, Redis for chunk buffering
  - Cross-conversation memory is automatic, per-user, summary-based, and not exposed in the MVP UI
  - REST handles CRUD while WebSocket is the canonical live chat send/stream path
  - Conversation titles start as timestamps, remain editable, and are later upgraded with non-blocking AI title generation
  - Model usage is recorded per message while the backend remembers the last-used model per conversation
  - Brand-new conversations default to `http://192.168.1.50:8080` with model `Hermes-3-Llama-3.1-8B-Q6_K_L`
  - Multi-model via async-openai with custom base_url + LlmBackend trait + ProviderRouter
  - Native dev workflow (Docker for deployment only)
  - Dark theme with orchid purple (#8B5CF6), CSS shimmer animations, prefers-reduced-motion
  - Owner vs regular user roles with different system prompts
- **Total Acceptance Tests**: 55 backend (compiled, failing) + 37 frontend (specifications) = 92
- **Unit of Work Dependency Graph**:
  - Independent: Unit 1 (infra), Unit 2 (domain), Unit 4 (AI routing), Unit 9 (frontend scaffold)
  - After Unit 2: Unit 3 (projections)
  - After Unit 1: Unit 5 (auth)
  - After Units 2+3+4+5: Unit 6 (API)
  - After Units 2+4+5: Unit 7 (WebSocket)
  - After Units 2-7: Unit 8 (wiring)
  - After Unit 9: Unit 10 (theme/animations)
