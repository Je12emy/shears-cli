use std::{error::Error, fmt::Display};

use clap::Parser;
use reqwest::{header::InvalidHeaderValue, Error as ReqwestError};
use serde::Deserialize;

#[derive(Debug, Parser, Clone)]
#[command(author = "Jeremy Zelaya R. <jeremy@je12emy.com>")]
#[command(about = "Automate cutting release branches for repositories hosted on Gitlab")]
pub struct Cli {
    #[arg(short = 't', long = "token")]
    pub private_token: String,
    #[arg(short = 'i', long = "id")]
    pub project_id: String,
    #[arg(short = 's', long = "source")]
    pub base_branch: String,
    #[arg(short = 'd', long = "destination")]
    pub target_branch: String,
    #[arg(default_value_t = String::from("https://gitlab.com"))]
    #[arg(short = 'u', long = "url")]
    pub gitlab_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub project_id: String,
    pub base_branch: String,
    pub target_branch: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub private_token: String,
    pub gitlab_url: String,
    pub projects: Vec<Project>,
}

#[derive(Debug)]
pub enum CliError {
    PrivateTokenError(InvalidHeaderValue),
    HTTPClientBuilderError(ReqwestError),
    HTTPError(HttpError),
    RequestError(ReqwestError),
}

#[derive(Debug)]
pub enum HttpError {
    Auth,
    NotFound,
    Server,
    Unexpected,
}

impl Error for HttpError {}
impl Error for CliError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::Auth => writeln!(f, "An authentication error was encountered, please ensure your access token is correct"),
            HttpError::NotFound => writeln!(f, "Not found error, make sure your project ID and access token credentials are correct"),
            HttpError::Server => writeln!(f, "An internal server error was encountered, if posible get in contact with a Gitlab adiminstrator"),
            HttpError::Unexpected => writeln!(f, "An unkown error was encountered, please report this issue"),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PrivateTokenError(_) => writeln!(
                f,
                "You may have passed an invalid non-visible character like new line: \"\\n\""
            ),
            Self::HTTPClientBuilderError(_) => {
                writeln!(f, "An error ocurred while building the HTTP client.")
            }
            Self::HTTPError(kind) => writeln!(f, "{}", kind),
            Self::RequestError(kind) => writeln!(
                f,
                "An error ocurred while processing your request, {}",
                kind
            ),
        }
    }
}

impl std::convert::From<InvalidHeaderValue> for CliError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::PrivateTokenError(value)
    }
}

impl std::convert::From<ReqwestError> for CliError {
    fn from(value: ReqwestError) -> Self {
        if value.is_builder() {
            return Self::HTTPClientBuilderError(value);
        }
        if value.is_request() {
            return Self::RequestError(value);
        }
        todo!()
    }
}
