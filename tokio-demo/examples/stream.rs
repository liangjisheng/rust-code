// Stream Trait用于读操作，它模拟Rust标准库的Iterator，
// 可进行迭代式读取和迭代式操作，非常具有Rust的风味

use tokio_stream::{self as stream, StreamExt};

async fn s1() {
    // 目前不能对Stream执行for value in stream{}的迭代操作，
    // 只能不断显式地调用next()方法来读取。比如可以使用下面两种循环读取的方式
    let mut stream = stream::iter(vec![0, 1, 2]);
    while let Some(value) = stream.next().await {
        println!("Got {}", value);
    }

    let mut stream = stream::iter(vec![0, 1, 2]);
    loop {
        match stream.next().await {
            Some(value) => println!("Got {}", value),
            None => break,
        }
    }
}

async fn vec_stream() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut stream = stream::iter(vec);

    while let Some(num) = stream.next().await {
        print!("{} ", num);
    }
}

async fn stream_map() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut stream = stream::iter(vec).map(|x| x * 2);

    while let Some(num) = stream.next().await {
        print!("{} ", num);
    }
}

async fn stream_filter() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut stream = stream::iter(vec).filter(|x| *x > 5);

    while let Some(num) = stream.next().await {
        print!("{} ", num);
    }
}

// 使用take方法限制只输出前 3 个数字

async fn stream_take() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut stream = stream::iter(vec).take(3);

    while let Some(num) = stream.next().await {
        print!("{} ", num);
    }
}

async fn stream_fold() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = stream::iter(vec).fold(0, |acc, x| acc + x).await;

    println!("{}", sum);
}

async fn stream_zip() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![6, 7, 8, 9, 10];
    let mut stream = vec1.iter().zip(vec2.iter());

    while let Some((num1, num2)) = stream.next() {
        println!("{} {}", num1, num2);
    }

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().zip(a2.iter());

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);
}

#[tokio::main]
async fn main() {
    // s1().await;
    // vec_stream().await;
    // stream_map().await;
    // stream_filter().await;
    // stream_take().await;
    // stream_fold().await;
    stream_zip().await;
}
