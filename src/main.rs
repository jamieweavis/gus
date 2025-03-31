use colored::Colorize;
use config::User;

mod cli;
mod config;
mod git;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run() -> Result<()> {
    git::verify_installed()?;

    let user = User {
        name: git::get_config("user.name")?,
        email: git::get_config("user.email")?,
    };
    let config = config::init(user)?;
    cli::parse(&config)?;

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        cli::print_help();
        eprintln!("\n{} {}", "Error:".red(), error.to_string().red());
        std::process::exit(1);
    }

    std::process::exit(0);
}
