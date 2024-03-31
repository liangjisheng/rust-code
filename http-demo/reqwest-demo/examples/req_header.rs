use error_chain::error_chain;
use serde::Deserialize;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use reqwest::Client;
use std::collections::HashMap;
use url::Url;

// header! { (XPoweredBy, "X-Powered-By") => [String] }

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;

    let mut response = Client::new()
        .get(url)
        .header(USER_AGENT, "Rust-test")
        .header(AUTHORIZATION, "Bearer DEadBEEfc001cAFeEDEcafBAd")
        .header("X-Powered-By", "Guybrush Threepwood")
        .send()
        .await?;

    let binding = response.url().clone();
    let u = binding.as_str();
    let out: HeadersEcho = response.json().await?;
    assert_eq!(
        out.headers["Authorization"],
        "Bearer DEadBEEfc001cAFeEDEcafBAd"
    );
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
    assert_eq!(u, "http://httpbin.org/headers?lang=rust&browser=servo");

    println!("{:?}", out);
    Ok(())
}
