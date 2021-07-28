-- Add up migration script here
ALTER TABLE boxes
    ADD COLUMN "default" bool NOT NULL DEFAULT false;

ALTER TABLE boxes
    ADD CONSTRAINT one_default_box UNIQUE (user_id, "default");

CREATE OR REPLACE FUNCTION default_user_box()
    RETURNS trigger AS
$$
BEGIN
    INSERT INTO boxes (user_id, "default", type)
    VALUES (new.id, true, 'user');

    RETURN new;
end;
$$ LANGUAGE plpgsql;

CREATE TRIGGER default_user_box_trig
    AFTER INSERT
    ON users
    FOR EACH ROW
EXECUTE PROCEDURE default_user_box();