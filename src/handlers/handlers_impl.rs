use actix_http::StatusCode;
use actix_web::{HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use crate::models::post::Post;
use crate::models::error::ApiError;

pub async fn get_post(post_id: web::Path<usize>) -> Post {
    Post {
        post_id: post_id.into_inner(),
        content: "Greetings...".parse().unwrap(),
    }
}

pub async fn get_teapot() -> Result<Post, ApiError> {
    Err(ApiError {
        response_code: StatusCode::IM_A_TEAPOT.as_u16(),
        message: String::from("I am a teapot")
    })
}

pub async fn post_echo(post: web::Json<Post>) -> Post {
    Post {
        post_id: post.post_id,
        content: String::from(&post.content)
    }
}

pub async fn get_posts() -> impl Responder {
    let mut posts : Vec<Post> = Vec::new();
    posts.push(Post{
        post_id: 1,
        content: "Post 1".parse().unwrap(),
    });
    posts.push(Post{
        post_id: 2,
        content: "Post 2".parse().unwrap(),
    });

    let body = serde_json::to_string(&posts).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}


#[cfg(test)]
mod tests {
    use std::slice::SliceIndex;
    use actix_web::body::to_bytes;
    use actix_web::http::{header, StatusCode};
    use actix_web::Responder;
    use actix_web::test::TestRequest;
    use actix_web::web::Json;
    use super::*;

    #[actix_web::test]
    async fn get_post_returns_greeting_with_correct_headers_and_post_id() {

        // given
        let post_id : usize = 12;

        let post = Post {
            post_id,
            content: "Greetings...".to_string(),
        };

        let post_id_path : web::Path<usize> = web::Path::from(post_id);
        let request = TestRequest::default().to_http_request();

        // when
        let result = get_post(post_id_path).await;
        let response = result.respond_to(&request);
        let headers = response.headers();

        // then
        assert_eq!(StatusCode::OK, response.status());
        assert!(headers.contains_key(header::CONTENT_TYPE));
        assert_eq!("application/json", headers.get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let body = response.into_body();
        let body_bytes = match to_bytes(body).await {
            Ok(x) => x,
            _ => panic!(),
        };

        let json_string = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(json_string, "{\"post_id\":12,\"content\":\"Greetings...\"}");

        let current_post : Post = serde_json::from_str(&json_string).unwrap();
        assert_eq!(post, current_post);
    }

    #[actix_web::test]
    async fn get_teapot_handler_returns_http_code_418() {

        // when
        let request = TestRequest::default().to_http_request();
        let result = get_teapot().await;
        let response = result.respond_to(&request);
        let headers = response.headers();

        // then
        assert_eq!(StatusCode::IM_A_TEAPOT, response.status());
        assert!(headers.contains_key(header::CONTENT_TYPE));
        assert_eq!("application/json", headers.get(header::CONTENT_TYPE).unwrap().to_str().unwrap());
    }

    #[actix_web::test]
    async fn post_echo_returns_the_posted_content() {

        // given
        let post = Post{
            post_id: 4711,
            content: "Agent 4711 is gone".to_string()
        };
        let expected_post = post.clone();

        let post_json = Json(post);

        // when
        let request = TestRequest::default().to_http_request();
        let result = post_echo(post_json).await;
        let response = result.respond_to(&request);
        let headers = response.headers();

        // then
        assert_eq!(StatusCode::OK, response.status());
        assert!(headers.contains_key(header::CONTENT_TYPE));
        assert_eq!("application/json", headers.get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let body = response.into_body();
        let body_bytes = match to_bytes(body).await {
            Ok(x) => x,
            _ => panic!(),
        };

        let body_json = String::from_utf8(body_bytes.to_vec()).unwrap();
        let current_post : Post = serde_json::from_str(&body_json).unwrap();

        assert_eq!(expected_post, current_post);
    }

}
