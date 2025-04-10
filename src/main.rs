use clap::{arg, command, ArgAction};

fn main() {
    let matches = command!()
        .subcommand(
            command!("init")
                .about("initializes a kanban project in the current directory")
                .arg(arg!(-n --name "name").action(ArgAction::SetTrue))
                .arg(arg!(-d --description "description").action(ArgAction::SetTrue))
                .arg(arg!(-a --author "author").action(ArgAction::SetTrue))
                .arg(arg!(-y --yes "yes").action(ArgAction::SetTrue))
        )
        .get_matches();
}
