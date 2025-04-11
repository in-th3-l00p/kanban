/*
 states of the initial command and its subcommands
*/

pub enum Command {
    Init,
    Project,
    Board(BoardCommand),
    Task(TaskCommand),
    Help // undefined
}

pub enum BoardCommand {
    Add,
    List,
    Select,
    Update,
    Remove,
}

pub enum TaskCommand {
    Add,
    List,
    Move,
    Select,
    Update,
    Remove,
}
