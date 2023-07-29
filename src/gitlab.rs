use anyhow::Result;
use reqwest::{blocking::Client, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitlabError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatedBranchResponse {
    pub name: String,
    pub web_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreatedMergeRequestResponse {
    pub source_branch: String,
    pub target_branch: String,
    pub web_url: String,
}

#[derive(Debug, Clone)]
pub struct CreateBranchArgs<'a> {
    pub gitlab_url: &'a str,
    pub project_id: &'a u32,
    pub branch: &'a str,
    pub source_branch: &'a str,
}

#[derive(Debug, Clone)]
pub struct CreateMergeRequestArgs<'a> {
    pub gitlab_url: &'a str,
    pub project_id: &'a u32,
    pub source_branch: &'a str,
    pub target_branch: &'a str,
    pub title: &'a str,
}

pub fn create_merge_request(
    client: &Client,
    args: &CreateMergeRequestArgs,
) -> Result<reqwest::blocking::Response, Error> {
    let create_pr_endpoint = format!(
        "{}/api/v4/projects/{}/merge_requests?source_branch={}&target_branch={}&title={}",
        args.gitlab_url, args.project_id, args.source_branch, args.target_branch, args.title
    );
    client.post(create_pr_endpoint).send()
}

pub fn create_branch(
    client: &Client,
    args: &CreateBranchArgs,
) -> Result<reqwest::blocking::Response, Error> {
    let create_branch_endpoint = format!(
        "{}/api/v4/projects/{}/repository/branches?branch={}&ref={}",
        args.gitlab_url, args.project_id, args.branch, args.source_branch
    );
    client.post(create_branch_endpoint).send()
}
