mod cli;
mod labels;

use terminal_spinners::{SpinnerBuilder, SpinnerHandle, DOTS};

use std::env;

use cli::sub_cmd::{list::LIST_CMD, remove::REMOVE_CMD, update::UPDATE_CMD};
use labels::config::Config;
use labels::github::GitHub;

/// GitHub API base url.
const API_URL: &str = "https://api.github.com";

/// An environment variable with your personal access token.
const LABELS_TOKEN: &str = "LABELS_TOKEN";

#[tokio::main]
async fn main() {
    let args = cli::app().get_matches();

    let config = match Config::new() {
        Ok(v) => v,
        Err(_) => return eprintln!("Error: no config file found"),
    };

    let repo = config.repo;
    let owner = config.owner;

    let token = match env::var(LABELS_TOKEN) {
        Ok(v) => v,
        Err(_) => return eprintln!("LABELS_TOKEN not found"),
    };

    let gh = match GitHub::new(API_URL.to_string(), token) {
        Ok(v) => v,
        Err(e) => {
            return eprintln!("{}", e);
        }
    };

    match args.subcommand() {
        Some((LIST_CMD, _)) => {
            let msg = format!("Getting labels from repo '{}'...", repo);
            let sp = create_spinner(&msg);

            match gh.print_labels(&owner, &repo).await {
                Ok(labels) => {
                    let msg = format!("Labels in repo '{}':", repo);
                    sp.text(msg);
                    sp.done();
                    println!("{}", labels);
                }
                Err(e) => {
                    let err_msg = format!("Error: {}", e);
                    sp.text(err_msg);
                    sp.error();
                }
            };
        }
        Some((UPDATE_CMD, _)) => {
            let msg = format!("Updating labels in repo '{}'", repo);
            let sp = create_spinner(&msg);

            match gh.update_labels(&config.labels, &owner, &repo).await {
                Ok(_) => {
                    let msg = format!("Successfully updated labels in repo '{}'", repo);
                    sp.text(msg);
                    sp.done();
                }
                Err(e) => {
                    let err_msg = format!("Error: {}", e);
                    sp.text(err_msg);
                    sp.error();
                }
            }
        }
        Some((REMOVE_CMD, _)) => {
            let msg = format!("Removing all labels from repo '{}'", repo);
            let sp = create_spinner(&msg);

            match gh.remove_labels(&owner, &repo).await {
                Ok(_) => {
                    let msg = format!("Successfully removed all labels in repo '{}'", repo);
                    sp.text(msg);
                    sp.done();
                }
                Err(e) => {
                    let err_msg = format!("Error: {}", e);
                    sp.text(err_msg);
                    sp.error();
                }
            }
        }
        _ => {
            if let Err(e) = cli::app().print_help() {
                return eprintln!("{}", e);
            }
        }
    }
}

fn create_spinner(msg: &str) -> SpinnerHandle {
    let formatted = format!(" {}", msg);
    SpinnerBuilder::new().spinner(&DOTS).text(formatted).start()
}
