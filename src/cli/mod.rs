mod commands;

use clap::{arg, command, value_parser, ArgAction};
use std::path::PathBuf;


fn get_clap_command() -> clap::Command {
    command!()
        .arg(
            arg!(-f --file <FILE> "project file")
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .default_value(".kanban")
        )
        .subcommand(
            command!("init")
                .about("initializes a kanban project in the current directory")
                .arg(arg!(-n --name "name").action(ArgAction::SetTrue))
                .arg(arg!(-d --description "description").action(ArgAction::SetTrue))
                .arg(arg!(-a --author "author").action(ArgAction::SetTrue))
                .arg(arg!(-y --yes "yes").action(ArgAction::SetTrue))
        )
        .subcommand(
            command!("project")
                .about("project operations")
        )
        .subcommand(
            command!("board")
                .about("board operations")
                .subcommand(command!("add").about("add a new board"))
                .subcommand(command!("list").about("list boards"))
                .subcommand(command!("select").about("select a board"))
                .subcommand(command!("update").about("update a board"))
                .subcommand(command!("remove").about("remove a board"))
        )
        .subcommand(
            command!("task")
                .about("task operations")
                .subcommand(
                    command!("add")
                        .about("add a new task to the selected board")
                )
                .subcommand(
                    command!("list")
                        .about("list tasks of selected board")
                )
                .subcommand(
                    command!("move")
                        .about("move a task to another column")
                        .arg(
                            arg!(<id> "the id or name of the task")
                                .action(ArgAction::SetTrue)
                        )
                        .arg(
                            arg!(<column> "the column's id or name where the tasks will be moved")
                                .action(ArgAction::SetTrue)
                        )
                )
                .subcommand(
                    command!("update")
                        .about("update a task")
                        .arg(arg!(<id> "the id or name of the task").action(ArgAction::SetTrue))
                )
                .subcommand(
                    command!("remove")
                        .about("remove a task from the selected board")
                )
        )
}

// optimize
fn print_help() -> std::io::Result<()> {
    get_clap_command().print_help()
}

pub fn execute_cli() -> std::io::Result<()> {
    let clap_command = get_clap_command();
    let matches = clap_command.get_matches();
    let project_file_path = matches
            .get_one::<PathBuf>("file")
            .expect("invalid project file");

    if let Some(_) = matches.subcommand_matches("project") {
        commands::program::execute(project_file_path)
    } else {
        print_help()
    }
}
