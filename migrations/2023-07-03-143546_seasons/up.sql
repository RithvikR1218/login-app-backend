-- Your SQL goes here
CREATE TABLE seasons (
  id SERIAL PRIMARY KEY,
  season_number INT NOT NULL,
  summary TEXT NOT NULL,
  tv_shows_id SERIAL REFERENCES tv_shows(id)
);