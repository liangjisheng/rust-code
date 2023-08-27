// drop trait 用于在值超出范围时释放文件或网络连接等资源
// drop trait 用于释放 Box <T> 指向的堆上的空间
// drop trait 用于实现 drop()方法，该方法对self进行可变引用

struct Example {
    a: i32,
}

// Drop trait 包含在 prelude 中，所以无需导入它
impl Drop for Example {
    fn drop(&mut self) {
        println!("Dropping the instance of Example with data : {}", self.a);
    }
}

fn main() {
    let a1 = Example { a: 10 };
    let b1 = Example { a: 20 };

    // 当实例移出作用域时，Rust会隐式调用drop()方法来删除Example类型的实例
    // 首先，它将删除b1实例，然后删除a1实例

    // 有时，有必要在范围结束之前删除该值。如果想提前删除该值，那么使用std::mem::drop函数来删除该值
    let c = Example { a: 3 };
    // error: explicit destructor calls not allowed
    // Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，
    // 这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次
    // c.drop();

    // Rust编译器抛出一个错误，不允许显式调用drop()方法。不是显式调用drop()方法
    // 而是调用std::mem::drop函数在值超出范围之前删除它
    drop(c);

    let d = Example { a: 4 };

    println!("Instances of Example type are created");
}
