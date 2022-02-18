use clap::{crate_authors, crate_description, crate_name, crate_version, Command};

pub(crate) fn app() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            Command::new("list")
                .version(crate_version!())
                .about("Prints labels in current repository (first 100 items)"),
        )
        .subcommand(
            Command::new("update")
                .version(crate_version!())
                .about("Updates all labels in current repository"),
        )
        .subcommand(
            Command::new("remove")
                .version(crate_version!())
                .about("Removes all labels in current repository"),
        )
}
