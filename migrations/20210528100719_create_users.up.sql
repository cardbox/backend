-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
    "id" uuid DEFAULT uuid_generate_v4(),
    "accesso_id" uuid NOT NULL,
    "first_name" varchar NOT NULL,
    "last_name" varchar NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "users_accesso_id" ON "users" USING BTREE ("accesso_id");

