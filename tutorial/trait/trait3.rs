// https://juejin.cn/post/7207783106970861629

use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("@{}发表了帖子...", self.summarize_author())
    }
}

// 实际上self是self: Self的简写，&self是self: &Self的简写，&mut self是self &mut Self的简写。
// Self代表的是当前实现了 trait 的类型，例如有一个类型 Foo 实现了 Summary trait，则实现方法时中
// 的 Self 就是Foo

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("{}发表了贴子", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}发表了贴子：{}", self.author, self.content)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}发表了微博", self.username)
    }
    fn summarize(&self) -> String {
        format!("@{}发表了微博：{}", self.username, self.content)
    }
}

fn t1() {
    let tweet = Tweet {
        username: String::from("haha"),
        content: String::from("the content"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize())
}

// Trait 可以用作函数参数
pub fn notify(item: &impl Summary) {
    // trait 参数
    println!("Breaking news! {}", item.summarize());
}

// 上面使用的 impl Trait 实际上只是一个语法糖，其完整书写形式如下，形如 T: Summary 被称为 trait 约束
pub fn notify1<T: Summary>(item: &T) {
    // trait 约束
    println!("Breaking news! {}", item.summarize());
}

fn notify_all(summaries: Vec<impl Summary>) {
    for summary in summaries {
        println!("notify: {}", summary.summarize())
    }
}

// 对于比较复杂的使用场景，特征约束可以让我们拥有更大的灵活性和语法表现能力
// 例如一个函数接受两个 impl Summary 的参数
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // trait 参数

// 泛型 T 约束：说明 item1 和 item2 必须拥有同样的类型，同时说明 T 必须实现 Summary trait
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// 通过 + 指定多个 trait bound
// pub fn notify(item: &(impl Summary + Display)) {}  // 语法糖形式
// pub fn notify<T: Summary + Display> (item: &T) {}  // trait bound 完整形式

// 当特征约束变得很多时，函数的签名就会变得很复杂，这时如果使用 where 可以对其做一些形式上的改进

// 当出现多个泛型类型时，过多的trait bound会导致函数签名难以阅读
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// 使用 where 做简化，使得函数名、参数列表和返回值类型都离得很近，
// 看起来跟没有那么多 trait bounds 的函数很像
fn some_function1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn t2() {
    let tweet = Tweet {
        username: String::from("haha"),
        content: String::from("the content"),
        reply: false,
        retweet: false,
    };
    let tweets = vec![tweet];
    notify_all(tweets);
}

fn notify3(summary: Box<dyn Summary>) {
    println!("notify: {}", summary.summarize())
}

fn notify_all3(summaries: Vec<Box<dyn Summary>>) {
    for summary in summaries {
        println!("notify: {}", summary.summarize())
    }
}

// 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候，
// 可以使用智能指针 Box 和关键字 dyn 组合的 trait 对象
fn t3() {
    let tweet = Tweet {
        username: String::from("haha"),
        content: String::from("the content"),
        reply: false,
        retweet: false,
    };
    let tweets: Vec<Box<dyn Summary>> = vec![Box::new(tweet)];
    notify_all3(tweets);
    // println!("{:?}", tweets);
}

// 实际上impl Summary是泛型编程中 Trait Bound 的语法糖，所以上面的impl trait 代码可以改写为以下形式
fn notify4<T: Summary>(summary: T) {
    println!("notify: {}", summary.summarize())
}

fn notify_all4<T: Summary>(summaries: Vec<T>) {
    for summary in summaries {
        println!("notify: {}", summary.summarize())
    }
}

fn t4() {
    let tweet = Tweet {
        username: String::from("haha"),
        content: String::from("the content"),
        reply: false,
        retweet: false,
    };
    let tweets = vec![tweet];
    notify_all4(tweets);
}

// 可以通过 impl Trait 来说明一个函数返回了一个类型，该类型实现了某个 trait
// 编译器报错，原因是 impl Trait 的返回值类型并不支持多种不同的类型返回
fn returns_summarizable(switch: bool) -> impl Summary {
    // if switch {
    //     Tweet {
    //         username: String::from("haha"),
    //         content: String::from("the content"),
    //         reply: false,
    //         retweet: false,
    //     }
    // } else {
    //     Post {
    //         title: "".to_string(),
    //         author: "".to_string(),
    //         content: "".to_string(),
    //     }
    // }
}

// 如果想要实现返回不同的类型，需要使用trait 对象
fn returns_summarizable1(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(Tweet {
            username: "".to_string(),
            content: "".to_string(),
            reply: false,
            retweet: false,
        }) // trait 对象
    } else {
        Box::new(Post {
            title: "".to_string(),
            author: "".to_string(),
            content: "".to_string(),
        }) // trait 对象
    }
}

fn main() {
    // t1();
    // t2();
    // t3();
    t4();
}
