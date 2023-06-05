use anyhow::{Context, Result};
use reqwest::header::{self, HeaderValue};
use std::io::{self, Write};

pub mod cli;
pub mod gitlab;
pub mod util;

use crate::gitlab::{Branch, MergeRequest};

fn main() -> Result<()> {
    let cli::Config {
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
                if new_branch_name.trim().is_empty() {
                    eprintln!("You new branch's name should not be empty");
                    continue;
                }
                new_branch_name = new_branch_name.trim().replace(' ', "-");
                break;
            }
            Err(_) => eprintln!("Unable to read your branch name, please try again!"),
        }
    }

    for project in projects {
        let create_branch_arguments = gitlab::CreateBranchArgs {
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
                Ok(_) => {
                    if new_pr_title.trim().is_empty() {
                        eprintln!("New merge request's name should not be empty!");
                        continue;
                    }
                    break;
                }
                Err(_) => eprintln!("Unable to read your branch name, please try again!"),
            }
        }

        let create_mr_arguments = gitlab::CreateMergeRequestArgs {
            gitlab_url: &gitlab_url,
            project_id: &project.project_id,
            source_branch: &new_branch_name,
            target_branch: &project.target_branch,
            title: &new_pr_title,
        };
        let create_mr_response = gitlab::create_merge_request(&client, &create_mr_arguments)
            .with_context(|| {
                format!(
                    "An error ocurred while processing your request to create a merge request: {}",
                    new_pr_title
                )
            })?;
        util::handle_response_status(create_mr_response.status())?;
        let new_pr: MergeRequest = create_mr_response.json()?;
        println!(
            "New merge request for branch: \"{}\" created!",
            new_branch.name
        );
        println!("URL: {}", new_pr.web_url);
    }

    Ok(())
}
