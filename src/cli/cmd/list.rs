use clap::{crate_version, Command};

pub(crate) const LIST_CMD: &str = "list";

pub(crate) fn list() -> Command<'static> {
    Command::new(LIST_CMD)
        .version(crate_version!())
        .about("Prints labels in current repository (first 100 items)")
}
