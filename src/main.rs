use clap::Parser;
use std::{
    borrow::{Borrow, BorrowMut},
    error::Error,
    panic,
};

pub mod args;
pub mod gitlab;
pub mod util;

use reqwest::{
    header::{self, HeaderValue},
    StatusCode,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Cli::parse();

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
    util::handle_response_status(
        create_branch_response.status(),
        String::from("branch"),
        &create_branch_response,
    );
    let new_branch = create_branch_response
        .json::<gitlab::Branch>()
        .expect("An error ocurred while reading the response");
    println!("New branch {} created!", new_branch.name);
    println!("URL: {}", new_branch.web_url);
    let create_pr_response = gitlab::create_pr(&client, &args)
        .expect("An error ocurred while processing your request to create a merge request");
    util::handle_response_status(
        create_pr_response.status(),
        String::from("merge request"),
        &create_pr_response,
    );
    let new_pr = create_pr_response
        .json::<gitlab::MergeRequest>()
        .expect("An error ocurred while reading the merge request response");
    println!("New pull request \"{}\" created!", new_pr.title);
    println!("URL: {}", new_pr.web_url);

    return Ok(());
}
