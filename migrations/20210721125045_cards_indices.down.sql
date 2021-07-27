-- Add down migration script here
DROP INDEX index_cards_on_title_trgm;
DROP INDEX index_cards_on_contents;
DROP INDEX index_cards_on_tags;

DROP FUNCTION jsonb_to_tsvector_multilang(jsonb, jsonb);

DROP EXTENSION IF EXISTS pg_trgm;