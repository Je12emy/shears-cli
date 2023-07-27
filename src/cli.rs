use clap::{Parser, Args, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CmdArgs {
    #[clap(subcommand)]
    pub action: ActionSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ActionSubcommand {
    #[clap(subcommand)]
    /// Create a resource
    Create(CreateSubCommand),
}

#[derive(Debug, Subcommand)]
pub enum CreateSubCommand {
    /// Create a release branch 
    Branch(Branch),
    /// Create a release merge request
    MergeRequest(MergeRequest),
    /// A release includes creating a new branch and a merge request
    Release(Release),
}

#[derive(Debug, Args)]
pub struct Release {
    #[command(flatten)]
    pub merge_request: MergeRequest,
}

#[derive(Debug, Args)]
pub struct Source {
    /// URL of your Gitlab instance
    pub gitlab_url: String,
    /// Personal access token
    pub access_token: String,
}

#[derive(Debug, Args)]
pub struct Branch {
    /// New branch name
    pub name: String,
    /// Source branch name
    pub source: String,
}

#[derive(Debug, Args)]
pub struct MergeRequest {
    /// Merge request title
    pub title: String,
    /// Target branch name
    pub target: String,
    /// Source branch
    #[command(flatten)]
    pub branch: Branch
}
