use actix_http::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub post_id: usize,
    pub content: String
}

impl From<web::Json<Post>> for Post {
    fn from(post: web::Json<Post>) -> Self {
        Post {
            post_id: post.post_id,
            content: post.content.clone()
        }
    }
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.post_id == other.post_id && self.content == other.content
    }
}

impl Responder for Post {
    type Body = BoxBody;

    #[allow(unused_variables)]
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self)
            .unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
