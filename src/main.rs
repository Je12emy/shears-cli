use std::error::Error;

use clap::Parser;
use reqwest::{blocking::Client, header, Error as ReqestError};

#[derive(Debug, Parser)]
#[command(author = "Jeremy Zelaya R. <jeremy@je12emy.com>")]
#[command(about = "Automate cutting release branches for repositories hosted on Gitlab")]
struct Cli {
    private_token: String,
    project_id: String,
    branch: String,
    base_branch: String,
    target_branch: String,
    title: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("PRIVATE-TOKEN", args.private_token.parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = create_branch(&client, &args).unwrap();
    println!("Branch res text: {:?}", response.text().unwrap());

    let response = create_pr(&client, &args).unwrap();
    println!("PR res text: {:?}", response.text().unwrap());

    return Ok(());
}

fn create_branch(client: &Client, args: &Cli) -> Result<reqwest::blocking::Response, ReqestError> {
    let create_branch_endpoint = format!(
        "https://gitlab.com/api/v4/projects/{}/repository/branches?branch={}&ref={}",
        args.project_id, args.branch, args.base_branch
    );
    client.post(create_branch_endpoint).send()
}

fn create_pr(client: &Client, args: &Cli) -> Result<reqwest::blocking::Response, ReqestError> {
    let create_pr_endpoint = format!(
        "https://gitlab.com/api/v4/projects/{}/merge_requests?source_branch={}&target_branch={}&title={}",
        args.project_id,
        args.branch,
        args.target_branch,
        args.title
    );
    client.post(create_pr_endpoint).send()
}
