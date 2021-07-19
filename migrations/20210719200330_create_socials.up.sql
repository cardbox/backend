-- Add up migration script here
CREATE TABLE IF NOT EXISTS socials
(
    id      uuid    NOT NULL DEFAULT uuid_generate_v4(),
    user_id uuid    NOT NULL REFERENCES users ("id"),
    name    varchar NOT NULL,
    link    varchar NOT NULL
);