use clap::Parser;

#[allow(unused)]
#[derive(Debug, Parser)]
#[command(author = "Jeremy Zelaya R. <jeremy@je12emy.com>")]
#[command(about = "Automate cutting release branches for repositories hosted on Gitlab")]
struct Cli {
    private_token: String,
    project_id: String,
    target_branch: String,
    base_branch: String,
    branch: String,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
