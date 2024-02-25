use serde::{Deserialize, Serialize};
use std::collections::HashMap;

async fn get_demo() {
    let resp = reqwest::get("https://httpbin.org/ip").await;
    let resp = match resp {
        Err(_e) => return,
        Ok(v) => v,
    };

    println!("{}", resp.status());
    println!("{}", resp.status().as_u16());

    // let res_bytes = match resp.bytes().await {
    //     Err(_e) => return,
    //     Ok(v) => v,
    // };
    // println!("{:?}", res_bytes);

    let res_json = match resp.json::<HashMap<String, String>>().await {
        Err(_e) => return,
        Ok(v) => v,
    };
    println!("{:?}", res_json);
}

async fn get_demo1() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("https://www.rust-lang.org").await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;
    println!("{body}");

    Ok(())
}

async fn get_demo2() {
    let url = String::from("https://httpbin.org/ip");
    // reqwest::get() is a convenience function.
    //
    // In most cases, you should create/build a reqwest::Client and reuse
    // it for all requests.
    let res = match reqwest::get(url).await {
        Err(_e) => return,
        Ok(v) => v,
    };

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = match res.text().await {
        Err(_e) => return,
        Ok(v) => v,
    };

    println!("{body}");
}

async fn post_demo() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;

    println!("{}", res.status());
    let res_json: serde_json::Value = res.json().await?;
    println!("{:?}", res_json);
    println!("{:#?}", res_json);

    Ok(())
}

async fn form_demo() -> Result<(), Box<dyn std::error::Error>> {
    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .form(&params)
        .send()
        .await?;

    println!("status {}", res.status());
    let res_body = res.text().await?;
    println!("{res_body}");

    Ok(())
}

async fn json_demo() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;

    println!("status {}", res.status());

    let body: serde_json::Value = res.json().await?;
    println!("{:?}", body);
    println!("{:#?}", body);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

async fn json_demo1() -> Result<(), Box<dyn std::error::Error>> {
    let new_post = Post {
        id: None,
        title: "Reqwest.rs".into(),
        body: "https://docs.rs/reqwest".into(),
        user_id: 1,
    };
    let new_post: Post = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

    println!("{new_post:#?}");
    // Post {
    //     id: Some(
    //         101
    //     ),
    //     title: "Reqwest.rs",
    //     body: "https://docs.rs/reqwest",
    //     user_id: 1
    // }
    Ok(())
}

async fn json_demo2() -> Result<(), Box<dyn std::error::Error>> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{echo_json:#?}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get_demo().await;
    // get_demo2().await;
    // let _ = get_demo1().await;
    // let _ = post_demo().await;
    // let _ = form_demo().await;
    // let _ = json_demo().await;
    // let _ = json_demo1().await;
    let _ = json_demo2().await;

    Ok(())
}
