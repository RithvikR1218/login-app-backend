<b>Clockbuster:</b><br />
A Netflix Clone using micro-service architecture<br />
Consists of multiple micro-services that have a singular function<br />
login-app-backend is the main REST API used to perform CRUD operations<br />

<b>Tech Stack:</b><br />
Language: Rust<br />
Framework: Actix-Web<br />
ORM: Diesel<br />
Database: PosgreSQL<br />

<b>How to Setup Server[Linux installation]:</b><br />
- ```cd login-app-backend```<br />
- ```sudo apt install libpq-dev```
- ```cargo install diesel_cli --no-default-features --features postgres```
- ```CREATE DATABASE rust_server``` [in psql CLI];
- Add ENV variables for DB connection<br />
- ```diesel migration run``` [Generates the tables]<br />
- ```cargo run``` [server runs]

<b>Work to be done:</b><br />
- Have proper error handling on the actix side
- Make JSON for error messages
- Change names to ID in routes=> Rip 
- Add duration,rating => imdb?? for episodes
- Add rating for seasons and shows too
- Add seeded data for major data => movies and tv show data
- Make route specifications using OpenAPI
- Make D routes for season and rewrite tv_show for recursive delete
- Make U routes for movie,tv_show,season,episode
- Make some basic queries and mutations for videos and rating and start work on the web_transport