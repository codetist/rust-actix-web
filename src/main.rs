use actix_web::{get, post, App, HttpResponse, Responder, HttpServer};
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[get("/api")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/api/echo")]
async fn echo(request_body: String) -> impl Responder {
    let response_string = format!("{} {}", "Hello", request_body);
    HttpResponse::Ok().body(response_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(hello)
            .service(echo)
            .service(ResourceFiles::new("/", generated))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
