mod config;
mod db;
mod handlers;
mod models;

use crate::handlers::*;
use actix_web::{web, App, HttpServer};
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
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/projects{_:/?}", web::get().to(get_projects))
            .route("/projects/{project_id}", web::get().to(get_project))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

// integration tests
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
    async fn test_get_status_err() {
        let mut app = test::init_service(App::new().route("/weird", web::get().to(status))).await;
        let req = test::TestRequest::with_header("content-type", "text/json").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
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
    async fn test_get_projects_err() {
        dotenv().ok();

        let config = crate::config::Config::from_env().unwrap();
        let test_pool = config.pg.create_pool(NoTls).unwrap();
        let mut app = test::init_service(
            App::new()
                .data(test_pool.clone())
                .route("/projects/", web::get().to(get_projects)),
        )
        .await;
        let req = test::TestRequest::with_header("content-type", "text/json").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
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

    #[actix_rt::test]
    async fn test_get_project_err() {
        let mut app =
            test::init_service(App::new().route("/projects/1000", web::get().to(get_projects)))
                .await;
        let req = test::TestRequest::with_header("content-type", "text/json").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
