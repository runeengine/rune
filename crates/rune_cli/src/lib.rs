mod cli;

use clap::Parser;
pub use cli::{Cli, Commands};

fn plugin_command(
    _cli: &Cli,
    command: &cli::PluginCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        cli::PluginCommands::Run { path, args } => {
            let path_str = path.as_os_str().to_string_lossy();
            println!("Running plugin: {}, {:?}", path_str, args);
            use rune::{PluginOptions, RunOptions};
            rune::run(RunOptions {
                minimal: false,
                plugin: Some(PluginOptions {
                    path: path.clone(),
                    args: args.clone(),
                }),
            });
        }
    }

    Ok(())
}

pub fn main() -> i32 {
    let cli = Cli::parse();

    // init_tracing(&cli.global);

    let result = match &cli.command {
        Some(cmd) => match cmd {
            Commands::Plugin { command } => plugin_command(&cli, command),
        },
        None => {
            println!("No command provided. Use --help for more information.");
            Ok(())
        }
    };

    if result.is_err() {
        eprintln!("{}", result.unwrap_err());
        std::process::exit(1);
    }

    0
}
