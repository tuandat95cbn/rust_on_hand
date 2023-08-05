use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;

#[macro_use]
extern crate log;

use env_logger::{Builder, Target};
mod config;
mod user;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    let pool = config::get_connection_pool();
    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/user").configure(user::controller::scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
