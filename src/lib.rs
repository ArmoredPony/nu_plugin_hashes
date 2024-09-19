mod commands_generated;
mod hasher;
mod hashers_generated;

use nu_plugin::Plugin;

pub struct HashesPlugin;

impl Plugin for HashesPlugin {
  fn version(&self) -> String {
    env!("CARGO_PKG_VERSION").into()
  }

  fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
    commands_generated::commands()
  }
}
