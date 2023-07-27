use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
struct User {
    id: String,
    name: String,
}

async fn create_user(info: web::Json<User>) -> Result<impl Responder> {
    println!("create users{}", info.id);
    Ok(web::Json(info))
}
async fn get_all_user() -> Result<impl Responder> {
    println!("hello");
    let mut vec = Vec::new();
    vec.push(User {
        id: String::from("1"),
        name: String::from("sdfsdfhjd"),
    });
    vec.push(User {
        id: String::from("2"),
        name: String::from("2sdfsdfhjd"),
    });
    Ok(web::Json(vec))
}
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_user))
            .route(web::get().to(get_all_user)),
    );
}
