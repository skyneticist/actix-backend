mod auth;
mod config;
mod db;
mod errors;
mod handlers;
mod models;

// use crate::models::{VideoCategory, VideoCategory::*, YoutubeVid};

use crate::handlers::*;
use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, http, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!(
        "Server starting @ http://{}:{}/",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(validator);
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".localhost:3000"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            // .wrap(auth)
            .wrap(cors)
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/projects", web::get().to(get_projects))
            .route("/projects/{project_id}", web::get().to(get_project))
            .route("/projects", web::post().to(add_project))
            .route("/books", web::get().to(get_books))
            .route("/books/{book_id}", web::get().to(get_book))
            .route("/books", web::post().to(add_book))
            .route("/albums", web::get().to(get_albums))
            .route("/albums/{album_id}", web::get().to(get_album))
            .route("/albums", web::post().to(add_album))
            .route("/videos", web::get().to(get_videos))
            .route("/videos/{video_id}", web::get().to(get_video))
            .route("/videos", web::post().to(add_video))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

// async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     let config = req
//         .app_data::<Config>()
//         .map(|data| data.get_ref().clone())
//         .unwrap_or_else(Default::default);
//     match auth::validate_token(credentials.token()) {
//         Ok(res) => {
//             if res == true {
//                 Ok(req)
//             } else {
//                 Err(AuthenticationError::from(config).into())
//             }
//         }
//         Err(_) => Err(AuthenticationError::from(config).into()),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_get_status_ok() {
        dotenv().ok();

        let config = crate::config::Config::from_env().unwrap();
        let test_pool = config.pg.create_pool(NoTls).unwrap();
        let mut app =
            test::init_service(App::new().data(test_pool).route("/", web::get().to(status))).await;
        let req = test::TestRequest::with_header("content-type", "text/json").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_projects_ok() {
        let mut app =
            test::init_service(App::new().route("/projects{_:/?}", web::get().to(get_projects)))
                .await;
        let req = test::TestRequest::with_header("content-type", "text/json").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_project_ok() {
        let mut app = test::init_service(
            App::new().route("/projects/{project_id}", web::get().to(get_projects)),
        )
        .await;
        let req = test::TestRequest::with_header("content-type", "text/json").to_request();
        test::TestRequest::default()
            .param("project_id", "1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
