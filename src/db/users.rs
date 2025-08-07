use mongodb::{bson::doc, bson::oid::ObjectId, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

pub struct UserRepo {
    collection: Collection<User>,
}

impl UserRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<User>("users"),
        }
    }

    pub async fn get_user(&self, username: &str) -> mongodb::error::Result<Option<User>> {
        self.collection.find_one(doc!{username: username}).await
    }

    pub async fn verify_password(
        &self,
        username: &str,
        password: &str,
    ) -> mongodb::error::Result<bool> {
        if let Some(user) = self.get_user(username).await? {
            Ok(user.password == password)
        } else {
            Ok(false)
        }
    }

    pub async fn set_user(&self, username: &str, password: &str) -> mongodb::error::Result<()> {
        let user = User {
            id: None,
            username: username.to_string(),
            password: password.to_string(),
        };
        if let Some(existing_user) = self.collection.find_one(doc! {username: user.username}).await? {
            // If user exists, update the password
            self.collection
                .update_one(
                    doc! { "_id": existing_user.id },
                    doc! { "$set": { "password": user.password } },
                )
                .await?;
            return Ok(());
        }
        Ok(())
    }
}