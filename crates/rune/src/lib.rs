pub(crate) mod my_debug;
pub(crate) mod my_minimal;
pub(crate) mod rune;
pub(crate) mod rune_plugin_runner;

use bevy::app::{App, AppExit};
pub use rune::{PluginOptions, RunePlugin};

pub struct RunOptions {
    pub minimal: bool,
    pub plugin: Option<PluginOptions>,
}

pub fn run(options: RunOptions) -> AppExit {
    let rune_plugin = RunePlugin::builder()
        .minimal(options.minimal)
        .plugin_only(options.plugin)
        .build();

    App::new().add_plugins(rune_plugin).run()
}
