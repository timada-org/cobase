-- Add up migration script here
CREATE TABLE IF NOT EXISTS _evento_events
(
    id UUID NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    aggregate_id VARCHAR(255) NOT NULL,
    version INT4 NOT NULL,
    data JSON NOT NULL,
    metadata JSON DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idk_aggregate_id ON _evento_events (aggregate_id);

CREATE TABLE IF NOT EXISTS groups
(
    id VARCHAR(21) NOT NULL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    user_id UUID NOT NULL
);
