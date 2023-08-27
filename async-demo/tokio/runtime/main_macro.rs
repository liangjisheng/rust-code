// 通过#[tokio::main]注解(annotation)，使得async main自身成为一个async runtime

use tokio;

// #[tokio::main(flavor = "multi_thread"] // 等价于#[tokio::main]
// #[tokio::main(flavor = "multi_thread", worker_threads = 10))]
// #[tokio::main(worker_threads = 10))]

// 创建单一线程的main runtime
// #[tokio::main(flavor = "current_thread")]

// 创建的是多线程 runtime
#[tokio::main]
async fn main() {}

// 等价于如下没有使用#[tokio::main]的代码
// fn main(){
//     tokio::runtime::Builder::new_multi_thread()
//         .worker_threads(N)
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(async { ... });
// }
