-- Add down migration script here
DROP TRIGGER default_user_box_trig ON users;
DROP FUNCTION default_user_box();

ALTER TABLE boxes
    DROP CONSTRAINT one_default_box;

ALTER TABLE boxes
    DROP COLUMN "default";
