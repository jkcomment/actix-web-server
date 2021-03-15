use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    message: String,
}

#[get("/")]
async fn get_greeting() -> &'static str {
    "Hello world!\r\n"
}

#[post("/hello")]
async fn post_greeting(req: web::Json<Request>) -> impl Responder {
    let str = format!("hello, {}", req.message.to_string());
    HttpResponse::Ok().body(str.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(get_greeting)
            .service(post_greeting)
    )
    .bind("127.0.0.1:3000")?
    .run()
    .await
}