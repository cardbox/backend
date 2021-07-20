-- Add down migration script here
ALTER TABLE session_tokens
    ALTER COLUMN expires_at SET DATA TYPE timestamp;