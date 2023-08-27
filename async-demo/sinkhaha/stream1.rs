// futures 库提供了一些基本的生成 Stream 的方法，除了上面用到的 iter 方法，还有：
// empty()：生成一个空的 Stream
// once()：生成一个只包含单个值的 Stream
// pending()：生成一个不包含任何值，只返回 Poll::Pending 的 Stream
// repeat()：生成一个一直返回相同值的 Stream
// repeat_with()：通过闭包函数无穷尽地返回数据的 Stream
// poll_fn()：通过一个返回 Poll> 的闭包来产生 Stream
// unfold()：通过初始值和返回 Future 的闭包来产生 Stream

use futures::prelude::*;

#[tokio::main]
async fn main() {
    let mut st = stream::iter(1..10)
        .filter(|x| future::ready(x % 2 == 0))
        .map(|x| x * x);

    while let Some(x) = st.next().await {
        println!("Got item: {}", x);
    }
}
