use http::{HTTPVersion, Header, Method, Response};
use kokoro::{core::query::Query, prelude::*};
use std::{
    io::Read,
    marker::PhantomData,
    net::SocketAddr,
    ops::Deref,
    sync::{Arc, RwLock},
};
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

pub trait Path {
    const P: &'static str;
}
pub struct PathQuery<P: Path> {
    event: Arc<HttpRequest>,
    _p: PhantomData<P>,
}
impl<P: Path> Deref for PathQuery<P> {
    type Target = HttpRequest;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}
impl<P: Path> Query<HttpRequest> for PathQuery<P> {
    fn create(n: Arc<HttpRequest>) -> Self {
        Self {
            event: n,
            _p: PhantomData,
        }
    }

    fn sub(n: &dyn Event) -> bool {
        if let Some(e) = n.downcast_ref::<HttpRequest>() {
            e.url == P::P
        } else {
            false
        }
    }
}
#[macro_export]
macro_rules! path {
    ($name:ident,$path:expr) => {
        #[derive(Debug)]
        struct $name;
        impl Path for $name {
            const P: &'static str = $path;
        }
    };
}
