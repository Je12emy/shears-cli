use std::{
    error::Error,
    io::{self, Write},
};

pub mod args;
pub mod gitlab;
pub mod util;

use reqwest::header::{self, HeaderValue};

fn main() -> Result<(), Box<dyn Error>> {
    let config = util::build_config();

    let private_token_header: HeaderValue = config.private_token
        .clone()
        .try_into()
        .expect("An error ocurred while creating the request headers, please make sure your ACCESS_TOKEN is correct");
    let mut headers = header::HeaderMap::new();
    headers.insert("PRIVATE-TOKEN", private_token_header);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("An error ocurred while creating the HTTP client");

    let gitlab_url = config.gitlab_url.clone();

    print!("Please enter a branch name: ");
    let _ = io::stdout().flush();
    let mut new_branch_name = String::new();
    io::stdin().read_line(&mut new_branch_name).unwrap();

    for project in config.projects {
        let create_branch_arguments = gitlab::CreateBranch {
            gitlab_url: &gitlab_url,
            project_id: &project.project_id,
            branch: &new_branch_name,
            source_branch: &project.base_branch,
        };
        let create_branch_response = gitlab::create_branch(&client, &create_branch_arguments)
            .expect("An error ocurred while processing your request to create a new branch");
        let new_branch = util::handle_response_status::<gitlab::Branch>(
            create_branch_response.status(),
            String::from("branch"),
            create_branch_response,
        );
        println!("New branch {} created!", new_branch.name);
        println!("URL: {}", new_branch.web_url);

        print!(
            "Please enter a PR title for branch \"{}\": ",
            new_branch.name,
        );
        let _ = io::stdout().flush();

        let mut new_pr_title = String::new();
        io::stdin().read_line(&mut new_pr_title).unwrap();

        let create_pr_arguments = gitlab::CreatePR {
            gitlab_url: &gitlab_url,
            project_id: &project.project_id,
            source_branch: &new_branch_name,
            target_branch: &project.target_branch,
            title: &new_pr_title,
        };
        let create_pr_response = gitlab::create_pr(&client, &create_pr_arguments)
            .expect("An error ocurred while processing your request to create a merge request");
        let new_pr = util::handle_response_status::<gitlab::MergeRequest>(
            create_pr_response.status(),
            String::from("merge request"),
            create_pr_response,
        );
        println!(
            "New pull request for branch: \"{}\" created!",
            new_branch.name
        );
        println!("URL: {}", new_pr.web_url);
    }

    return Ok(());
}
