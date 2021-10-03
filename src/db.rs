use crate::models::{ProjectDetails, ProjectList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_projects(client: &Client) -> Result<Vec<ProjectList>, io::Error> {
    let sql = client.prepare("select * from project_list").await.unwrap();

    let projects = client
        .query(&sql, &[])
        .await
        .expect("Error getting project list")
        .iter()
        .map(|row| ProjectList::from_row_ref(row).unwrap())
        .collect::<Vec<ProjectList>>();

    Ok(projects)
}

pub async fn get_project(
    client: &Client,
    project_id: i32,
) -> Result<Vec<ProjectDetails>, io::Error> {
    let sql = client
        .prepare("select * from project_details where project_id = $1 order by id")
        .await
        .unwrap();

    let project = client
        .query(&sql, &[&project_id])
        .await
        .expect("Error getting project")
        .iter()
        .map(|row| ProjectDetails::from_row_ref(row).unwrap())
        .collect::<Vec<ProjectDetails>>();

    Ok(project)
}

#[test]
fn test_get_projects() {
    assert_eq!(true, true);
}
