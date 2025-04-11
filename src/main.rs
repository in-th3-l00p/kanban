mod project;
mod cli;

fn main() -> std::io::Result<()> {
    cli::execute_cli()
}
