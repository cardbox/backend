-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

CREATE OR REPLACE FUNCTION jsonb_to_tsvector_multilang(jsonb, jsonb) RETURNS
    TSVECTOR AS $$
SELECT jsonb_to_tsvector('english', $1, $2) ||
       jsonb_to_tsvector('russian', $1, $2) ||
       jsonb_to_tsvector('german', $1, $2) ||
       jsonb_to_tsvector('simple', $1, $2)
$$ LANGUAGE SQL IMMUTABLE;

CREATE INDEX index_cards_on_title_trgm ON cards USING gin (title gin_trgm_ops);

CREATE INDEX index_cards_on_tags ON cards USING gin (tags);

CREATE INDEX index_cards_on_contents ON cards USING gin (
    jsonb_to_tsvector_multilang(
        jsonb_path_query_array(contents, 'strict $.**.text'),
        '["string"]'
    )
);