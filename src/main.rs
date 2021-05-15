mod cli;

fn main() {
    let args = cli::args();
    let list = args.is_present("list");
    let update = args.is_present("update");

    dbg!(&list);
    dbg!(&update);
}
