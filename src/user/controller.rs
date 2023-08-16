use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::{Deserialize, Serialize};

use super::{common::PageRequest, create_user_handler, models::UserLogin, users_handler};
#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

async fn create_user(
    info: web::Json<User>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl Responder> {
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().unwrap();

        create_user_handler::create_user(&mut conn, &info.name)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    // map diesel query errors to a 500 error response

    Ok(HttpResponse::Created().json(user))
}

async fn get_all_user(
    page_request: web::Query<PageRequest>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<impl Responder> {
    let users_paginated = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().unwrap();

        users_handler::users(&mut conn, &page_request.size, &page_request.page)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    // map diesel query errors to a 500 error response

    Ok(HttpResponse::Created().json(users_paginated))
}
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_user))
            .route(web::get().to(get_all_user)),
    );
}
