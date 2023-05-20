use clap::Parser;

#[allow(unused)]
#[derive(Debug, Parser)]
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
