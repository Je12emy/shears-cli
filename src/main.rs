use anyhow::Result;
use clap::{CommandFactory, Parser};
use gitlab::CreatedBranchResponse;
use reqwest::header::{self, HeaderValue};

pub mod cli;
pub mod gitlab;

fn main() -> Result<()> {
    let args = cli::CmdArgs::parse();
    match args.action {
        cli::ActionSubcommand::Create(create_cmd) => {
            if args
                .source
                .access_token
                .eq(cli::INVALID_TOKEN_DEFAULT_VALUE)
            {
                let mut cmd = cli::CmdArgs::command();
                cmd.error(
                    clap::error::ErrorKind::InvalidValue,
                    "Please provide a valid value for access token",
                )
                .exit();
            }
            let private_token_header = HeaderValue::from_str(&args.source.access_token)?;

            let mut headers = header::HeaderMap::new();
            headers.insert("PRIVATE-TOKEN", private_token_header);

            let client = reqwest::blocking::Client::builder()
                .default_headers(headers)
                .build()?;

            match create_cmd {
                cli::CreateSubCommand::Branch(create_branch_cmd) => {
                    let cli::Branch {
                        base,
                        name,
                        project,
                    } = create_branch_cmd;
                    let payload = gitlab::CreateBranchArgs {
                        branch: name.as_str(),
                        source_branch: base.as_str(),
                        gitlab_url: args.source.gitlab_url.as_str(),
                        project_id: &project.id,
                    };
                    let res = gitlab::create_branch(&client, &payload)?;
                    if res.status().is_success() {
                        let created_branch: CreatedBranchResponse = res.json()?;
                        println!("{}", created_branch.web_url)
                    }
                }
                cli::CreateSubCommand::MergeRequest(create_merge_request_cmd) => {
                    let cli::MergeRequest {
                        title,
                        target,
                        source,
                        project,
                    } = create_merge_request_cmd;
                    let payload = gitlab::CreateMergeRequestArgs {
                        title: title.as_str(),
                        gitlab_url: args.source.gitlab_url.as_str(),
                        project_id: &project.id,
                        source_branch: source.as_str(),
                        target_branch: target.as_str(),
                    };
                    let res = gitlab::create_merge_request(&client, &payload)?;
                    if res.status().is_success() {
                        let created_merge_request: gitlab::CreatedMergeRequestResponse = res.json()?;
                        println!("{}", created_merge_request.web_url)
                    }
                }
            }
        }
    }
    Ok(())
}
