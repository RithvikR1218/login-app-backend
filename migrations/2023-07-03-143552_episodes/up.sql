-- Your SQL goes here
CREATE TABLE episodes (
  id SERIAL PRIMARY KEY,
  season_number INT NOT NULL,
  summary TEXT NOT NULL,
  seasons_id SERIAL REFERENCES seasons(id)
);