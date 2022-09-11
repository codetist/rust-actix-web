
use actix_web::{get, post, HttpResponse, Responder, web};
use crate::models::post::Post;

#[get("/api/post/{post_id}")]
async fn greet_handler(post_id: web::Path<usize>) -> impl Responder {
    greet(post_id).await
}

async fn greet(post_id: web::Path<usize>) -> impl Responder {
    let post = Post {
        post_id: post_id.into_inner(),
        content: "Greetings...".parse().unwrap(),
    };
    HttpResponse::Ok().json(post)
}


#[post("/api/echo")]
async fn echo(post: web::Json<Post>) -> impl Responder {
    HttpResponse::Ok().json(post)
}


#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::http::{header, StatusCode};
    use actix_web::test::TestRequest;
    use super::*;

    #[actix_web::test]
    async fn greet_returns_greeting_with_correct_headers_and_post_id() {

        // given
        let post_id : usize = 12;

        let post = Post {
            post_id,
            content: "Greetings...".to_string(),
        };

        let post_id_path : web::Path<usize> = web::Path::from(post_id);
        let request = TestRequest::default().to_http_request();

        // when
        let result = greet(post_id_path).await;
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

}
