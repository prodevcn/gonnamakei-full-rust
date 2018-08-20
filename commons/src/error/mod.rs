use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::ErrorKind;

use arangors::ClientError;
use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;
use warp::http::StatusCode;

pub use codes::*;

mod codes;

/// A simple result that returns an `AppError` in case of an error.
pub type AppResult<T> = Result<T, AppError>;

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub status_code: Option<StatusCode>,

    /// Specific error.
    #[serde(rename = "errorCode")]
    pub code: ArcStr,

    /// The param involved in the error if it exists
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub param: Option<ArcStr>,

    /// A human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: Option<ArcStr>,
}

impl AppError {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(code: ArcStr) -> Self {
        AppError {
            status_code: None,
            code,
            param: None,
            message: None,
        }
    }

    pub fn new_with_status(status_code: StatusCode, code: ArcStr) -> Self {
        AppError {
            status_code: Some(status_code),
            code,
            param: None,
            message: None,
        }
    }

    // SETTERS ----------------------------------------------------------------

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn code(mut self, code: ArcStr) -> Self {
        self.code = code;
        self
    }

    pub fn param(mut self, param: ArcStr) -> Self {
        self.param = Some(param);
        self
    }

    pub fn message(mut self, message: ArcStr) -> Self {
        self.message = Some(message);
        self
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AppError {}

impl warp::reject::Reject for AppError {}

impl From<std::io::Error> for AppError {
    fn from(v: std::io::Error) -> Self {
        match v.kind() {
            ErrorKind::NotFound => AppError::new(arcstr::literal!("IO::NotFound")),
            ErrorKind::PermissionDenied => AppError::new(arcstr::literal!("IO::PermissionDenied")),
            ErrorKind::ConnectionRefused => {
                AppError::new(arcstr::literal!("IO::ConnectionRefused"))
            }
            ErrorKind::ConnectionReset => AppError::new(arcstr::literal!("IO::ConnectionReset")),
            ErrorKind::ConnectionAborted => {
                AppError::new(arcstr::literal!("IO::ConnectionAborted"))
            }
            ErrorKind::NotConnected => AppError::new(arcstr::literal!("IO::NotConnected")),
            ErrorKind::AddrInUse => AppError::new(arcstr::literal!("IO::AddrInUse")),
            ErrorKind::AddrNotAvailable => AppError::new(arcstr::literal!("IO::AddrNotAvailable")),
            ErrorKind::BrokenPipe => AppError::new(arcstr::literal!("IO::BrokenPipe")),
            ErrorKind::AlreadyExists => AppError::new(arcstr::literal!("IO::AlreadyExists")),
            ErrorKind::WouldBlock => AppError::new(arcstr::literal!("IO::WouldBlock")),
            ErrorKind::InvalidInput => AppError::new(arcstr::literal!("IO::InvalidInput")),
            ErrorKind::InvalidData => AppError::new(arcstr::literal!("IO::InvalidData")),
            ErrorKind::TimedOut => AppError::new(arcstr::literal!("IO::TimedOut")),
            ErrorKind::WriteZero => AppError::new(arcstr::literal!("IO::WriteZero")),
            ErrorKind::Interrupted => AppError::new(arcstr::literal!("IO::Interrupted")),
            ErrorKind::UnexpectedEof => AppError::new(arcstr::literal!("IO::UnexpectedEof")),
            _ => AppError::new(arcstr::literal!("IO::Other")),
        }
    }
}

impl From<arangors::ClientError> for AppError {
    fn from(v: arangors::ClientError) -> Self {
        match v {
            ClientError::InsufficientPermission {
                permission,
                operation,
            } => AppError::new(arcstr::literal!("ArangoDB::InsufficientPermission"))
                .message(format!("Permission: {:?}, operation: {}", permission, operation).into()),
            ClientError::InvalidServer(e) => {
                AppError::new(arcstr::literal!("ArangoDB::InvalidServer")).message(e.into())
            }
            ClientError::Arango(e) => {
                let message = e.message().to_string().into();
                AppError::new(format!("ArangoDB::{}:{}", e.code(), e.error_num()).into())
                    .message(message)
            }
            ClientError::Serde(e) => {
                AppError::from(e).code(arcstr::literal!("ArangoClientError::Serde"))
            }
            ClientError::HttpClient(e) => AppError::new(arcstr::literal!("ArangoDB::HttpClient"))
                .message(format!("{:?}", e).into()),
        }
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(v: toml::ser::Error) -> Self {
        AppError::new(arcstr::literal!("Toml::Serialization")).message(format!("{}", v).into())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(v: toml::de::Error) -> Self {
        AppError::new(arcstr::literal!("Toml::Deserialization")).message(format!("{}", v).into())
    }
}

impl From<serde_json::error::Error> for AppError {
    fn from(v: serde_json::error::Error) -> Self {
        AppError::new(arcstr::literal!("Serde::Deserialization")).message(format!("{}", v).into())
    }
}

impl From<data_encoding::DecodeError> for AppError {
    fn from(v: data_encoding::DecodeError) -> Self {
        AppError::new(arcstr::literal!("DataEncoding::Decoding")).message(format!("{}", v).into())
    }
}

impl From<tindercrypt::errors::Error> for AppError {
    fn from(v: tindercrypt::errors::Error) -> Self {
        AppError::new(arcstr::literal!("DataEncrypting::Decrypting"))
            .message(format!("{:?}", v).into())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(v: reqwest::Error) -> Self {
        AppError::new(arcstr::literal!("HTTPClient::Sending")).message(format!("{:?}", v).into())
    }
}

impl From<solana_sdk::bs58::decode::Error> for AppError {
    fn from(v: solana_sdk::bs58::decode::Error) -> Self {
        let message = match v {
            solana_sdk::bs58::decode::Error::BufferTooSmall => arcstr::literal!("Buffer too small"),
            solana_sdk::bs58::decode::Error::InvalidCharacter { character, index } => {
                format!("Invalid character '{}' at {}", character, index).into()
            }
            solana_sdk::bs58::decode::Error::NonAsciiCharacter { index } => {
                format!("No ASCII character at {}", index).into()
            }
            _ => arcstr::literal!("Undefined Base58 error"),
        };

        AppError::new(arcstr::literal!("DataEncoding::Decoding")).message(message)
    }
}

impl From<anchor_client::solana_client::client_error::ClientError> for AppError {
    fn from(v: anchor_client::solana_client::client_error::ClientError) -> Self {
        match v.kind {
            anchor_client::solana_client::client_error::ClientErrorKind::Io(v) => v.into(),
            anchor_client::solana_client::client_error::ClientErrorKind::Reqwest(v) => v.into(),
            anchor_client::solana_client::client_error::ClientErrorKind::RpcError(v) => {
                AppError::new(arcstr::literal!("Solana::RpcError")).message(v.to_string().into())
            }
            anchor_client::solana_client::client_error::ClientErrorKind::SerdeJson(v) => v.into(),
            anchor_client::solana_client::client_error::ClientErrorKind::SigningError(v) => {
                v.into()
            }
            anchor_client::solana_client::client_error::ClientErrorKind::TransactionError(v) => {
                AppError::new(arcstr::literal!("Solana::TransactionError"))
                    .message(v.to_string().into())
            }
            _ => AppError::new(arcstr::literal!("Solana::Undefined")).message(v.to_string().into()),
        }
    }
}

impl From<solana_sdk::signature::SignerError> for AppError {
    fn from(v: solana_sdk::signature::SignerError) -> Self {
        AppError::new(arcstr::literal!("Solana::SigningError")).message(v.to_string().into())
    }
}

impl From<anchor_client::ClientError> for AppError {
    fn from(v: anchor_client::ClientError) -> Self {
        match v {
            anchor_client::ClientError::SolanaClientError(v) => v.into(),
            anchor_client::ClientError::ProgramError(v) => {
                AppError::new(arcstr::literal!("Solana::ProgramError"))
                    .message(v.to_string().into())
            }
            anchor_client::ClientError::AccountNotFound => {
                AppError::new(arcstr::literal!("Solana::AccountNotFound"))
            }
            _ => AppError::new(arcstr::literal!("Solana::Undefined")).message(v.to_string().into()),
        }
    }
}
