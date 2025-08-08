use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use core::hash;
use std::env;

pub fn hash_password(password: &str) -> String {
    let salt_key = match env::var("SALT_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("SALT_KEY not set in environment variables. Panicking.");
            std::process::exit(1);
        }
    };
    let salt = SaltString::from_b64(&salt_key).unwrap();
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    println!("Password hashed successfully. {}", hashed_password);
    return hashed_password;
}
