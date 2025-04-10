mod project;

use clap::{arg, command, ArgAction};
use project::Project;

fn main() -> std::io::Result<()> {
    let matches = command!()
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
                        .arg(arg!(<id> "the id or name of the task").action(ArgAction::SetTrue))
                        .arg(arg!(<column> "the column's id or name where the tasks will be moved").action(ArgAction::SetTrue))
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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("project") {
        let project = Project::load(".kanban")?;
        println!("project name {}", project.name);
        println!("project description {}", project.description);
        println!("project author {}", project.author);
    }

    Ok(())
}
