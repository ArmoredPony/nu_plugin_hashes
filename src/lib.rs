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
    // TODO: try to add `blake2::Blake2bVar`
    // TODO: try to add `sha1_checked::Sha1`
    // MD5 is skipped on purpose
    // SHA256 is skipped on purpose
    commands_generated::commands()
  }
}
