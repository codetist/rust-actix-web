use actix_web::{get, post, Responder, web};
use crate::models::post::Post;
use crate::models::error::ApiError;
use crate::handlers::handlers_impl::*;

#[get("/posts/{post_id}")]
async fn get_post_handler(post_id: web::Path<usize>) -> Post {
    get_post(post_id).await
}

#[get("/posts")]
async fn get_posts_handler() -> impl Responder {
    get_posts().await
}

#[post("/echo")]
async fn post_echo_handler(post: web::Json<Post>) -> Post {
    return post_echo(post).await
}

#[get("/teapot")]
async fn get_teapot_handler() -> Result<Post, ApiError> {
    return get_teapot().await
}