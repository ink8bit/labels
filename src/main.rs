mod cli;
mod label;

use label::config::Config;
use label::github::GitHub;

#[tokio::main]
async fn main() {
    let args = cli::app().get_matches();
    let list = args.is_present("list");
    let update = args.is_present("update");

    if !list && !update {
        if let Err(e) = cli::app().print_help() {
            return eprintln!("{}", e);
        }
    }

    let config = match Config::new() {
        Ok(v) => v,
        Err(e) => return eprintln!("{}", e),
    };
    let repo = config.repo;
    let owner = config.owner;

    let gh = GitHub::new(&owner, &repo);

    if list {
        println!("Labels in repo {}", repo);
        if let Err(e) = gh.print_labels().await {
            return eprintln!("{}", e);
        }
    }

    if update {
        println!("Updating labels in repo {}", repo);
        if let Err(e) = gh.update_labels(&config.labels).await {
            eprintln!("{}", e);
        }
    }
}
