use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;

pub fn generate_jwt(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut claims = BTreeMap::new();
    claims.insert("sub", username);
    let binding = (chrono::Utc::now() + chrono::Duration::days(7)).timestamp().to_string();
    claims.insert("exp", &binding);
     
    let secret_key = env::var("JWT_SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes())?;
    
    let token = claims.sign_with_key(&key)?;
    Ok(token)
}

pub fn verify_jwt(token: &str) -> bool {
    let secret_key = match env::var("JWT_SECRET") {
        Ok(key) => key,
        Err(_) => return false,
    };
    
    let key = match Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()) {
        Ok(k) => k,
        Err(_) => return false,
    };
    
    let claims: BTreeMap<String, String> = match token.verify_with_key(&key) {
        Ok(c) => c,
        Err(_) => return false,
    };
    
    // Check if expired
    if let Some(exp_str) = claims.get("exp") {
        if let Ok(exp_timestamp) = exp_str.parse::<i64>() {
            return chrono::Utc::now().timestamp() <= exp_timestamp;
        }
    }
    
    true
}