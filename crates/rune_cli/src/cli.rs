mod styles;

use clap::{Args, Parser, Subcommand};

#[derive(Args)]
pub struct GlobalOptions {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<std::path::PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    /// Turn off debugging information
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(styles = styles::STYLES)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[clap(flatten)]
    pub global: GlobalOptions,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage plugins
    Plugin {
        /// Plugin commands
        #[command(subcommand)]
        command: PluginCommands,
    },
}

#[derive(Subcommand)]
pub enum PluginCommands {
    /// Run a plugin
    Run {
        /// Path of the plugin to run
        path: std::path::PathBuf,

        /// Arguments to pass to the plugin
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}
