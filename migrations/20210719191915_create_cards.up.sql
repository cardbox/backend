-- Add up migration script here
CREATE TABLE IF NOT EXISTS cards
(
    id         uuid DEFAULT uuid_generate_v4(),
    user_id    uuid        NOT NULL REFERENCES users ("id"),
    title      varchar     NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    contents   jsonb       NOT NULL,
    tags       varchar array,

    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "cards_user_id" ON cards USING btree ("user_id");