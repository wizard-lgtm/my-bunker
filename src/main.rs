use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use tera::Tera;
use dotenv::dotenv;
use std::{env, sync::Arc};
mod routes;
use routes::user::{login, login_form};


pub mod db;
use db::{connect::connect_to_db, Db};
pub mod utils;

struct AppState {
    db: Arc<Db>,
    tera: Tera
}


#[derive(serde::Deserialize)]
struct LoginPostData {
    username: String,
    password: String,
}

async fn home(data: web::Data<AppState>) -> impl Responder {
    let context = tera::Context::new();
    match data.tera.render("index.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html")
            .body(html),
        Err(e) => {
            println!("Template error: {}", e);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = connect_to_db().await.expect("Failed to connect to database");
    println!("Connected to database");

    let tera = match Tera::new("src/views/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    let app_state = web::Data::new(AppState {
        db: Arc::new(Db {
            user_repo: db.user_repo,
        }),
        tera: tera,
    });

    let port:u16 = env::var("PORT").unwrap_or_else(|_| "8001".to_string()).parse().unwrap();
    println!("Server running on port {}", port);

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/public", "./public").show_files_listing())
            .app_data(app_state.clone())

            .service(login)
            .service(login_form)
            .route("/", web::get().to(home))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}