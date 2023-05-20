use serde::Deserialize;

use crate::args::Cli;
use reqwest::{blocking::Client, Error};

#[derive(Debug, Deserialize)]
pub struct GitlabError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
    pub web_url: String,
}

#[derive(Debug, Deserialize)]
pub struct MergeRequest {
    pub title: String,
    pub source_branch: String,
    pub target_branch: String,
    pub web_url: String,
}

pub fn create_pr(client: &Client, args: &Cli) -> Result<reqwest::blocking::Response, Error> {
    let create_pr_endpoint = format!(
        "{}/api/v4/projects/{}/merge_requests?source_branch={}&target_branch={}&title={}",
        args.gitlab_url, args.project_id, args.branch, args.target_branch, args.title
    );
    client.post(create_pr_endpoint).send()
}

pub fn create_branch(client: &Client, args: &Cli) -> Result<reqwest::blocking::Response, Error> {
    let create_branch_endpoint = format!(
        "{}/api/v4/projects/{}/repository/branches?branch={}&ref={}",
        args.gitlab_url, args.project_id, args.branch, args.base_branch
    );
    client.post(create_branch_endpoint).send()
}
