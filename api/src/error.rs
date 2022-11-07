use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};

use thiserror::Error as ThisError;
use tracing::log::error;
use validator::ValidationErrors;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("internal server error")]
    InternalServerError(String),

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
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_, _) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut res = HttpResponseBuilder::new(self.status_code());

        if let Error::InternalServerError(e) = self {
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
        Error::InternalServerError(e.to_string())
    }
}

impl From<actix::MailboxError> for Error {
    fn from(e: actix::MailboxError) -> Self {
        Error::InternalServerError(e.to_string())
    }
}

impl From<pulsar::Error> for Error {
    fn from(e: pulsar::Error) -> Self {
        Error::InternalServerError(e.to_string())
    }
}

impl From<evento::Error> for Error {
    fn from(e: evento::Error) -> Self {
        Error::InternalServerError(e.to_string())
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(e: mongodb::error::Error) -> Self {
        Error::InternalServerError(e.to_string())
    }
}
