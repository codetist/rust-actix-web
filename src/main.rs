mod models;
mod handlers;

use actix_web::{App, HttpServer, web};
use actix_web_static_files::ResourceFiles;
use crate::handlers::handlers::*;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn register_app_services(config: &mut web::ServiceConfig) {
    let generated = generate();

    let post_scope = web::scope("/api")
        .service(get_posts_handler)
        .service(get_post_handler)
        .service(post_echo_handler)
        .service(get_teapot_handler);

    config.service(post_scope)
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
mod app_integration_tests {
    use super::*;

    use actix_web::{http::header::ContentType, test, App};
    use actix_web::test::TestRequest;
    use actix_web::web::Bytes;
    use actix_http::Request;
    use actix_web::http::StatusCode;
    use crate::models::post::Post;

    fn build_get_request(path: &str, content_type: ContentType) -> Request {
        TestRequest::get()
            .uri(path)
            .insert_header(content_type)
            .to_request()
    }

    fn build_post_request(path: &str, content_type: ContentType, payload: Option<Post>) -> Request {
        let request = TestRequest::post()
            .uri(path)
            .insert_header(content_type);

        if payload.is_some() {
            return request.set_json(payload.unwrap())
                .to_request();
        }

        request.to_request()
    }

    #[actix_web::test]
    async fn integration_test_get_greeting_returns_static_content_and_post_id_from_parameter() {
        // given
        let post_id = 23;
        let expected_body = Post {
            post_id,
            content: "Greetings...".to_string(),
        };

        // when
        let app = test::init_service(App::new().configure(register_app_services)).await;
        let req = build_get_request(format!("{}{}", "/api/posts/", post_id.to_string()).as_str(), ContentType::json());
        let resp = test::call_service(&app, req).await;
        let status = resp.status();
        let body: Post = test::read_body_json(resp).await;

        // then
        assert_eq!(status, StatusCode::OK);
        assert_eq!(expected_body, body);
    }

    #[actix_web::test]
    async fn integration_test_post_echo_returns_the_posted_value() {
        // given
        let post = Post {
            post_id: 12,
            content: "Some text".to_string(),
        };
        let expected_post = post.clone();

        // when
        let app = test::init_service(App::new().configure(register_app_services)).await;
        let req = build_post_request("/api/echo", ContentType::json(), Some(post));
        let resp = test::call_service(&app, req).await;
        let status = resp.status();
        let body: Post = test::read_body_json(resp).await;

        // then
        assert_eq!(status, StatusCode::OK);
        assert_eq!(expected_post, body)
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