use clap::Command;

pub(crate) const LIST_CMD: &str = "list";

pub(crate) fn list() -> Command {
    Command::new(LIST_CMD).about("Print labels in current repository (first 100 items)")
}
