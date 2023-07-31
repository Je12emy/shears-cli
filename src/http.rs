use anyhow::Result;
use clap::{error::ErrorKind, Command};
use reqwest::{blocking::Response, StatusCode};
use serde::de::DeserializeOwned;
use std::{error::Error, fmt::Display};

use crate::gitlab::ValidationErrorResponse;

pub enum Resource {
    Branch,
    MergeRequest,
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::Branch => write!(f, "branch"),
            Resource::MergeRequest => write!(f, "merge request"),
        }
    }
}

#[derive(Debug)]
pub enum ResponseHandlerError {
    NotOk(clap::error::Error),
    JSONDeserialization(reqwest::Error),
}

impl Error for ResponseHandlerError {}

impl Display for ResponseHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseHandlerError::NotOk(err) => write!(f, "{}", err),
            ResponseHandlerError::JSONDeserialization(err) => write!(f, "{}", err),
        }
    }
}

impl From<clap::error::Error> for ResponseHandlerError {
    fn from(value: clap::Error) -> Self {
        Self::NotOk(value)
    }
}

impl From<reqwest::Error> for ResponseHandlerError {
    fn from(value: reqwest::Error) -> Self {
        Self::JSONDeserialization(value)
    }
}

pub fn handle_response<T>(
    res: Response,
    mut cmd: Command,
    resource: Resource,
) -> Result<T, ResponseHandlerError>
where
    T: DeserializeOwned,
{
    match res.status() {
        StatusCode::CREATED => {
            let resource: T = res.json()?;
            Ok(resource)
        }
        StatusCode::BAD_REQUEST => {
            let error: ValidationErrorResponse = res.json()?;
            Err(ResponseHandlerError::NotOk(
                cmd.error(ErrorKind::InvalidValue, error.message),
            ))
        }
        StatusCode::UNAUTHORIZED => Err(ResponseHandlerError::NotOk(cmd.error(
            ErrorKind::InvalidValue,
            "An invalid token has been provided",
        ))),
        StatusCode::FORBIDDEN => Err(ResponseHandlerError::NotOk(cmd.error(
            ErrorKind::InvalidValue,
            "You are not allowed to perform this operation, please check your API permissions",
        ))),
        StatusCode::NOT_FOUND => Err(ResponseHandlerError::NotOk(cmd.error(
            ErrorKind::InvalidValue,
            "Make sure the provided values exist",
        ))),
        _ => Err(ResponseHandlerError::NotOk(cmd.error(
            ErrorKind::InvalidValue,
            format!("An error has ocurred while creating your new {}", resource),
        ))),
    }
}
