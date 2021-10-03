mod config;
mod models;
mod handlers;
mod db;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;
use crate:: handlers::*;

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
