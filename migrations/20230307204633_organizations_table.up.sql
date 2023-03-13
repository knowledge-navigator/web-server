-- Add up migration script here
CREATE TABLE IF NOT EXISTS organizations (
    id serial PRIMARY KEY,
    name VARCHAR (255) NOT NULL,
    description TEXT,
    utc_created TIMESTAMPTZ,
    utc_last_updated TIMESTAMPTZ,
    moderators INT [],
    members INT []
);