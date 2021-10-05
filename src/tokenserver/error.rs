use std::fmt;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::{
    ser::{SerializeMap, Serializer},
    Serialize,
};

#[derive(Debug)]
pub struct TokenserverError {
    status: &'static str,
    location: ErrorLocation,
    name: &'static str,
    description: &'static str,
    http_status: StatusCode,
}

impl Default for TokenserverError {
    fn default() -> Self {
        Self {
            status: "",
            location: ErrorLocation::default(),
            name: "",
            description: "Unauthorized",
            http_status: StatusCode::UNAUTHORIZED,
        }
    }
}

impl TokenserverError {
    pub fn invalid_generation() -> Self {
        Self {
            status: "invalid-generation",
            location: ErrorLocation::Body,
            ..Self::default()
        }
    }

    pub fn invalid_keys_changed_at() -> Self {
        Self {
            status: "invalid-keysChangedAt",
            location: ErrorLocation::Body,
            ..Self::default()
        }
    }

    pub fn invalid_key_id(description: &'static str) -> Self {
        Self {
            status: "invalid-key-id",
            description,
            ..Self::default()
        }
    }

    pub fn invalid_credentials(description: &'static str) -> Self {
        Self {
            status: "invalid-credentials",
            description,
            ..Self::default()
        }
    }

    pub fn invalid_client_state(description: &'static str) -> Self {
        Self {
            status: "invalid-client-state",
            description,
            ..Self::default()
        }
    }

    pub fn internal_error() -> Self {
        Self {
            status: "internal-error",
            location: ErrorLocation::Internal,
            description: "Server error",
            http_status: StatusCode::INTERNAL_SERVER_ERROR,
            ..Self::default()
        }
    }

    pub fn unsupported(description: &'static str) -> Self {
        Self {
            status: "error",
            location: ErrorLocation::Url,
            description,
            http_status: StatusCode::NOT_FOUND,
            ..Self::default()
        }
    }

    pub fn unauthorized(description: &'static str) -> Self {
        Self {
            status: "error",
            description,
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorLocation {
    Header,
    Url,
    Body,
    Internal,
}

impl Default for ErrorLocation {
    fn default() -> Self {
        Self::Header
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header => write!(f, "header"),
            Self::Url => write!(f, "url"),
            Self::Body => write!(f, "body"),
            Self::Internal => write!(f, "internal"),
        }
    }
}

impl fmt::Display for TokenserverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).map_err(|_| fmt::Error)?
        )
    }
}

impl ResponseError for TokenserverError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(ErrorResponse::from(self))
    }

    fn status_code(&self) -> StatusCode {
        self.http_status
    }
}

struct ErrorResponse {
    status: &'static str,
    errors: [ErrorInstance; 1],
}

struct ErrorInstance {
    location: ErrorLocation,
    name: &'static str,
    description: &'static str,
}

impl From<&TokenserverError> for ErrorResponse {
    fn from(error: &TokenserverError) -> Self {
        ErrorResponse {
            status: error.status,
            errors: [ErrorInstance {
                location: error.location,
                name: error.name,
                description: error.description,
            }],
        }
    }
}

impl Serialize for ErrorInstance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("location", &self.location.to_string())?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("description", &self.description)?;
        map.end()
    }
}

impl Serialize for ErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("status", &self.status)?;
        map.serialize_entry("errors", &self.errors)?;
        map.end()
    }
}

impl Serialize for TokenserverError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ErrorResponse::from(self).serialize(serializer)
    }
}
