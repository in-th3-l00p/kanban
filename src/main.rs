mod project;
mod cli;

use project::Project;
use cli::commands::{Command};

fn main() -> std::io::Result<()> {
    let program_state = cli::get_program_state();

    match program_state.command {
        Command::Project => {
            let project = Project::load(program_state.project_file_path)?;
            println!("project name {}", project.name);
            println!("project description {}", project.description);
            println!("project author {}", project.author);
        },
        _ => cli::print_help()?
    }

    Ok(())
}
