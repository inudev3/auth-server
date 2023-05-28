#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::model::PgPool;


pub mod model;
pub mod utils;
pub mod errors;
pub mod invitation_handler;
pub mod schema;
pub mod email_service;
pub mod register_handler;
pub mod auth_handler;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG", "simple-auth-server=debug, actix_web=info,actix_server=info"
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: PgPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    // let utils = std::env::var("SECRET_KEY").unwrap_or_else("NOKEY".to_string());
    HttpServer::new(move||{
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(time::Duration::days(1))
                    .secure(false)
            ))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(web::resource("/invitation")
                        .route(web::post().to(invitation_handler::post_invitation))
                    ).service(web::resource("/register/{invitation_id}")
                    .route(web::post().to(register_handler::register_user))
                )
            )
    }).bind("127.0.0.1:8080")?.run().await
}


