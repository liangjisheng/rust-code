// 读写锁有几种不同的设计方式：

// 读锁优先: 只要有读操作申请锁，优先将锁分配给读操作。这种方式可以提供非常好的并发能力，但是大量的读操作可能会长时间阻挡写操作
// 写锁优先: 只要有写操作申请锁，优先将锁分配给写操作。这种方式可以保证写操作不会被饿死，但会严重影响并发能力

use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let lock = RwLock::new(5);

    // 多个读锁共存
    {
        // read()返回RwLockReadGuard
        let r1 = lock.read().await;
        let r2 = lock.read().await;
        assert_eq!(*r1, 5); // 对Guard解引用，即可得到其内部的值
        assert_eq!(*r2, 5);
    } // 读锁(r1, r2)在此释放

    // 只允许一个写锁存在
    {
        // write()返回RwLockWriteGuard
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    } // 写锁(w)被释放
}
