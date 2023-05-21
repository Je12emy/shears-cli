use std::fs;

use crate::{args, gitlab};
use clap::Parser;
use dialoguer::MultiSelect;
use directories::ProjectDirs;
use reqwest::{blocking::Response, StatusCode};
use serde::de::DeserializeOwned;

pub fn handle_response_status<T>(status: StatusCode, resource: String, response: Response) -> T
where
    T: DeserializeOwned,
{
    match status {
        StatusCode::OK => parse_response::<T>(response, resource),
        StatusCode::CREATED => parse_response::<T>(response, resource),
        StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized, please make sure your personal access token is correct!")
        }
        StatusCode::NOT_FOUND => {
            let json_response = response.json::<gitlab::GitlabError>().expect(
                format!(
                    "An unkown error happened while creating your new {}!",
                    resource
                )
                .as_str(),
            );
            panic!("Not Found error: {}", json_response.message)
        }
        StatusCode::BAD_REQUEST => {
            let text = response.text().unwrap();
            println!("text: {}", text);
            // let json_response = response.json().expect(
            //     format!(
            //         "An unkown error happened while creating your new {}!",
            //         resource
            //     )
            //     .as_str(),
            // );
            panic!(
                "A validation error ocurred while creating your new {}",
                resource
            );
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            panic!("Internal server error, please contact Gitlab if you see this");
        }
        _ => panic!(
            "An unexpected error ocurred while creating your {}",
            resource
        ),
    }
}

fn parse_response<T>(response: Response, resource: String) -> T
where
    T: DeserializeOwned,
{
    let new_entity = response.json::<T>().expect(
        format!(
            "An error ocurred while reading the response to create a {}",
            resource
        )
        .as_str(),
    );
    return new_entity;
}

pub fn build_config() -> args::Config {
    let mut config: args::Config;
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

    let project_dir = ProjectDirs::from("com", "je12emy", "shears-cli").unwrap();
    let config_dir = project_dir.config_dir();
    let config_file = fs::read_to_string(config_dir.join("config.toml")).unwrap();
    config = toml::from_str(&config_file).unwrap();
    let selected_projects = select_projects(config.projects);
    config.projects = selected_projects;
    return config;
}

fn select_projects(projects: Vec<args::Project>) -> Vec<args::Project> {
    let defaults = vec![true];
    let options: Vec<&str> = projects.iter().map(|p| p.project_id.as_str()).collect();
    let chosen: Vec<usize> = MultiSelect::new()
        .with_prompt("Please pick projects for this release")
        .defaults(&defaults)
        .items(&options)
        .interact()
        .unwrap();
    let picked_projects = projects
        .iter()
        .enumerate()
        .filter(|&(index, _)| chosen.contains(&index))
        .map(|(_, value)| value.clone())
        .collect();
    return picked_projects;
}
