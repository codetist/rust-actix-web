use actix_web::{get, post, App, HttpResponse, Responder, HttpServer, web};
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[get("/api")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/api/echo")]
async fn echo(request_body: String) -> impl Responder {
    let response_string = format!("{} {}", "Hello", request_body);
    HttpResponse::Ok().body(response_string)
}

fn register_app_services(config: &mut web::ServiceConfig) {
    let generated = generate();
    config
        .service(greet)
        .service(echo)
        .service(ResourceFiles::new("/", generated));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(register_app_services)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    use actix_web::{http::header::ContentType, test, App};
    use actix_web::http::StatusCode;
    use actix_web::test::TestRequest;
    use actix_web::web::Bytes;
    use actix_http::Request;

    fn build_get_request(path: &str, content_type: ContentType) -> Request {
        TestRequest::get()
            .uri(path)
            .insert_header(content_type)
            .to_request()
    }

    fn build_post_request(path: &str, content_type: ContentType, payload: Bytes) -> Request {
        let request = TestRequest::post()
            .uri(path)
            .insert_header(content_type);

        if !payload.is_empty() {
            return request.set_payload(payload)
                .to_request();
        }

        request.to_request()
    }

    #[actix_web::test]
    async fn integration_test_get_greeting() {
        let app = test::init_service(App::new().configure(register_app_services)).await;
        let req = build_get_request("/api", ContentType::plaintext());

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello World!"))
    }

    #[actix_web::test]
    async fn integration_test_post_echo() {
        let app = test::init_service(App::new().configure(register_app_services)).await;
        let req = build_post_request("/api/echo", ContentType::plaintext(), Bytes::from_static(b"User"));

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello User"))
    }

    #[actix_web::test]
    async fn integration_test_get_static_index_html() {
        let app = test::init_service(App::new().configure(register_app_services)).await;
        let req = build_get_request("/", ContentType::plaintext());

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Meine Index"))
    }
}