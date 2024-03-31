// 此实例使用 GitHub API - 速率限制展示如何处理远程服务器错误。本实例使用
// hyper::header! 宏来解析响应头并检查 reqwest::StatusCode::Forbidden。
// 如果响应超过速率限制，则将等待并重试

use error_chain::error_chain;

use reqwest::StatusCode;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, UNIX_EPOCH};

error_chain! {
   foreign_links {
       Io(std::io::Error);
       Time(std::time::SystemTimeError);
       Reqwest(reqwest::Error);
   }
}

// header! { (XRateLimitLimit, "X-RateLimit-Limit") => [usize] }
// header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [usize] }
// header! { (XRateLimitReset, "X-RateLimit-Reset") => [u64] }

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        let url = "https://api.github.com/users/rust-lang-nursery";
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        let rate_limit = response
            .headers()
            .get("X-RateLimit-Limit")
            .ok_or("response doesn't include the expected X-RateLimit-Limit header")?;

        let a = rate_limit.to_str().unwrap();
        let rate_limit = usize::from_str(a).unwrap();

        let rate_remaining = response
            .headers()
            .get("X-RateLimit-Remaining")
            .ok_or("response doesn't include the expected X-RateLimit-Remaining header")?;

        let a = rate_remaining.to_str().unwrap();
        let rate_remaining = usize::from_str(a).unwrap();

        let rate_reset_at = response
            .headers()
            .get("X-RateLimit-Reset")
            .ok_or("response doesn't include the expected X-RateLimit-Reset header")?;

        let a = rate_reset_at.to_str().unwrap();
        let rate_reset_at = u64::from_str(a).unwrap();

        let rate_reset_within = Duration::from_secs(rate_reset_at) - UNIX_EPOCH.elapsed()?;

        if response.status() == StatusCode::FORBIDDEN && rate_remaining == 0 {
            println!("Sleeping for {} seconds.", rate_reset_within.as_secs());
            thread::sleep(rate_reset_within);
            return main();
        } else {
            println!(
                "Rate limit is currently {}/{}, the reset of this limit will be within {} seconds.",
                rate_remaining,
                rate_limit,
                rate_reset_within.as_secs(),
            );
            break;
        }
    }
    Ok(())
}
