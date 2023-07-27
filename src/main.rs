use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
#[macro_use]
extern crate log;

use env_logger::{Builder, Target};
use std::env;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new().service(web::scope("/user").configure(controller::user::scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
