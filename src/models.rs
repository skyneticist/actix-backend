use derive_more::{Add, Display, From};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize)]
pub enum ServerStatus {
    Up,
    Down,
    Unhealthy,
}

#[derive(Serialize)]
pub struct Status {
    pub status: ServerStatus,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "project_list")]
pub struct ProjectList {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "project_details")]
pub struct ProjectDetails {
    pub id: i32,
    pub title: String,
    pub in_progress: bool,
    pub project_id: i32,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "books")]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub release_date: String,
    pub publisher: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "albums")]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub release_date: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "videos")]
pub struct YoutubeVid {
    pub id: i32,
    pub title: String,
    pub channel: String,
    pub category: String,
    pub rank: i32,
}

#[derive(Debug, Deserialize, Serialize, Add, Display, From, PartialEq)]
pub enum VideoCategory {
    Electronics,
    Comedy,
    Goat,
    Food,
    Science,
    Engineering,
    Programming,
    Robotics,
    Creepy,
    FeelGood,
    Dark,
}
