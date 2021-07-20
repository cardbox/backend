-- Add up migration script here
CREATE TYPE box_type AS ENUM ('user');

CREATE TABLE IF NOT EXISTS boxes
(
    id      uuid DEFAULT uuid_generate_v4(),
    user_id uuid     NOT NULL REFERENCES users ("id"),
    type    box_type NOT NULL,

    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "boxes_user_id" ON boxes USING btree ("user_id");