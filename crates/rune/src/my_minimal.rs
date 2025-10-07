use bevy::app::{App, Plugin, PluginGroup, ScheduleRunnerPlugin};
use std::time::Duration;

pub struct MyMinimalPlugin;
impl Plugin for MyMinimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::log::LogPlugin::default());

        app.add_plugins(bevy::MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 0.5), // Run 1 times per 2 second.
        )));

        app.world_mut()
            .get_resource_mut::<bevy::time::Time<bevy::time::Virtual>>()
            .unwrap()
            .set_max_delta(Duration::MAX);

        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.add_plugins(bevy::image::ImagePlugin::default());
    }
}
