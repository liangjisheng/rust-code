// 每个分支可以有一个if前置条件，当if前置条件为false时，对应的分支将被select!忽略(禁用)，
// 但该分支的异步任务仍然会执行，只不过select!不再轮询它(即不再推进异步任务的执行)。
// 下面是官方手册对select!工作流程的描述：
// 1. 评估所有分支中存在的if前置条件，如果某个分支的前置条件返回false，则禁用该分支。
//    注意，循环(如loop)时，每一轮执行的select!都会清除分支的禁用标记
// 2. 收集所有分支中的异步表达式(包括已被禁用的分支)，并在同一个线程中推进所有未被禁用的异步任务执行，然后等待
// 3. 当某个分支的异步任务完成，将该异步任务的返回值与对应分支的模式进行匹配，如果匹配成功，
//    则执行对应分支的handler，如果匹配失败，则禁用该分支，本次select!调用不会再考虑该分支。
//    如果匹配失败，则重新等待下一个异步任务的完成
// 4. 如果所有分支最终都被禁用，则执行else分支，如果不存在else分支，则panic

// 默认情况下，select!会伪随机公平地轮询每一个分支，如果确实需要让select!按照任务书写顺序去轮询
// 可以在 select! 中使用 biased

use tokio;

#[tokio::main]
async fn main() {
    let mut count = 0u8;
    loop {
        tokio::select! {
            // 如果取消biased，挑选的任务顺序将随机，可能会导致分支中的断言失败
            biased;
            _ = async {}, if count < 1 => { count += 1; assert_eq!(count, 1); }
            _ = async {}, if count < 2 => { count += 1; assert_eq!(count, 2); }
            _ = async {}, if count < 3 => { count += 1; assert_eq!(count, 3); }
            _ = async {}, if count < 4 => { count += 1; assert_eq!(count, 4); }
            else => { break; }
        };
    }
}

// 如果注释掉biased，那么在第一轮循环中，由于select!中的4个if前置条件均为true，因此按照随机的顺序
// 推进这4个异步任务。由于上面示例中的异步任务表达式不做任何事，因此第一个被推进的异步任务会先完成，
// select! 将取消剩余3个任务，假如先完成任务的分支的断言通过，那么 select! 返回后将进入下一轮loop循环
// 重新调用一次 select! 宏，重新评估if条件，这次将只有3个分支通过检测，不通过的那个分支将被禁用
// select! 将按照随机顺序推进这3个分支
