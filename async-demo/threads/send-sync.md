# send sync

[post](https://juejin.cn/post/7210747150815707193)

Rust 中与并发相关的内容都属于标准库，而不是语言本身的内容，但是有两个并发概念是内嵌于语言中的
std::marker 中的 Sync 和 Send trait。  
Send和Sync是 Rust 安全并发的重中之重，但是实际上它们只是标记特征
(marker trait，该类型 trait 未定义任何行为，因此非常适合用于标记), 来看看它们的作用：

- 实现Send的类型可以在线程间安全的传递其所有权
- 实现Sync的类型可以在线程间安全的共享(通过引用，当且仅当&T是Send时，T是Sync的)

在 Rust 中，几乎所有类型都默认实现了Send和Sync，意味着一个复合类型(例如结构体), 
只要它内部的所有成员都实现了Send或者Sync，那么它就自动实现了Send或Sync的。
只要有一个成员不是Send或Sync，该复合类型就不是Send或Sync的

实现Send的类型可以在线程间安全的传递其所有权, 实现Sync的类型可以在线程间安全的共享(通过引用)
绝大部分类型都实现了Send和Sync，常见的未实现的有：裸指针、Cell、RefCell、Rc等
可以为自定义类型实现Send和Sync，但是需要unsafe代码块，必须小心维护。
可以为部分 Rust 中的类型实现Send、Sync，但是需要使用 newtype
