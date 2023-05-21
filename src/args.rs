use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Parser, Clone)]
#[command(author = "Jeremy Zelaya R. <jeremy@je12emy.com>")]
#[command(about = "Automate cutting release branches for repositories hosted on Gitlab")]
pub struct Cli {
    #[arg(short = 't', long = "token")]
    pub private_token: String,
    #[arg(short = 'i', long = "id")]
    pub project_id: String,
    #[arg(short = 's', long = "source")]
    pub base_branch: String,
    #[arg(short = 'd', long = "destination")]
    pub target_branch: String,
    #[arg(default_value_t = String::from("https://gitlab.com"))]
    #[arg(short = 'u', long = "url")]
    pub gitlab_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub project_id: String,
    pub base_branch: String,
    pub target_branch: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub private_token: String,
    pub gitlab_url: String,
    pub projects: Vec<Project>,
}
