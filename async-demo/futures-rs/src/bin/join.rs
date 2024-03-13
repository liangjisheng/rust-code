use futures::channel::mpsc;
use futures::future::TryFutureExt;
use futures::future::{join, join_all, try_join, try_join_all};
use futures::StreamExt;
use futures::{SinkExt, Stream};
use std::io;
use std::pin::Pin;

#[derive(Debug)]
struct Book;

#[derive(Debug)]
struct Music;

async fn enjoy_book_and_music() -> (Book, Music) {
    let book = enjoy_book().await;
    let music = enjoy_music().await;
    (book, music)
}

// Rust 中的 Future 是惰性的，直到调用 .await 时，才会开始运行。而那两个
// await 由于在代码中有先后顺序，因此它们是顺序运行的。

async fn enjoy_book() -> Book {
    Book
}

async fn enjoy_music() -> Music {
    Music
}

// 为了正确的并发运行两个 Future ， 我们来试试 futures::join! 宏

async fn enjoy_book_and_music1() -> (Book, Music) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();
    join(book_fut, music_fut).await
}

// 由于 join! 必须等待它管理的所有 Future 完成后才能完成，如果你希望在某一个
// Future 报错后就立即停止所有 Future 的执行，可以使用 try_join!，特别是当
// Future 返回 Result 时

async fn get_book() -> Result<Book, String> {
    /* ... */
    Ok(Book)
}
async fn get_music() -> Result<Music, String> {
    /* ... */
    Ok(Music)
}

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book();
    let music_fut = get_music();
    try_join(book_fut, music_fut).await
}

// 有一点需要注意，传给 try_join! 的所有 Future 都必须拥有相同的错误类型。
// 如果错误类型不同，可以考虑使用来自 futures::future::TryFutureExt
// 模块的 map_err 和 err_info 方法将错误进行转换:

async fn get_book1() -> Result<Book, ()> {
    /* ... */
    Ok(Book)
}
async fn get_music1() -> Result<Music, String> {
    /* ... */
    Ok(Music)
}

async fn get_book_and_music1() -> Result<(Book, Music), String> {
    let book_fut = get_book1().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music1();
    try_join(book_fut, music_fut).await
}

async fn f_join() {
    // join通过两个Future参数，构造一个新的 Future
    // 这个Future等待输入的两个Future完成，返回值为包含两个结果的元组
    // try_join为join的TryFuture版本，出现错误将返回第一个遇到的错误
    async fn foo(i: u32) -> u32 {
        i
    }
    let result = join(foo(1), foo(2)).await;
    assert_eq!(result, (1, 2));

    async fn try_foo(i: u32) -> Result<u32, u32> {
        if i > 10 {
            Err(i)
        } else {
            Ok(i)
        }
    }
    let result = try_join(try_foo(1), try_foo(2)).await;
    assert_eq!(result, Ok((1, 2)));
    let result = try_join(try_foo(1), try_foo(22)).await;
    assert_eq!(result, Err(22));
}

async fn f_join_all() {
    // join_all接受多个Future，构造一个新的Future，这个Future等待输入的所以Future完成，返回值为结果数组
    // try_join_all为join_all的TryFuture版本，出现错误将返回第一个遇到的错误
    async fn foo(i: u32) -> u32 {
        i
    }

    let futures = vec![foo(1), foo(2), foo(3)];
    assert_eq!(join_all(futures).await, [1, 2, 3]);

    async fn try_foo(i: u32) -> Result<u32, u32> {
        if i > 10 {
            Err(i)
        } else {
            Ok(i)
        }
    }

    let futures = vec![try_foo(1), try_foo(2), try_foo(3)];
    assert_eq!(try_join_all(futures).await, Ok(vec![1, 2, 3]));
    let futures = vec![try_foo(1), try_foo(22), try_foo(33)];
    assert_eq!(try_join_all(futures).await, Err(22));
}

fn main() {
    let output = futures::executor::block_on(enjoy_book_and_music());
    println!("{:?}", output.0);
    println!("{:?}", output.1);

    let output = futures::executor::block_on(enjoy_book_and_music1());
    println!("{:?}", output.0);
    println!("{:?}", output.1);

    let output = futures::executor::block_on(get_book_and_music()).unwrap();
    println!("{:?}", output.0);
    println!("{:?}", output.1);

    let outputResult = futures::executor::block_on(get_book_and_music1());
    let output = match outputResult {
        Err(_err) => return,
        Ok(v) => v,
    };
    println!("{:?}", output.0);
    println!("{:?}", output.1);
}
