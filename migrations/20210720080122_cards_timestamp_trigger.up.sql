-- Add up migration script here
CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON cards
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();