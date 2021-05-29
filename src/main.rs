mod cli;
mod label;

use label::config::Config;
use label::github;

fn main() {
    let args = cli::args();
    let list = args.is_present("list");
    let update = args.is_present("update");

    let config = match Config::new() {
        Ok(v) => v,
        Err(e) => return eprint!("Config file not found: {}", e),
    };

    let repo = config.repo;
    let owner = config.owner;

    if list {
        println!("Labels in repo {}", repo);
        if let Err(e) = github::print_labels(&owner, &repo) {
            panic!("{}", e);
        }
    }

    if update {
        println!("Updating labels in repo {}", repo);
        if let Err(e) = github::update_labels(&owner, &repo, &config.labels) {
            panic!("{}", e);
        }
    }
}
