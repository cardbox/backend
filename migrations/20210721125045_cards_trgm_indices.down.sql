-- Add down migration script here
DROP INDEX index_cards_on_title_trgm;
DROP INDEX index_cards_on_contents;
DROP FUNCTION jsonb_to_tsvector_multilang(jsonb, jsonb);