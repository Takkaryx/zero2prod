CREATE TABLE subscriptions (
    id uuid NOT NULL,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL,
    PRIMARY KEY (id)
);-- Add migration script here
