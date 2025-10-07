use bevy::app::{App, Plugin, Startup};
use bevy::ecs::change_detection::Res;
use bevy::ecs::resource::Resource;
use bevy::tasks::IoTaskPool;

// use bevy::prelude::*;

#[derive(Debug, Resource)]
struct PluginPath(std::path::PathBuf);

#[derive(Debug, Resource)]
struct PluginArgs(Vec<String>);

pub struct RunePluginRunnerPlugin {
    pub path: std::path::PathBuf,
    pub args: Vec<String>,
}

impl Plugin for RunePluginRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PluginPath(self.path.clone()))
            .insert_resource(PluginArgs(self.args.clone()))
            .add_systems(Startup, run_rune_plugin);
    }
}

fn run_rune_plugin(plugin_path: Res<PluginPath>, plugin_args: Res<PluginArgs>) {
    use bevy_workflows::workflows_runner::wasi_runtime::runtime_wasmtime::component_runner::{
        ComponentRunner, WasiCtx,
    };

    let path = plugin_path.0.clone();
    let args = plugin_args.0.clone();

    IoTaskPool::get()
        .spawn(async move {
            let wasi_ctx = WasiCtx::builder()
                .inherit_stdio()
                .inherit_network()
                .args(&args)
                .build();
            let mut component_runner = ComponentRunner::new(wasi_ctx);
            component_runner.run_component_from_file(&path).await
        })
        .detach();
}
