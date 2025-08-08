use mongodb::{bson::doc, bson::oid::ObjectId, Collection, Database};
use serde::{Deserialize, Serialize};
use crate::utils::password::{hash_password};

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
        self.get_all_users().await?;
        println!("Fetching user with username: {}", username);
        self.collection.find_one(doc!{"username": username}).await
    }
    pub async fn get_all_users(&self) -> mongodb::error::Result<()> {
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut users = Vec::new();
        cursor.current().iter().for_each(|user| {
            if let Ok(user) = user {
                users.push(user);
            }
        });
        println!("Fetching all users from the database");
        println!("Users found: {}", users.len());
        if users.is_empty() {
            println!("No users found in the database.");    
        } else {
            println!("Users found: {:?}", users);
        }
        Ok(())
    }



    pub async fn set_user(&self, username: &str, password: &str) -> mongodb::error::Result<()> {
        let hashed = hash_password(password);
        let user = User {
            id: None,
            username: username.to_string(),
            password: hashed,
        };
        if let Some(existing_user) = self.collection.find_one(doc! { "username": &user.username }).await? {
            // If user exists, update the password
            self.collection
                .update_one(
                    doc! { "_id": existing_user.id },
                    doc! { "$set": { "password": &user.password } },
                )
                .await?;
            return Ok(());
        }
        self.collection.insert_one(user).await?;
        Ok(())
    }
}