use std::fs::File;

use kokoro::{dynamic_plugin::toml::toml, prelude::*, core::query::EventQuery};
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
    ctx.subscribe(index);
    ctx.run_sync();

    Ok(())
}

fn index(req: EventQuery<HttpRequest>) {
    let url = &req.url;
    if let Some(req) = req.take() {
        req.respond(Response::from_file(
            File::open(format!(".{}", url)).unwrap(),
        ))
        .unwrap();
    }
}
