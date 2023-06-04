use std::fs;

use crate::{args, HttpRequestError, ShearsError};
use clap::Parser;
use directories::ProjectDirs;
use reqwest::StatusCode;

pub fn handle_response_status(status: StatusCode) -> Result<(), ShearsError> {
    match status {
        StatusCode::OK => Ok(()),
        StatusCode::CREATED => Ok(()),
        StatusCode::UNAUTHORIZED => Err(ShearsError::HTTPRequestError(HttpRequestError::Auth)),
        StatusCode::NOT_FOUND => Err(ShearsError::HTTPRequestError(HttpRequestError::NotFound)),
        StatusCode::BAD_REQUEST => Err(ShearsError::HTTPRequestError(HttpRequestError::Unexpected)),
        StatusCode::INTERNAL_SERVER_ERROR => {
            Err(ShearsError::HTTPRequestError(HttpRequestError::Server))
        }
        _ => Err(ShearsError::HTTPRequestError(HttpRequestError::Unexpected)),
    }
}

pub fn build_config() -> args::Config {
    let config: args::Config;
    let user_passed_arguments = std::env::args().count() > 1;

    if user_passed_arguments {
        let args = args::Cli::parse();
        config = args::Config {
            private_token: args.private_token,
            gitlab_url: args.gitlab_url.clone(),
            projects: vec![args::Project {
                project_id: args.project_id,
                base_branch: args.base_branch,
                target_branch: args.target_branch,
            }],
        };
        return config;
    }

    let project_dir = ProjectDirs::from("com", "je12emy", "shears-cli")
        .expect("Unable to locate a configuration directory");
    let config_dir = project_dir.config_dir();
    let config_file =
        fs::read_to_string(config_dir.join("config.toml")).expect("Unable to locate config.toml");
    config = toml::from_str(&config_file).expect(
        "Unable to read config.toml, please make sure you have a valid configuration file syntax",
    );
    return config;
}
