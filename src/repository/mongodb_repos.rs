use mongodb::{
    bson::{extjson::de::Error, doc},
    results::InsertOneResult,
    Client, Collection,
};
use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init(uri: String) -> Self {
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            username: new_user.username,
            mail: new_user.mail,
            password: new_user.password,
            salt: new_user.salt
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, username: String) -> Result<Option<User>, Error> {
        let user = self
            .col
            .find_one(doc! { "username": username }, None)
            .await
            .ok()
            .expect("Error finding user");
        Ok(user)
    }
}