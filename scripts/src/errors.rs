#![allow(dead_code)]

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

use arcstr::ArcStr;
use serde::Serialize;
use warp::http::StatusCode;

use commons::error::AppError;

/// A simple result that returns a `ServerError` in case of an error.
pub type ServerResult<T> = Result<T, ServerError>;

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// The standard error of the module.
#[derive(Debug, Clone, Serialize)]
pub struct ServerError(AppError);

impl ServerError {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(code: ArcStr) -> Self {
        ServerError(AppError::new(code))
    }

    pub fn new_with_status(status_code: StatusCode, code: ArcStr) -> Self {
        ServerError(AppError::new_with_status(status_code, code))
    }

    // SETTERS ----------------------------------------------------------------

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.0.status_code = Some(status_code);
        self
    }

    pub fn code(mut self, code: ArcStr) -> Self {
        self.0.code = code;
        self
    }

    pub fn param(mut self, param: ArcStr) -> Self {
        self.0.param = Some(param);
        self
    }

    pub fn message(mut self, message: ArcStr) -> Self {
        self.message = Some(message);
        self
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ServerError {}

impl warp::reject::Reject for ServerError {}

impl Deref for ServerError {
    type Target = AppError;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ServerError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ServerError> for AppError {
    fn from(v: ServerError) -> Self {
        v.0
    }
}

impl From<AppError> for ServerError {
    fn from(v: AppError) -> Self {
        ServerError(v)
    }
}
