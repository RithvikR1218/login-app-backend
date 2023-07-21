-- Your SQL goes here
CREATE TABLE user_videos (
  id SERIAL PRIMARY KEY,
  tv_shows_id INTEGER REFERENCES tv_shows(id),
  users_id INTEGER REFERENCES users(id),
  movies_id INTEGER REFERENCES movies(id),
  user_rating INTEGER,
  time_left INTEGER
);