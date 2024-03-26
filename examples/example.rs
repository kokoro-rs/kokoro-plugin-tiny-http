use kokoro::{dynamic_plugin::toml::toml, prelude::*};
use kokoro_plugin_tiny_http_event::{http::Response, *};
fn main() -> Result<()> {
    let ctx = channel_ctx();
    let pf = PluginFinder::new("./plugin");
    let plugin = pf.find("kokoro_plugin_tiny_http");
    let config = toml! {
        host = "0.0.0.0"
        port = 1145
    };
    ctx.plugin_dynamic(plugin, Some(config.into()))?;
    ctx.subscribe(hello);
    ctx.run_sync();

    Ok(())
}

path!(Hello, "/hello");
fn hello(req: PathQuery<Hello>) {
    if let Some(req) = req.take() {
        req.respond(Response::from_string("Hello World!")).unwrap();
    }
}
