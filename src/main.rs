use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_hashes::HashesPlugin;

fn main() {
  serve_plugin(&HashesPlugin, MsgPackSerializer);
}
