// trait 作为参数

use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// impl Trait 很方便，适用于短小的例子。更长的 trait bound 则适用于更复杂的场景

// pub fn notify(item1: &impl Summary, item2: &impl Summary)

// 这适用于 item1 和 item2 允许是不同类型的情况（只要它们都实现了 Summary）
// 不过如果你希望强制它们都是相同类型呢？这只有在使用 trait bound 时才有可能

// pub fn notify<T: Summary>(item1: &T, item2: &T)

// 通过 + 指定多个 trait bound
// pub fn notify(item: &(impl Summary + Display))
// pub fn notify<T: Summary + Display>(item: &T)

// 通过 where 简化 trait bound

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    unimplemented!()
}

// 还可以像这样使用 where 从句
fn some_function1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}

// 这个函数签名就显得不那么杂乱，函数名、参数列表和返回值类型都离得很近，
// 看起来跟没有那么多 trait bounds 的函数很像

fn main() {}
