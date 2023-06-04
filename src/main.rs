use std::{
    error::Error,
    fmt::Display,
    fs::write,
    io::{self, Write},
    process,
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
    HTTPRequestError(HttpRequestError),
}

#[derive(Debug)]
pub enum HttpRequestError {
    Auth,
    NotFound,
    Server,
    Unexpected,
}

impl Error for HttpRequestError {}
impl Error for ShearsError {}

impl Display for HttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpRequestError::Auth => writeln!(f, "An authentication error was encountered, please ensure your access token is correct"),
            HttpRequestError::NotFound => writeln!(f, "Not found error, make sure your project ID and access token credentials are correct"),
            HttpRequestError::Server => writeln!(f, "An internal server error was encountered, if posible get in contact with a Gitlab adiminstrator"),
            HttpRequestError::Unexpected => writeln!(f, "An error ocurred while building the HTTP client."),
        }
    }
}

impl Display for ShearsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PrivateTokenError(_) => writeln!(f, "An error ocurred while creating the request headers, please make sure your ACCESS_TOKEN is correct"),
            Self::HTTPClientBuilderError(_) => writeln!(f, "An error ocurred while building the HTTP client."),
            Self::HTTPRequestError(kind) => writeln!(f, "{}", kind)
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
            Self::HTTPClientBuilderError(value);
        };
        todo!()
    }
}

fn main() -> Result<(), ShearsError> {
    let args::Config {
        private_token,
        projects,
        gitlab_url,
    } = util::build_config();

    let private_token_header = HeaderValue::from_str(&private_token).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let mut headers = header::HeaderMap::new();
    headers.insert("PRIVATE-TOKEN", private_token_header);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });

    let mut new_branch_name = String::new();
    loop {
        print!("Please enter a branch name: ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut new_branch_name) {
            Ok(_) => {
                new_branch_name = new_branch_name.trim().replace(" ", "-");
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
            .expect("An error ocurred while processing your request to create a new branch");
        util::handle_response_status(create_branch_response.status()).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(0);
        });
        let new_branch: Branch = create_branch_response.json().unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(0);
        });
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
        let create_pr_response = gitlab::create_pr(&client, &create_pr_arguments)
            .expect("An error ocurred while processing your request to create a merge request");
        util::handle_response_status(create_pr_response.status()).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(0);
        });
        let new_pr: MergeRequest = create_pr_response.json().unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(0);
        });
        println!(
            "New pull request for branch: \"{}\" created!",
            new_branch.name
        );
        println!("URL: {}", new_pr.web_url);
    }

    return Ok(());
}
