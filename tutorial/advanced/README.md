# advanced

将关键字 match 用作函数标识符。你可以使用原始标识符将 match 作为函数名称使用：
原始标识符允许使用你选择的任何单词作为标识符，即使该单词恰好是保留关键字。

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

assert!(r#match("foo", "foobar"));
```

```log
r"...", r#"..."#, r##"..."##, etc.	原始字符串字面值，未处理的转义字符
b"..."	字节字符串字面值; 构造一个字节数组类型而非字符串
br"...", br#"..."#, br##"..."## 等	原始字节字符串字面值，原始和字节字符串字面值的结合
'...'	字符字面值
b'...'	ASCII 码字节字面值

//	行注释
//!	内部行文档注释
///	外部行文档注释
/*...*/	块注释
/*!...*/	内部块文档注释
/**...*/	外部块文档注释
```

Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。
我们更倾向于称之为 never type。这个名字描述了它的作用：在函数从不返回的时候充当返回值。

描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型。允许 match 的分支以
continue 结束是因为 continue 并不真正返回一个值；相反它把控制权交回上层循环，所以在
Err 的情况，事实上并未对 guess 赋值。

never type 的另一个用途是 panic!。还记得 Option<T> 上的 unwrap 函数吗？它产生一个值或 panic。这里是它的定义：

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

虽然 &T 是一个储存了 T 所在的内存位置的单个值，&str 则是 两个 值：str 的地址和其长度。这样，&str 
就有了一个在编译时可以知道的大小：它是 usize 长度的两倍。也就是说，我们总是知道 &str 的大小，而无论
其引用的字符串是多长。这里是 Rust 中动态大小类型的常规用法：它们有一些额外的元信息来储存动态信息的大小。
这引出了动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。
可以将 str 与所有类型的指针结合：比如 Box<str> 或 Rc<str>

每一个 trait 都是一个可以通过 trait 名称来引用的动态大小类型
我们提到了为了将 trait 用于 trait 对象，必须将它们放入指针之后，比如 &dyn Trait 或 
Box<dyn Trait>（Rc<dyn Trait> 也可以）
