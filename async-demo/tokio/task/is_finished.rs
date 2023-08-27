// 可使用 JoinHandle 的 is_finished() 方法来判断任务是否已终止，它是非阻塞的

#[tokio::main]
async fn main() {
    let task = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    });

    // 立即输出 false
    println!("1 {}", task.is_finished());
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    // 输出 true
    println!("2 {}", task.is_finished());
}
