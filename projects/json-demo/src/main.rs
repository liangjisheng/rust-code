use serde::{Deserialize, Serialize};
use serde_json::Value;

static JSON_STR: &str = r#"
{
  "article": "how to work with json in Rust",
  "author": "tdep",
  "paragraph": [
    {
      "name": "untyped"
    },
    {
      "name": "strongly typed"
    },
    {
      "name": "writing json"
    }
  ]
}
"#;

// 使用 serde Serialize, Deserialize 使结构能够被序列化和反序列化
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn read_json() {
    let parsed: Value = serde_json::from_str(JSON_STR).unwrap();
    println!(
        "\nThe title of the article is {}",
        parsed["paragraph"][0]["name"]
    );
    println!("The title of the article is {}", parsed["article"])
}

fn read_json_typed() {
    let parsed: Article = serde_json::from_str(JSON_STR).unwrap();
    println!(
        "\n The name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
}

fn write_json() {
    let article: Article = Article {
        article: String::from("how to work with json in Rust"),
        author: String::from("tdep"),
        paragraph: vec![
            Paragraph {
                name: String::from("untyped"),
            },
            Paragraph {
                name: String::from("strongly typed"),
            },
            Paragraph {
                name: String::from("writing json"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    let json_pretty = serde_json::to_string_pretty(&article).unwrap();
    println!("the JSON is: {}", json);
    println!("the JSON is: {}", json_pretty);
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

// #[serde(skip_serialize)] 在序列化时忽略该字段
// #[serde(skip_deserialize)] 在反序列化时忽略该字段
// #[serde(skip)] 同时忽略这个字段
#[derive(Serialize, Deserialize, Debug)]
struct Point1 {
    x: i32,
    #[serde(skip_serializing)]
    // 序列化的时候忽略 y, 可是这样以后，结构体在反序列化时会造成运行时错误
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Point2 {
    x: f64,
    // 提供默认值
    #[serde(skip_serializing, default)]
    y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Point3 {
    x: f64,
    // 手动设置提供值的函数
    #[serde(skip_serializing, default = "default_y")]
    y: f64,
}

// 重命名字段
#[derive(Debug, Serialize, Deserialize)]
struct Point4 {
    #[serde(rename = "X")]
    x: f64,
    #[serde(rename = "Y")]
    y: f64,
}

fn default_y() -> f64 {
    5.0
}

fn json_point() {
    let point = Point { x: 1, y: 2 };
    let json: String = serde_json::to_string(&point).unwrap();
    println!("{}", json);

    let point: Point = serde_json::from_str(&json).unwrap();
    println!("{:?}", point);
    // # 将输出格式化
    println!("{:#?}", point);
}

#[derive(Debug, Serialize, Deserialize)]
enum Week {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn json_week() {
    let json: String = serde_json::to_string(&Week::Friday).unwrap();
    println!("{}", json);

    let week: Week = serde_json::from_str(&json).unwrap();
    println!("{:#?}", week);
}

// 添加一个 serde(tag) 宏
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "WeekDay")]
enum Week1 {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn json_week1() {
    let json: String = serde_json::to_string(&Week1::Friday).unwrap();
    println!("{}", json);

    let week: Week1 = serde_json::from_str(&json).unwrap();
    println!("{:#?}", week);
}

#[derive(Debug, Serialize, Deserialize)]
enum IP {
    IPv4(String),
    IPv6(String),
}

fn json_ip() {
    let json: String = serde_json::to_string(&IP::IPv4("127.0.0.1".to_string())).unwrap();
    println!("{}", json);

    let ip: IP = serde_json::from_str(&json).unwrap();
    println!("{:#?}", ip);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "ip")]
enum IP1 {
    IPv4(String),
    IPv6(String),
}

fn json_ip1() {
    let json: String = serde_json::to_string(&IP1::IPv4("127.0.0.1".to_string())).unwrap();
    println!("{}", json);

    let ip: IP1 = serde_json::from_str(&json).unwrap();
    println!("{:#?}", ip);
}

#[derive(Debug, Serialize, Deserialize)]
struct UnitStruct;

fn json_struct() {
    let json: String = serde_json::to_string(&UnitStruct).unwrap();
    println!("{}", json);

    let n: UnitStruct = serde_json::from_str(&json).unwrap();
    println!("{:#?}", n);
}

use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    //age2 is error on purpose
    let data = r#"
        {
            "name": "John Doe",
            "age2": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // ? 表示有错误就直接返回了
    let p: Person = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

fn main() {
    // read_json();
    // read_json_typed();
    // write_json();

    // json_point();
    // json_week();
    // json_week1();
    // json_ip();
    // json_ip1();
    // json_struct();

    match typed_example() {
        Ok(_) => println!("program ran ok"),
        Err(e) => println!("program ran with error {:?}", e),
    }
}
