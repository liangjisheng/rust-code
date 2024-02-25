# type

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

## DST

读者大大们之前学过的几乎所有类型，都是固定大小的类型，包括集合 Vec、String 和 HashMap 等，而动态大
小类型刚好与之相反：编译器无法在编译期得知该类型值的大小，只有到了程序运行时，才能动态获知。对于动态类型，
我们使用 DST(dynamically sized types)或者 unsized 类型来称呼它。
因为编译器无法在编译期获知类型大小，若你试图在代码中直接使用 DST 类型，将无法通过编译

Rust 中常见的 DST 类型有: str、[T]、dyn Trait，它们都无法单独被使用，必须要通过引用或者 Box 来间接使用

## sized

```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

在上面，Rust 自动添加的特征约束 T: Sized，表示泛型函数只能用于一切实现了 Sized 特征的类型上，而所
有在编译时就能知道其大小的类型，都会自动实现 Sized 特征
每一个特征都是一个可以通过名称来引用的动态大小类型。因此如果想把特征作为具体的类型来传递给函数，你必须
将其转换成一个特征对象：诸如 &dyn Trait 或者 Box<dyn Trait> (还有 Rc<dyn Trait>)这些引用类型。

现在还有一个问题：假如想在泛型函数中使用动态数据类型怎么办？可以使用 ?Sized 特征

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

?Sized 特征用于表明类型 T 既有可能是固定大小的类型，也可能是动态大小的类型。还有一点要注意的是，函数参数
类型从 T 变成了 &T，因为 T 可能是动态大小的，因此需要用一个固定大小的指针(引用)来包裹它。
