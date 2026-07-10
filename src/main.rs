use nu_plugin::{MsgPackSerializer, serve_plugin};

mod lib;

fn main() {
    serve_plugin(&nu_plugin_typetree::TypeTreePlugin, MsgPackSerializer);
}
