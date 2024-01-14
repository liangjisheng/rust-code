// fn match(needle: &str, haystack: &str) -> bool {
//     haystack.contains(needle)
// }

// 将关键字 match 用作函数标识符。你可以使用原始标识符将 match 作为函数名称使用：
// 原始标识符允许使用你选择的任何单词作为标识符，即使该单词恰好是保留关键字。
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn a1() {
    // 类型别名用来创建类型同义词
    // Rust 提供了声明 类型别名（type alias）的能力，使用 type 关键字来给予现有类型另一个名字
    type Kilometers = i32;

    // 这意味着 Kilometers 是 i32 的 同义词（synonym）
    // Kilometers 不是一个新的、单独的类型。Kilometers 类型的值将被完全当作 i32 类型值来对待：

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // 类型别名的主要用途是减少重复
    // type Thunk = Box<dyn Fn() + Send + 'static>;

    // 类型别名也经常与 Result<T, E> 结合使用来减少重复。
    // type Result<T> = std::result::Result<T, std::io::Error>;
    // 因为这是一个别名，它只是另一个 Result<T, E>，这意味着可以在其上使用
    // Result<T, E> 的任何方法，以及像 ? 这样的特殊语法。
}

fn main() {
    assert!(r#match("foo", "foobar"));
    a1();
}
