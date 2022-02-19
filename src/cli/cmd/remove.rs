use clap::{crate_version, Command};

pub(crate) const REMOVE_CMD: &str = "remove";

pub(crate) fn remove() -> Command<'static> {
    Command::new(REMOVE_CMD)
        .version(crate_version!())
        .about("Removes all labels in current repository")
}
