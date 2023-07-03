-- Your SQL goes here
CREATE TABLE user_videos (
  id SERIAL PRIMARY KEY,
  tv_shows_id SERIAL REFERENCES tv_shows(id),
  users_id SERIAL REFERENCES users(id)
);