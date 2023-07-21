-- Your SQL goes here
CREATE TABLE movies (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  director VARCHAR NOT NULL,
  rating FLOAT NOT NULL,
  summary TEXT NOT NULL,
  duration FLOAT NOT NULL
);