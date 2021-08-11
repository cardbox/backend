-- Add up migration script here
CREATE OR REPLACE FUNCTION to_tsvector_multilang(text) RETURNS tsvector AS
$$
SELECT to_tsvector('english', $1) || to_tsvector('russian', $1) || to_tsvector('german', $1) ||
       to_tsvector('simple', $1)
$$ LANGUAGE sql IMMUTABLE;

CREATE OR REPLACE FUNCTION plainto_tsquery_multilang(text) RETURNS tsquery AS
$$
SELECT plainto_tsquery('english', $1) || plainto_tsquery('russian', $1) || plainto_tsquery('german', $1) ||
       plainto_tsquery('simple', $1)
$$ LANGUAGE sql IMMUTABLE;

ALTER TABLE users
    ADD COLUMN ts tsvector NOT NULL GENERATED ALWAYS AS (
                        to_tsvector_multilang(first_name) || to_tsvector_multilang(last_name) ||
                        to_tsvector_multilang(coalesce(bio, '')) || to_tsvector_multilang(coalesce(username, '')) ||
                        to_tsvector_multilang(coalesce(work, '')) ) STORED;

CREATE INDEX ts_user_index ON users USING gin (ts);
