-- Your SQL goes here
CREATE TABLE tv_shows (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  director VARCHAR NOT NULL,
  rating FLOAT NOT NULL,
  summary TEXT NOT NULL
);