use std::error::Error;

use clap::Parser;
use reqwest::{
    blocking::Client,
    header::{self, HeaderValue},
    Error as ReqestError,
};

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
    #[arg(default_value_t = String::from("https://gitlab.com"))]
    gitlab_url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
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

    let response = create_branch(&client, &args).unwrap();
    println!("Branch res text: {:?}", response.text().unwrap());

    let response = create_pr(&client, &args).unwrap();
    println!("PR res text: {:?}", response.text().unwrap());

    return Ok(());
}

fn create_branch(client: &Client, args: &Cli) -> Result<reqwest::blocking::Response, ReqestError> {
    let create_branch_endpoint = format!(
        "{}/api/v4/projects/{}/repository/branches?branch={}&ref={}",
        args.gitlab_url, args.project_id, args.branch, args.base_branch
    );
    client.post(create_branch_endpoint).send()
}

fn create_pr(client: &Client, args: &Cli) -> Result<reqwest::blocking::Response, ReqestError> {
    let create_pr_endpoint = format!(
        "{}/api/v4/projects/{}/merge_requests?source_branch={}&target_branch={}&title={}",
        args.gitlab_url, args.project_id, args.branch, args.target_branch, args.title
    );
    client.post(create_pr_endpoint).send()
}
