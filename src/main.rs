use clap::Parser;
use std::{error::Error, panic};

pub mod args;
pub mod gitlab;

use reqwest::{
    header::{self, HeaderValue},
    StatusCode,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Cli::parse();
    println!("{:?}", args);

    let private_token_header: HeaderValue = args.private_token
        .clone()
        .try_into()
        .expect("An error ocurred while creating the request headers, please make sure your ACCESS_TOKEN is correct");
    let mut headers = header::HeaderMap::new();
    headers.insert("PRIVATE-TOKEN", private_token_header);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("An error ocurred while creating the HTTP client");

    let create_branch_response = gitlab::create_branch(&client, &args)
        .expect("An error ocurred while processing your request to create a new branch");
    match create_branch_response.status() {
        StatusCode::OK => (),
        StatusCode::CREATED => (),
        StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized, please make sure your personal access token is correct!")
        }
        StatusCode::NOT_FOUND => {
            let json_response = create_branch_response
                .json::<gitlab::GitlabError>()
                .expect("An unkown error happened while creating your new branch!");
            panic!("Not Found error: {}", json_response.message)
        }
        StatusCode::BAD_REQUEST => {
            let json_response = create_branch_response
                .json::<gitlab::GitlabError>()
                .expect("An unkown error happened while creating your new branch!");
            panic!(
                "A validation error ocurred while creating your new branch: {}",
                json_response.message
            );
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            panic!("Internal server error, please contact Gitlab if you see this");
        }
        _ => panic!("An unexpected error ocurred while creating your new branch"),
    }

    let new_branch = create_branch_response
        .json::<gitlab::Branch>()
        .expect("An error ocurred while reading the response");
    println!("New branch {} created!", new_branch.name);
    println!("URL: {}", new_branch.web_url);

    let create_pr_response = gitlab::create_pr(&client, &args)
        .expect("An error ocurred while processing your request to create a merge request");
    match create_pr_response.status() {
        StatusCode::OK => (),
        StatusCode::CREATED => (),
        StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized, please make sure your personal access token is correct!")
        }
        StatusCode::NOT_FOUND => {
            let json_response = create_pr_response
                .json::<gitlab::GitlabError>()
                .expect("An unkown error happened while creating your new merge request!");
            panic!("Not Found error: {}", json_response.message)
        }
        StatusCode::BAD_REQUEST => {
            let json_response = create_pr_response
                .json::<gitlab::GitlabError>()
                .expect("An unkown error happened while creating your new merge request!");
            panic!(
                "A validation error ocurred while creating your new merge request: {}",
                json_response.message
            );
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            panic!("Internal server error, please contact Gitlab if you see this");
        }
        _ => panic!("An unexpected error ocurred while creating your merge request"),
    }

    let new_pr = create_pr_response
        .json::<gitlab::MergeRequest>()
        .expect("An error ocurred while reading the merge request response");
    println!("New pull request \"{}\" created!", new_pr.title);
    println!("URL: {}", new_pr.web_url);

    return Ok(());
}
