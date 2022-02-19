use clap::{crate_version, Command};

pub(crate) const UPDATE_CMD: &str = "update";

pub(crate) fn update() -> Command<'static> {
    Command::new(UPDATE_CMD)
        .version(crate_version!())
        .about("Update all labels in current repository")
}
