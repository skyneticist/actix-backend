use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

/*
    This file contains every Http handler registered with its
    corresponding route in App::new declaration in main.rs.

    Each method (other than status()) instantiates a
    db pool client that is passed to a specific method
    invoked from the db crate.

    Additional parameters are passed for methods that require
    additional input (e.g. POST) with each method using match
    enum branching on the result returned from the db call.

    Each path returns a HttpResponse, satisfying the Responder
    return type. Ok() match results in HttpResponse::Ok() and
    Err() match results in HttpResponse::InternalServerError()
*/

// status - Responds with status of server
pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: ServerStatus::Up,
    })
}

pub async fn get_books(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_books(&client).await;

    match result {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_book(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_book(&client, path.0).await;

    match result {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn add_book(db_pool: web::Data<Pool>, book_json: web::Json<Book>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::add_book(&client, book_json).await;

    match result {
        Ok(book) => HttpResponse::Ok().json::<&Book>(&book),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

// projects
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

pub async fn add_project(
    db_pool: web::Data<Pool>,
    project_json: web::Json<ProjectDetails>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::add_project(&client, project_json).await;

    match result {
        Ok(project) => HttpResponse::Ok().json::<&ProjectDetails>(&project),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

// albums
pub async fn get_albums(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::get_albums(&client).await;

    match result {
        Ok(album) => HttpResponse::Ok().json(album),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_album(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_album(&client, path.0).await;

    match result {
        Ok(album) => HttpResponse::Ok().json(album),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn add_album(db_pool: web::Data<Pool>, album_json: web::Json<Album>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::add_album(&client, album_json).await;

    match result {
        Ok(album) => HttpResponse::Ok().json::<&Album>(&album),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

// videos
pub async fn get_videos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::get_videos(&client).await;

    match result {
        Ok(video) => HttpResponse::Ok().json(video),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_video(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::get_video(&client, path.0).await;

    match result {
        Ok(yt_vid) => HttpResponse::Ok().json(yt_vid),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn add_video(
    db_pool: web::Data<Pool>,
    video_json: web::Json<YoutubeVid>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::add_video(&client, video_json).await;

    match result {
        Ok(yt_vid) => HttpResponse::Ok().json::<&YoutubeVid>(&yt_vid),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
