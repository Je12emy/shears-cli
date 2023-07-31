use anyhow::Result;
use clap::{error::ErrorKind, CommandFactory, Parser};
use reqwest::header::{self, HeaderValue};

use crate::http::ResponseHandlerError;

pub mod cli;
pub mod gitlab;
pub mod http;

fn main() -> Result<()> {
    let args = cli::CmdArgs::parse();
    let mut cmd = cli::CmdArgs::command();
    match args.action {
        cli::ActionSubcommand::Create(create_cmd) => {
            if args
                .source
                .access_token
                .eq(cli::INVALID_TOKEN_DEFAULT_VALUE)
            {
                cmd.error(
                    ErrorKind::InvalidValue,
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
                    let created_branch = http::handle_response::<gitlab::CreatedBranchResponse>(
                        res,
                        cmd,
                        http::Resource::Branch,
                    )
                    .map_err(|err| {
                        if let ResponseHandlerError::NotOk(cmd_error) = err {
                            cmd_error.exit()
                        }
                        err
                    })?;
                    println!("{}", created_branch.web_url);
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
                    let created_merge_request = http::handle_response::<
                        gitlab::CreatedMergeRequestResponse,
                    >(
                        res, cmd, http::Resource::Branch
                    )
                    .map_err(|err| {
                        if let ResponseHandlerError::NotOk(cmd_error) = err {
                            cmd_error.exit()
                        }
                        err
                    })?;
                    println!("{}", created_merge_request.web_url);
                }
            }
        }
    }
    Ok(())
}
