use std::path::PathBuf;
use crate::project::Project;

pub fn execute(project_file_path: &PathBuf) -> std::io::Result<()> {
    let project = Project::load(project_file_path)?;
    println!("project name {}", project.name);
    println!("project description {}", project.description);
    println!("project author {}", project.author);

    Ok(())
}