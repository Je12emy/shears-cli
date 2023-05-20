use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = "Jeremy Zelaya R. <jeremy@je12emy.com>")]
#[command(about = "Automate cutting release branches for repositories hosted on Gitlab")]
pub struct Cli {
    pub private_token: String,
    pub project_id: String,
    pub branch: String,
    pub base_branch: String,
    pub target_branch: String,
    pub title: String,
    #[arg(default_value_t = String::from("https://gitlab.com"))]
    pub gitlab_url: String,
}
