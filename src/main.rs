use anyhow::{Context, Result};
use std::{
    error::Error,
    fmt::Display,
    io::{self, Write},
};

pub mod args;
pub mod gitlab;
pub mod util;

use reqwest::{
    header::{self, HeaderValue, InvalidHeaderValue},
    Error as ReqwestError,
};

use crate::gitlab::{Branch, MergeRequest};

#[derive(Debug)]
pub enum ShearsError {
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
impl Error for ShearsError {}

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

impl Display for ShearsError {
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

impl std::convert::From<InvalidHeaderValue> for ShearsError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::PrivateTokenError(value)
    }
}

impl std::convert::From<ReqwestError> for ShearsError {
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

fn main() -> Result<()> {
    let args::Config {
        private_token,
        projects,
        gitlab_url,
    } = util::build_config();

    let private_token_header = HeaderValue::from_str(&private_token)?;

    let mut headers = header::HeaderMap::new();
    headers.insert("PRIVATE-TOKEN", private_token_header);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let mut new_branch_name = String::new();
    loop {
        print!("Please enter a branch name: ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut new_branch_name) {
            Ok(_) => {
                new_branch_name = new_branch_name.trim().replace(' ', "-");
                break;
            }
            Err(_) => eprintln!("Unable to read your branch name, please try again!"),
        }
    }

    for project in projects {
        let create_branch_arguments = gitlab::CreateBranch {
            gitlab_url: &gitlab_url,
            project_id: &project.project_id,
            branch: &new_branch_name,
            source_branch: &project.base_branch,
        };
        let create_branch_response = gitlab::create_branch(&client, &create_branch_arguments)
            .with_context(|| {
                format!(
                    "An error ocurred while processing your request to create the branch \"{}\"",
                    &new_branch_name
                )
            })?;
        util::handle_response_status(create_branch_response.status())?;
        let new_branch: Branch = create_branch_response.json()?;
        println!("New branch {} created!", new_branch.name);
        println!("URL: {}", new_branch.web_url);

        print!(
            "Please enter a PR title for branch \"{}\": ",
            new_branch.name,
        );
        let _ = io::stdout().flush();

        let mut new_pr_title = String::new();
        loop {
            print!("Please enter a branch name: ");
            let _ = io::stdout().flush();
            match io::stdin().read_line(&mut new_pr_title) {
                Ok(_) => break,
                Err(_) => eprintln!("Unable to read your branch name, please try again!"),
            }
        }

        let create_pr_arguments = gitlab::CreatePR {
            gitlab_url: &gitlab_url,
            project_id: &project.project_id,
            source_branch: &new_branch_name,
            target_branch: &project.target_branch,
            title: &new_pr_title,
        };
        let create_pr_response =
            gitlab::create_pr(&client, &create_pr_arguments).with_context(|| {
                format!(
                    "An error ocurred while processing your request to create a merge request: {}",
                    new_pr_title
                )
            })?;
        util::handle_response_status(create_pr_response.status())?;
        let new_pr: MergeRequest = create_pr_response.json()?;
        println!(
            "New pull request for branch: \"{}\" created!",
            new_branch.name
        );
        println!("URL: {}", new_pr.web_url);
    }

    Ok(())
}
