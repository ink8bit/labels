use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};

pub(crate) fn args() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .about("Shows labels in current repository (first 100 items)")
                .conflicts_with("update"),
        )
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .about("Updates all labels in current repository")
                .conflicts_with("list"),
        )
        .get_matches()
}
