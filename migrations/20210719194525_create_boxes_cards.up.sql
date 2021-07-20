-- Add up migration script here
CREATE TABLE IF NOT EXISTS boxes_cards
(
    box_id  uuid NOT NULL REFERENCES boxes ("id") ON UPDATE CASCADE ON DELETE CASCADE,
    card_id uuid NOT NULL REFERENCES cards ("id") ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT boxes_cards_pk PRIMARY KEY (box_id, card_id)
);