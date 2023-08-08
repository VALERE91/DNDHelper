mod api;
mod models;
mod repository;
mod config;

use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::{web::{Data, scope}, App, HttpServer, cookie::Key};
use actix_identity::IdentityMiddleware;

use log::info;
use repository::mongodb_repos::MongoRepo;

use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();

    log4rs::init_file(config.log_config_file, Default::default())
                .expect("Log config file not found.");

    let db = MongoRepo::init(config.db_uri).await;
    let db_data = Data::new(db);

    let secret_key = Key::generate();

    info!("Starting server at http://${0}:${1}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
        .wrap(actix_web::middleware::Logger::default())
        .wrap(actix_web::middleware::Compress::default())
        .wrap(IdentityMiddleware::default())
        .wrap(SessionMiddleware::new(
            RedisActorSessionStore::new(config.redis_uri.clone()),
            secret_key.clone()
       ))
        .app_data(db_data.clone())
        .service(scope("/api")
            .configure(api::user_api::config_user_routes))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}