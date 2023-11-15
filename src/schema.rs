// @generated automatically by Diesel CLI.

diesel::table! {
    episodes (id) {
        id -> Int4,
        episode_name -> Varchar,
        director -> Varchar,
        summary -> Text,
        seasons_id -> Int4,
    }
}

diesel::table! {
    movies (id) {
        id -> Int4,
        title -> Varchar,
        director -> Varchar,
        rating -> Float8,
        summary -> Text,
        duration -> Float8,
        video_link -> Varchar,
        picture_link -> Varchar,
    }
}

diesel::table! {
    seasons (id) {
        id -> Int4,
        season_number -> Int4,
        summary -> Text,
        tv_shows_id -> Int4,
    }
}

diesel::table! {
    tv_shows (id) {
        id -> Int4,
        title -> Varchar,
        director -> Varchar,
        rating -> Float8,
        summary -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_name -> Varchar,
        user_email -> Varchar,
        user_password -> Varchar,
    }
}

diesel::joinable!(episodes -> seasons (seasons_id));
diesel::joinable!(seasons -> tv_shows (tv_shows_id));

diesel::allow_tables_to_appear_in_same_query!(
    episodes,
    movies,
    seasons,
    tv_shows,
    users,
);
