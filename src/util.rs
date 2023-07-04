use std::fs;

use crate::cli::{self, CliError, HttpError, Project};
use anyhow::Result;
use clap::Parser;
use directories::ProjectDirs;
use picker::Picker;
use reqwest::StatusCode;

pub fn handle_response_status(status: StatusCode) -> Result<(), CliError> {
    match status {
        StatusCode::OK => Ok(()),
        StatusCode::CREATED => Ok(()),
        StatusCode::UNAUTHORIZED => Err(CliError::HTTPError(HttpError::Auth)),
        StatusCode::NOT_FOUND => Err(CliError::HTTPError(HttpError::NotFound)),
        StatusCode::BAD_REQUEST => Err(CliError::HTTPError(HttpError::Unexpected)),
        StatusCode::INTERNAL_SERVER_ERROR => Err(CliError::HTTPError(HttpError::Server)),
        _ => Err(CliError::HTTPError(HttpError::Unexpected)),
    }
}

pub fn build_config() -> cli::Config {
    let mut config: cli::Config;
    let user_passed_arguments = std::env::args().count() > 1;

    if user_passed_arguments {
        let cli = cli::Cli::parse();
        config = cli::Config {
            private_token: cli.private_token,
            gitlab_url: cli.gitlab_url.clone(),
            projects: vec![cli::Project {
                name: String::from(""),
                project_id: cli.project_id,
                base_branch: cli.base_branch,
                target_branch: cli.target_branch,
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

    let options: Vec<&str> = config.projects.iter().map(|x| x.name.as_str()).collect();
    let mut picker = Picker::new(&options);
    picker.select();
    if let Some(selected) = picker.selected_options_indexes {
        let mut remaining_projects: Vec<Project> = Vec::new();
        for (index, project) in config.projects.into_iter().enumerate() {
            if selected.contains(&index) {
                remaining_projects.push(project);
            }
        }
        config.projects = remaining_projects;
    }
    config
}
