use crate::{models::user_model::User, repository::mongodb_repos::MongoRepo};
use actix_web::{get, post, HttpResponse, Responder, HttpRequest, HttpMessage, web::{ServiceConfig, Data, Json}};
use actix_identity::Identity;
use actix_session::storage::RedisActorSessionStore;

pub fn config_user_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_user);
}

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        username: new_user.username.to_owned(),
        mail: new_user.mail.to_owned(),
        password: new_user.password.to_owned(),
        salt: new_user.salt.to_owned()
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/login")]
pub async fn login_user(db: Data<MongoRepo>, request: HttpRequest, user: Json<User>) -> HttpResponse {
    let user_db = db.get_user(user.username.to_owned().unwrap()).await.unwrap().unwrap();

    if user_db.password != user.password {
        return HttpResponse::Unauthorized().body("Wrong password");
    }

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), user_db.id.unwrap().to_hex()).unwrap();

    HttpResponse::Ok().body("Logged in")
}