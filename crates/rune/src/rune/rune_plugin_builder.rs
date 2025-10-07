// use super::super::run_options::Plugin as PluginOptions;
use super::{PluginOptions, RunMode, RunePlugin, WorkflowsMode};

#[derive(Default)]
pub struct RunePluginBuilder {
    run_mode: RunMode,
    workflows_mode: WorkflowsMode,
}

impl RunePluginBuilder {
    pub fn minimal(&mut self, minimal: bool) -> &mut Self {
        if minimal {
            self.run_mode = RunMode::Minimal
        };
        self
    }
    pub fn plugin_only(&mut self, plugin: Option<PluginOptions>) -> &mut Self {
        if plugin.is_some() {
            let plugin = plugin.unwrap();
            self.workflows_mode = WorkflowsMode::Plugin(PluginOptions {
                path: plugin.path,
                args: plugin.args,
            });
        }
        self
    }
    pub fn build(&self) -> RunePlugin {
        RunePlugin {
            run_mode: self.run_mode.clone(),
            workflows_mode: self.workflows_mode.clone(),
        }
    }
}
