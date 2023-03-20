use std::fmt;

use ntex::{
    http::StatusCode,
    web::{HttpResponse, WebResponseError},
};

#[derive(Debug, Clone)]
pub enum CustomError {
    NotFound(String),
    BadRequest(String),
    AuthFailed(String),
    InternalServerErrror(String),
}

impl WebResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::AuthFailed(_) => StatusCode::UNAUTHORIZED,
            Self::InternalServerErrror(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self, _: &ntex::web::HttpRequest) -> HttpResponse {
        HttpResponse::new(self.status_code()).set_body(
            match self {
                Self::NotFound(e) => e,
                Self::BadRequest(e) => e,
                Self::AuthFailed(e) => e,
                Self::InternalServerErrror(e) => e,
            }
            .into(),
        )
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(e) => write!(f, "{e}"),
            Self::BadRequest(e) => write!(f, "{e}"),
            Self::AuthFailed(e) => write!(f,"{e}"),
            Self::InternalServerErrror(e)=> write!(f, "{e}"),
            // CustomError::NotFound(e) => write!(f, "{e}"),
            // Self::BadRequest(e) => write!(f, "{e}"),
            // CustomError::InternalServerErrror(e) => write!(f, "{e}"),
        }
    }
}

impl From<sqlx::Error> for CustomError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound("找不到对应的数据".into()),
            _ => Self::InternalServerErrror("Internal Svr Err, contact aa@bb.com".into()),
        }
    }
}
