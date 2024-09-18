pub mod hasher;

use nu_plugin::Plugin;

pub struct HashesPlugin;

impl Plugin for HashesPlugin {
  fn version(&self) -> String {
    env!("CARGO_PKG_VERSION").into()
  }

  fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
    vec![]
  }
}
