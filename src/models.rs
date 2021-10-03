use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="project_list")]
pub struct ProjectList {
    pub id: i32,
    pub title: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="project_details")]
pub struct ProjectDetails {
    pub id: i32,
    pub title: String,
    pub in_progress: bool,
    pub list_id: i32
}
