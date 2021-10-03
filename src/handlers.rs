use crate::db;
use crate::models::Status;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

pub async fn get_projects(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_projects(&client).await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_project(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_project(&client, path.0).await;

    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

// unit tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{http, test};

//     #[actix_rt::test]
//     async fn test_index_ok() {
//         let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
//         let resp = index(req).await;
//         assert_eq!(resp.status(), http::StatusCode::OK);
//     }
// }