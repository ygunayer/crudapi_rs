use std::fmt::{Display, Formatter};
use actix_web::http::StatusCode;
use actix_web::{ResponseError, HttpResponse};
use crate::db::error::DbError;
use log;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    ServerFailure,
    BusinessValidation,
    DataNotFound
}

impl Into<StatusCode> for ErrorKind {
    fn into(self) -> StatusCode {
        match self {
            Self::ServerFailure => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BusinessValidation => StatusCode::UNPROCESSABLE_ENTITY,
            Self::DataNotFound => StatusCode::NOT_FOUND
        }
    }
}

#[derive(Debug)]
pub struct CoreApiError {
    kind: ErrorKind,
    message: String
}

impl CoreApiError {
    pub fn not_found(key: &str) -> CoreApiError {
        CoreApiError { kind: ErrorKind::DataNotFound, message: key.into() } // TODO message
    }

    pub fn business_validation(key: &str) -> CoreApiError {
        CoreApiError { kind: ErrorKind::BusinessValidation, message: key.into() } // TODO message
    }
}

impl From<DbError> for CoreApiError {
    fn from(err: DbError) -> Self {
        CoreApiError {
            kind: ErrorKind::ServerFailure,
            message: format!("An error has occurred: {:?}", err)
        }
    }
}

impl Display for CoreApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.kind, self.message)
    }
}

impl ResponseError for CoreApiError {
    fn status_code(&self) -> StatusCode {
        self.kind.into()
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        log::error!("An error has occurred: {:?} with message: {}", &self.kind, &self.message);
        match self.kind {
            ErrorKind::DataNotFound => todo!("not found"), // TODO 404
            ErrorKind::BusinessValidation => todo!("business validation"), // TODO 400 or 422
            ErrorKind::ServerFailure => todo!("server failure") // TODO 500
        }
    }
}
