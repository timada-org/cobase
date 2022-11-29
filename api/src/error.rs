use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};

use thiserror::Error as ThisError;
use tracing::error;
use validator::ValidationErrors;

#[derive(ThisError, Debug, Clone)]
pub enum Error {
    #[error("internal server error")]
    InternalServerErr(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("{0} `{1}` does not exist")]
    NotFound(String, String),
}

impl Error {
    pub fn into_response(self) -> Result<HttpResponse, Self> {
        Err(self)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::InternalServerErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_, _) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut res = HttpResponseBuilder::new(self.status_code());

        if let Error::InternalServerErr(e) = self {
            error!("{}", e);
        }

        res.json(
            serde_json::json!({"code": self.status_code().as_u16(), "message": self.to_string()}),
        )
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        Error::BadRequest(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::InternalServerErr(e.to_string())
    }
}

impl From<actix::MailboxError> for Error {
    fn from(e: actix::MailboxError) -> Self {
        Error::InternalServerErr(e.to_string())
    }
}

impl From<pulsar::Error> for Error {
    fn from(e: pulsar::Error) -> Self {
        Error::InternalServerErr(e.to_string())
    }
}

impl From<evento::Error> for Error {
    fn from(e: evento::Error) -> Self {
        Error::InternalServerErr(e.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::InternalServerErr(e.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(e: uuid::Error) -> Self {
        Error::InternalServerErr(e.to_string())
    }
}
