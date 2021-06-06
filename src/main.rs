mod cli;
mod label;

use label::config::Config;
use label::github::GitHub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::app().get_matches();
    let list = args.is_present("list");
    let update = args.is_present("update");

    if !list && !update {
        cli::app().print_help()?;
        return Ok(());
    }

    let config = Config::new()?;
    let repo = config.repo;
    let owner = config.owner;

    let gh = GitHub::new(&owner, &repo);

    if list {
        println!("Labels in repo {}", repo);
        gh.print_labels().await?;
    }

    if update {
        println!("Updating labels in repo {}", repo);
        gh.update_labels(&config.labels).await?;
    }

    Ok(())
}
