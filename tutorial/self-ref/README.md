# self ref

若 T: Unpin ( Rust 类型的默认实现)，那么 Pin<'a, T> 跟 &'a mut T 完全相同，
也就是 Pin 将没有任何效果, 该移动还是照常移动

绝大多数标准库类型都实现了 Unpin ，事实上，对于 Rust 中你能遇到的绝大多数类型，
该结论依然成立 ，其中一个例外就是：async/await 生成的 Future 没有实现 Unpin

你可以通过以下方法为自己的类型添加 !Unpin 约束：

使用文中提到的 std::marker::PhantomPinned
使用nightly 版本下的 feature flag

可以将值固定到栈上，也可以固定到堆上
将 !Unpin 值固定到栈上需要使用 unsafe
将 !Unpin 值固定到堆上无需 unsafe ，可以通过 Box::pin 来简单的实现

当固定类型 T: !Unpin 时，你需要保证数据从被固定到被 drop 这段时期内，其内存不会
变得非法或者被重用