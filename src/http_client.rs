use std::time::Duration;

use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};

pub type HttpClient =
    Client<HttpsConnector<HttpConnector>, BoxBody<Bytes, std::convert::Infallible>>;

/// Create a HTTP 1 client instance.
///
/// Parameter: Timeout for idle sockets being kept-alive
pub fn make_client(pool_idle_timeout_seconds: u64) -> Result<HttpClient, std::io::Error> {
    let https = HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_or_http()
        .enable_http1()
        .build();

    Ok(Client::builder(TokioExecutor::new())
        .pool_idle_timeout(Duration::from_secs(pool_idle_timeout_seconds))
        .build(https))
}
