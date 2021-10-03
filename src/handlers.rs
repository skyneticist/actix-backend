use crate::models::Status;
use crate::db;
use deadpool_postgres::{Pool, Client};
use actix_web::{web, HttpResponse};

pub async fn status() -> HttpResponse {
    web::HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

pub async fn get_projects(db_pool: web::Data<Pool>) -> HttpResponse {
    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_projects(&client).await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_project(db_pool: web::Data<Pool>, path: web::Path<i32>) -> HttpResponse {
    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");
    
    let result = db::get_project(&client, path.0).await;

    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
