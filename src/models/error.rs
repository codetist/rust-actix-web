use std::fmt::{Display, Formatter};
use actix_http::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiError {
    pub response_code: u16,
    pub message: String
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.response_code).unwrap()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(body)
    }
}