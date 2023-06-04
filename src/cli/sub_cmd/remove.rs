use clap::Command;

pub(crate) const REMOVE_CMD: &str = "remove";

pub(crate) fn remove() -> Command {
    Command::new(REMOVE_CMD).about("Remove all labels in current repository")
}
