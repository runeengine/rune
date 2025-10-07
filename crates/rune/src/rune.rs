mod rune_plugin_builder;

use super::rune_plugin_runner::RunePluginRunnerPlugin;
use rune_plugin_builder::RunePluginBuilder;

use bevy::app::{App, Plugin};
use bevy::ecs::system::Commands;

#[derive(Default)]
pub struct RunePlugin {
    run_mode: RunMode,
    workflows_mode: WorkflowsMode,
}

#[derive(Clone, Default)]
enum RunMode {
    Minimal,
    #[default]
    Normal,
}

#[derive(Clone, Default)]
enum WorkflowsMode {
    #[default]
    Normal,
    Plugin(PluginOptions),
}

#[derive(Clone)]
pub struct PluginOptions {
    pub path: std::path::PathBuf,
    pub args: Vec<String>,
}

impl RunePlugin {
    pub fn builder() -> RunePluginBuilder {
        RunePluginBuilder::default()
    }
}

impl Plugin for RunePlugin {
    fn build(&self, app: &mut App) {
        match &self.run_mode {
            RunMode::Minimal => {
                app.add_plugins(super::my_minimal::MyMinimalPlugin);
            }
            RunMode::Normal => {
                app.add_plugins(bevy::DefaultPlugins);
                app.add_systems(bevy::app::Startup, setup_camera);
            }
        }

        app.add_plugins(super::my_debug::MyDebugPlugin);

        app.add_plugins(rune_remote::RuneRemotePlugin);
        // app.add_plugins(bevy_workflows::WorkflowsPlugin);

        match &self.workflows_mode {
            WorkflowsMode::Normal => {}
            WorkflowsMode::Plugin(options) => {
                app.add_plugins(RunePluginRunnerPlugin {
                    path: options.path.clone(),
                    args: options.args.clone(),
                });
            }
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(bevy::camera::Camera2d::default());
}
