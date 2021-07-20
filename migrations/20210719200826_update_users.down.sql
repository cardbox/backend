-- Add down migration script here
ALTER TABLE users
    DROP COLUMN username,
    DROP COLUMN bio,
    DROP COLUMN avatar,
    DROP COLUMN work;