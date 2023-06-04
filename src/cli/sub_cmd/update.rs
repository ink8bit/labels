use clap::Command;

pub(crate) const UPDATE_CMD: &str = "update";

pub(crate) fn update() -> Command {
    Command::new(UPDATE_CMD).about("Update all labels in current repository")
}
