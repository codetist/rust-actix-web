use actix_web::web;
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