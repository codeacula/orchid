-- =============================================================================
-- Projection tables for Orchid MVP read models
-- =============================================================================

ALTER TABLE users ADD COLUMN IF NOT EXISTS role VARCHAR(32);

UPDATE users
SET role = CASE WHEN is_owner THEN 'owner' ELSE 'user' END
WHERE role IS NULL;

CREATE TABLE IF NOT EXISTS conversation_history_views (
    conversation_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    title TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_conversation_history_views_user_id
    ON conversation_history_views(user_id);

CREATE TABLE IF NOT EXISTS conversation_list_entries (
    conversation_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    title TEXT NOT NULL,
    last_message_preview TEXT NOT NULL DEFAULT '',
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_conversation_list_entries_user_id
    ON conversation_list_entries(user_id);

CREATE INDEX IF NOT EXISTS idx_conversation_list_entries_updated_at
    ON conversation_list_entries(updated_at DESC);

CREATE TABLE IF NOT EXISTS conversation_memory_views (
    conversation_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    summary TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_conversation_memory_views_user_id
    ON conversation_memory_views(user_id);
