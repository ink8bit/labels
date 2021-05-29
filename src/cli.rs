use clap::{crate_authors, crate_description, crate_name, crate_version, App, ArgMatches};

pub(crate) fn args() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            App::new("list")
                .version(crate_version!())
                .about("Prints labels in current repository (first 100 items)"),
        )
        .subcommand(
            App::new("update")
                .version(crate_version!())
                .about("Updates all labels in current repository"),
        )
        .get_matches()
}
