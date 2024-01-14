# thread

[thread](https://juejin.cn/post/7210747150815707193)
[atomic](https://juejin.cn/post/7211095650130149434)

Rust 中与并发相关的内容都属于标准库，而不是语言本身的内容，但是有两个并发概念是内嵌于语言中的
std::marker 中的 Sync 和 Send trait。  
Send和Sync是 Rust 安全并发的重中之重，但是实际上它们只是标记特征
(marker trait，该类型 trait 未定义任何行为，因此非常适合用于标记), 来看看它们的作用：

Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间传送
Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。
换一种方式来说，对于任意类型 T，如果 &T（T 的不可变引用）是 Send 的话 T 就是 Sync 的，
这意味着其引用就可以安全的发送到另一个线程。

在 Rust 中，几乎所有类型都默认实现了Send和Sync，意味着一个复合类型(例如结构体), 
只要它内部的所有成员都实现了Send或者Sync，那么它就自动实现了Send或Sync的。
只要有一个成员不是Send或Sync，该复合类型就不是Send或Sync的

实现Send的类型可以在线程间安全的传递其所有权, 实现Sync的类型可以在线程间安全的共享(通过引用)
绝大部分类型都实现了Send和Sync，常见的未实现的有：裸指针、Cell、RefCell、Rc等
可以为自定义类型实现Send和Sync，但是需要unsafe代码块，必须小心维护。
可以为部分 Rust 中的类型实现Send、Sync，但是需要使用 newtype
