mod cli;
mod labels;

use terminal_spinners::{SpinnerBuilder, SpinnerHandle, DOTS};

use labels::config::Config;
use labels::github::GitHub;

#[tokio::main]
async fn main() {
    let args = cli::app().get_matches();
    let list = args.is_present("list");
    let update = args.is_present("update");
    let remove = args.is_present("remove");

    if !list && !update && !remove {
        if let Err(e) = cli::app().print_help() {
            return eprintln!("{}", e);
        }
    }

    let config = match Config::new() {
        Ok(v) => v,
        Err(_) => return eprintln!("Error: no config file found"),
    };

    let repo = config.repo;
    let owner = config.owner;

    let gh = GitHub::new(&owner, &repo);

    if list {
        let msg = format!("Getting labels from repo '{}'...", repo);
        let sp = create_spinner(&msg);

        match gh.print_labels().await {
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

    if update {
        let msg = format!("Updating labels in repo '{}'", repo);
        let sp = create_spinner(&msg);

        match gh.update_labels(&config.labels).await {
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

    if remove {
        let msg = format!("Removing all labels from repo '{}'", repo);
        let sp = create_spinner(&msg);

        match gh.remove_labels().await {
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
}

fn create_spinner(msg: &str) -> SpinnerHandle {
    let formatted = format!(" {}", msg);
    SpinnerBuilder::new().spinner(&DOTS).text(formatted).start()
}
