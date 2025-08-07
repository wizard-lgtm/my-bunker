use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;
use dotenv::dotenv;
mod db;
use db::connect::connect_to_db;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
    println!("Connected to database: {:?}", db.name());

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}
