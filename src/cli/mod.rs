use clap::{crate_authors, crate_description, crate_name, crate_version, Command};

pub mod sub_cmd;

use sub_cmd::{list::list, remove::remove, update::update};

pub(crate) fn app() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(list())
        .subcommand(update())
        .subcommand(remove())
}
