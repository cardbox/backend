-- Add down migration script here
DROP INDEX ts_user_index;
ALTER TABLE users DROP COLUMN ts;
DROP FUNCTION to_tsvector_multilang(text);
DROP FUNCTION plainto_tsquery_multilang(text);