use thiserror::Error;
#[derive(Error, Debug)]
pub enum APIError {
    #[error("Reqwest Error")]
    InternalServerError(String)
}

impl actix_web::error::ResponseError for APIError {
    fn status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let mut resp = actix_web::HttpResponse::new(self.status_code());
        let buffer: String;
        match self {
            APIError::InternalServerError(content) => {buffer = content.clone() }
        }
        resp.headers_mut().insert(
            reqwest::header::CONTENT_TYPE,
            actix_web::http::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        resp.set_body(actix_web::dev::Body::from(buffer))
    }
}

impl std::convert::From<reqwest::Error> for APIError {
    fn from(input: reqwest::Error) -> Self {
        APIError::InternalServerError(input.to_string())
    }
}