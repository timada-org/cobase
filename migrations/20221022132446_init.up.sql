-- Add up migration script here
CREATE TABLE IF NOT EXISTS _evento_events
(
    id uuid NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    aggregate_id varchar(255) NOT NULL,
    version int4 NOT NULL,
    data json NOT NULL,
    metadata jsonb DEFAULT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX ON _evento_events (aggregate_id);
CREATE INDEX ON _evento_events USING GIN (metadata jsonb_ops);

CREATE TABLE IF NOT EXISTS _evento_deadletters
(
    id uuid NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    aggregate_id varchar(255) NOT NULL,
    version int4 NOT NULL,
    data json NOT NULL,
    metadata jsonb DEFAULT NULL,
    created_at timestamptz NOT NULL
);

CREATE TABLE IF NOT EXISTS _evento_subscriptions
(
    id uuid NOT NULL PRIMARY KEY,
    consumer_id uuid NOT NULL,
    key varchar(255) NOT NULL,
    enabled BOOLEAN NOT NULL,
    cursor uuid NULL,
    updated_at timestamptz NULL,
    created_at timestamptz NOT NULL
);

CREATE UNIQUE INDEX ON _evento_subscriptions (key);

CREATE TABLE IF NOT EXISTS rooms
(
    id VARCHAR(21) NOT NULL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    user_id UUID NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE TABLE IF NOT EXISTS warehouses
(
    id VARCHAR(21) NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE UNIQUE INDEX ON warehouses (user_id);
