use crate::syllables::Syllables;
use axum::extract::State;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Username {
    pub values: Vec<String>,
}

pub async fn username(State(syllables): State<Syllables>) -> Json<Username> {
    let mut usernames = Vec::new();
    for _ in 0..10 {
        usernames.push(syllables.generate_username(7));
    }
    Json(Username { values: usernames })
}

#[derive(Serialize)]
pub struct Fullname {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct Names {
    pub values: Vec<Fullname>,
}

pub async fn names(State(syllables): State<Syllables>) -> Json<Names> {
    let mut names = Vec::new();
    for _ in 0..10 {
        names.push(Fullname {
            first_name: syllables.generate_name(3, 7),
            last_name: syllables.generate_name(3, 7),
        });
    }
    Json(Names { values: names })
}
