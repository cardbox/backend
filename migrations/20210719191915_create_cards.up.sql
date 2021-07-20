-- Add up migration script here
CREATE TABLE IF NOT EXISTS cards
(
    id         uuid DEFAULT uuid_generate_v4(),
    author_id  uuid        NOT NULL REFERENCES users ("id"),
    title      varchar     NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    contents   jsonb       NOT NULL,
    tags       varchar array,

    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "cards_author_id" ON cards USING btree ("author_id");