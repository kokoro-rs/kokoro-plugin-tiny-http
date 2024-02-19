use http::{HTTPVersion, Header, Method, Response};
use kokoro::prelude::*;
use std::{io::Read, net::SocketAddr, ops::Deref, sync::RwLock};
pub use tiny_http as http;
use tiny_http::Request;
#[derive(Event)]
pub struct HttpRequest {
    subject: RwLock<Option<Request>>,
    pub secure: bool,
    pub method: Method,
    pub url: String,
    pub headers: Vec<Header>,
    pub http_version: HTTPVersion,
    pub body_length: Option<usize>,
    pub remote_addr: Option<SocketAddr>,
}
impl Deref for HttpRequest {
    type Target = RwLock<Option<Request>>;
    fn deref(&self) -> &Self::Target {
        &self.subject
    }
}
impl HttpRequest {
    pub fn try_response<F, R>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&Request) -> Response<R>,
        R: Read,
    {
        if let Ok(mut req) = self.try_write() {
            if let Some(req) = req.take() {
                let res = f(&req);
                req.respond(res)?;
            }
        }
        Ok(())
    }
    pub fn take(&self) -> Option<Request> {
        self.write().ok()?.take()
    }
    pub fn new(req: Request) -> Self {
        Self {
            secure: req.secure().clone(),
            method: req.method().clone(),
            url: req.url().to_string(),
            headers: req.headers().iter().map(|h| h.clone()).collect(),
            http_version: req.http_version().clone(),
            body_length: req.body_length().clone(),
            remote_addr: req.remote_addr().map(|a| a.clone()),
            subject: RwLock::new(Some(req)),
        }
    }
}


