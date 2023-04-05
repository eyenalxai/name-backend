use crate::syllables::Syllables;
use axum::extract::State;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Username {
    pub value: String,
}

#[derive(Serialize)]
pub struct Usernames {
    pub values: Vec<Username>,
}

pub async fn username(State(syllables): State<Syllables>) -> Json<Usernames> {
    let mut usernames = Vec::new();
    for _ in 0..10 {
        usernames.push(Username {
            value: syllables.generate_username(7),
        });
    }
    Json(Usernames { values: usernames })
}

#[derive(Serialize)]
pub struct Fullname {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct Fullnames {
    pub values: Vec<Fullname>,
}

pub async fn fullnames(State(syllables): State<Syllables>) -> Json<Fullnames> {
    let mut names = Vec::new();
    for _ in 0..10 {
        names.push(Fullname {
            first_name: syllables.generate_name(3, 7),
            last_name: syllables.generate_name(3, 7),
        });
    }
    Json(Fullnames { values: names })
}
