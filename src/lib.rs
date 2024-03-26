use kokoro::{default_impl::plugin::anyhow::anyhow, prelude::*};
use kokoro_plugin_tiny_http_event::*;
use serde::Deserialize;
#[derive(Deserialize, DynamicPlugin)]
pub struct TinyHttp {
    pub host: Option<String>,
    pub port: Option<u32>,
}
impl Create for TinyHttp {
    fn create(config: Option<toml::Value>) -> Result<Self> {
        if let Some(config) = config {
            Ok(Self::deserialize(config)?)
        } else {
            Ok(Self {
                host: None,
                port: None,
            })
        }
    }
}
impl Plugin for TinyHttp {
    const NAME: &'static str = "tiny-http";
    type MODE = MPSC;
    fn apply(ctx: Context<Self, Self::MODE>) -> Result<()> {
        let host = ctx.host.clone().unwrap_or("127.0.0.1".to_string());
        let port = ctx.port.as_ref().unwrap_or(&2333);
        let addr = format!("{host}:{port}");
        let client = http::Server::http(addr.clone()).or(Err(anyhow!("服务启动失败")))?;
        ctx.spawn(move |ctx, s| {
            for _ in s {
                let r = client.recv();
                if let Ok(rq) = r {
                    ctx.publish(HttpRequest::new(rq));
                }
            }
        });
        println!("Http 服务于 {addr} 启动");
        Ok(())
    }
}
