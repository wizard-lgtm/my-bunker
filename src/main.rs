use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;
use dotenv::dotenv;
use std::sync::Arc;

pub mod db;
use db::{connect::connect_to_db, Db};
use crate::utils::password::{verify_password as verify_password_hash};
pub mod utils;

struct AppState {
    db: Arc<Db>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(serde::Deserialize)]
struct LoginPostData {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(user: web::Json<LoginPostData>, data: web::Data<AppState>) -> impl Responder {
    let user_repo = &data.db.user_repo;
    match user_repo.get_user(&user.username).await {
        Ok(Some(db_user)) => {
            if verify_password_hash(&user.password, &db_user.password){
                return HttpResponse::Ok().body("Login successful");
            } else {
                return HttpResponse::Unauthorized().body("Invalid password");
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

async fn manual_hello() -> impl Responder {
    let tera = match Tera::new("./src/views/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let context = tera::Context::new();
    // Use the template name relative to the glob root
    let html = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = connect_to_db().await.expect("Failed to connect to database");
    println!("Connected to database");

    let app_state = web::Data::new(AppState {
        db: Arc::new(Db {
            user_repo: db.user_repo,
        }),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(echo)
            .service(login)
            .route("/hey", web::get().to(manual_hello))
            // .route("/users", web::get().to(get_users)) // Remove or implement get_users
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}