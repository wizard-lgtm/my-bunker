use actix_web::{get, http, post, web, HttpResponse, Responder};

use crate::{utils::password::{hash_password}, AppState, LoginPostData};

#[post("/login")]
pub async fn login(user: web::Json<LoginPostData>, data: web::Data<AppState>) -> impl Responder {
    println!("Login attempt for user: {}", user.username);
    if user.username.is_empty() || user.password.is_empty() {
        return HttpResponse::BadRequest().body("Username and password cannot be empty");
    }
    hash_password(&user.password);
    let user_repo = &data.db.user_repo;
    match user_repo.get_user(&user.username).await {
        Ok(Some(db_user)) => {
            let hashed_user_password = hash_password(&user.password);
            println!("db password: {}", db_user.password);
            println!("Provided password: {}", hashed_user_password);

            // Verify the password
            if db_user.password == hashed_user_password {
                println!("User {} logged in successfully", user.username);

                // Generate JWT token
                match crate::utils::jwt::generate_jwt(&db_user.username) {
                    Ok(token) => {
                        println!("JWT token generated successfully for user: {}", db_user.username);

                        return HttpResponse::Ok().body(token);
                    }
                    Err(e) => {
                        println!("Error generating JWT token: {}", e);
                        return HttpResponse::InternalServerError().body("Internal server error");
                    }
                }
            } else {
                println!("Invalid password for user: {}", user.username);
                return HttpResponse::Unauthorized().body("Invalid username or password");
            }
        }
        Ok(None) => {
            return HttpResponse::NotFound().body("User not found");
        }
        Err(e) => {
            println!("Error fetching user: {}", e);
            return HttpResponse::InternalServerError().body("Internal server error");
        }

    }
}

#[get("/login")]
pub async fn login_form(data: web::Data<AppState>) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("title", "Login Page");

    match data.tera.render("login.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html")
            .body(html),
        Err(e) => {
            println!("Template error: {}", e);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}