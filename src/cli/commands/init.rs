use std::path::PathBuf;
use dialoguer::Input;
use crate::project::Project;

pub struct PromptOutput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
}

/// opens a command line prompt for the user to input data specific to the project
pub fn prompt(name: bool, description: bool, author: bool) -> PromptOutput {
    let mut output = PromptOutput {
        name: None,
        description: None,
        author: None,
    };

    if name {
        output.name = Some(Input::<String>::new()
            .with_prompt("project name")
            .interact_text()
            .unwrap());
    }
    if description {
        output.description = Some(Input::<String>::new()
            .with_prompt("description")
            .interact_text()
            .unwrap());
    }
    if author {
        output.author = Some(Input::<String>::new()
            .with_prompt("author")
            .interact_text()
            .unwrap());
    }

    output
}

/// creates the *kanban* project in the given directory
pub fn initialize(
    project_file_path: &PathBuf,
    name: String,
    description: String,
    author: String,
) -> std::io::Result<()> {
    let project = Project::new(
        name,
        description,
        author
    );
    project.store(project_file_path.to_str().unwrap())
}