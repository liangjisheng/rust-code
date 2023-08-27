use futures::executor::block_on;
use std::future::Future;

// 运行 cargo run --example async-test2
fn main() {
    let name1 = "alice".to_string();
    let name2 = "bob".to_string();

    // Future 除了可以用 await 来执行外，还可以直接用 executor 执行
    block_on(say_hello1(&name1));
    block_on(say_hello2(&name2));
}

async fn say_hello1(name: &str) -> usize {
    println!("Hello {}", name);
    42
}

// async fn 关键字相当于一个返回 impl Future<Output> 的语法糖
fn say_hello2<'fut>(name: &'fut str) -> impl Future<Output = usize> + 'fut {
    async move {
        println!("Hello {}", name);
        42
    }
}