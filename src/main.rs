mod api;
mod models;
mod repository;

//modify imports below
use actix_web::{web::{Data, scope}, App, HttpServer};
use log::info;
use repository::mongodb_repos::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");

    info!("Starting server at http://localhost:8080");

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(scope("/api")
                .configure(api::user_api::config_user_routes))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}