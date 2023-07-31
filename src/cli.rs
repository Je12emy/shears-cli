use clap::{Args, Parser, Subcommand};

pub const INVALID_TOKEN_DEFAULT_VALUE: &str = "INVALID";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CmdArgs {
    #[clap(subcommand)]
    pub action: ActionSubcommand,

    #[command(flatten)]
    pub source: Source,
}

#[derive(Debug, Subcommand)]
pub enum ActionSubcommand {
    /// Create a resource
    #[clap(subcommand)]
    Create(CreateSubCommand),
}

#[derive(Debug, Subcommand)]
pub enum CreateSubCommand {
    /// Create a release branch
    Branch(Branch),
    /// Create a release merge request
    MergeRequest(MergeRequest),
}

#[derive(Debug, Args)]
pub struct Source {
    /// URL of your Gitlab instance
    #[arg(
        global = true,
        default_value = "https://gitlab.com",
        short = 'u',
        long = "url"
    )]
    pub gitlab_url: String,
    /// Personal access token
    #[arg(global = true, default_value = INVALID_TOKEN_DEFAULT_VALUE, short, long)]
    pub access_token: String,
}

#[derive(Debug, Args)]
pub struct Branch {
    /// New branch name
    #[arg(short, long)]
    pub name: String,
    /// Source branch name
    #[arg(short, long)]
    pub base: String,
    // Project information
    #[command(flatten)]
    pub project: Project,
}

#[derive(Debug, Args)]
pub struct MergeRequest {
    /// Merge request title
    #[arg(short, long)]
    pub title: String,
    /// Target branch name
    #[arg(short, long)]
    pub target: String,
    /// Source branch
    #[arg(short, long)]
    pub source: String,
    // Project information
    #[command(flatten)]
    pub project: Project,
}

#[derive(Debug, Args)]
pub struct Project {
    /// Project ID
    #[arg(short, long)]
    pub id: u32,
}
