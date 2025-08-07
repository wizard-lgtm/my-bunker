use mongodb::{Client, Database};
pub async fn connect_to_db() -> Result<Database, mongodb::error::Error> {
    let uri = match std::env::var("MONGODB_URI"){
        Ok(uri) => uri,
        Err(_) => {
            println!("MONGODB_URI not set in environment variables. Panicking.");
            std::process::exit(1);
        }
    };
    let db_name = match std::env::var("DATABASE_NAME") {
        Ok(name) => name,
        Err(_) => {
            println!("DATABASE_NAME not set in environment variables. Using default 'bunker'.");
            "bunker".to_string()
        }
    };
    let client = Client::with_uri_str(uri).await?;
    let db = client.database(&db_name);
    Ok(db)
}