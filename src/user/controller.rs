use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, error};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use serde::{Serialize, Deserialize};

use super::{create_user_handler, models::UserLogin};
#[derive(Serialize,Deserialize)]
struct User {
    id: String,
    name: String,
}

async fn create_user(info: web::Json<User>, pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> Result<impl Responder> {
     let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().unwrap();

        create_user_handler::create_user(&mut conn, &info.name)
    })
    .await?.map_err(error::ErrorInternalServerError)?;
    // map diesel query errors to a 500 error response

    Ok(HttpResponse::Created().json(user))
}

async fn get_all_user() -> Result<impl Responder> {
    println!("hello");
    let mut vec = Vec::new();
    Ok(web::Json(vec))
}
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_user))
            .route(web::get().to(get_all_user)),
    );
}
