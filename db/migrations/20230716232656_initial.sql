-- Add migration script here
CREATE TABLE IF NOT EXISTS items (
  id           INTEGER PRIMARY KEY NOT NULL,
  is_members   INTEGER NOT NULL, -- Change to BOOLEAN on postgres
  alch_low     INTEGER,
  alch_high    INTEGER,
  buy_limit    INTEGER,
  value        INTEGER,
  icon         TEXT,
  examine_text TEXT,
  last_updated INTEGER
)
