-- Your SQL goes here
CREATE TABLE episodes (
  id SERIAL PRIMARY KEY,
  episode_name VARCHAR NOT NULL,
  director VARCHAR NOT NULL,
  summary TEXT NOT NULL,
  seasons_id SERIAL REFERENCES seasons(id)
);