-- Add down migration script here

DROP TABLE IF EXISTS _evento_events;
DROP TABLE IF EXISTS _evento_deadletters;
DROP TABLE IF EXISTS _evento_subscriptions;

DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS warehouses;
