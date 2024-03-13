// async fn 函数如果拥有引用类型的参数，那它返回的 Future 的生命周期就会被
// 这些参数的生命周期所限制:

// 在一般情况下，在函数调用后就立即 .await 不会存在任何问题，例如foo(&x).await。
// 但是，若 Future 被先存起来或发送到另一个任务或者线程，就可能存在问题了

use std::future::Future;

// fn bad() -> impl Future<Output = u8> {
//     let x = 5;
//     borrow_x(&x) // ERROR: `x` does not live long enough
// }

async fn borrow_x(x: &u8) -> u8 {
    *x
}

// 以上代码会报错，因为 x 的生命周期只到 bad 函数的结尾。 但是 Future 显然会活得更久

// 其中一个常用的解决方法就是将具有引用参数的 async fn 函数转变成一个具有 'static
// 生命周期的 Future 。 以上解决方法可以通过将参数和对 async fn 的调用放在同一个
// async 语句块来实现:

fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

// async 允许我们使用 move 关键字来将环境中变量的所有权转移到语句块内，就像闭包那样，
// 好处是你不再发愁该如何解决借用生命周期的问题，坏处就是无法跟其它代码实现对变量的共享:

// 多个不同的 `async` 语句块可以访问同一个本地变量，只要它们在该变量的作用域内执行
async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{my_string}");
    };

    let future_two = async {
        // ...
        println!("{my_string}");
    };

    // 运行两个 Future 直到完成
    let ((), ()) = futures::join!(future_one, future_two);
}

// 由于 `async move` 会捕获环境中的变量，因此只有一个 `async move` 语句块可以访问该变量，
// 但是它也有非常明显的好处： 变量可以转移到返回的 Future 中，不再受借用生命周期的限制
fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{my_string}");
    }
}

fn main() {
    // futures::executor::block_on(good());
    futures::executor::block_on(blocks());
    futures::executor::block_on(move_block());
}
