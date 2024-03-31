use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
    // node_id: String,
    // avatar_url: String,
    // gravatar_id: String,
    // url: String,
    // html_url: String,
    // followers_url: String,
    // following_url: String,
    // gists_url: String,
    // starred_url: String,
    // subscriptions_url: String,
    // organizations_url: String,
    // repos_url: String,
    // events_url: String,
    // received_events_url: String,
    // site_admin: bool,
}

async fn g1() -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);

    let client = reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(30))
        .build()?;

    let response = client
        .get(&request_url)
        .header("User-Agent", "my-demo-app")
        .send()
        .await?;

    // println!("Response: {:?} {}", response.version(), response.status());
    // println!("Headers: {:#?}\n", response.headers());

    // let body = response.text().await?;
    // println!("Body:\n{}", body);
    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);

    Ok(())
}

async fn g2() -> Result<(), Box<dyn std::error::Error>> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(30, 0);
    let client = reqwest::ClientBuilder::new().timeout(timeout).build()?;
    // 发送 head 请求
    let response = client.head(&request_url).send().await?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // g1().await
    g2().await

    // Ok(())
}
