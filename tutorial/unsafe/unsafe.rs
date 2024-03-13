// 在使用Unsafe Rust的代码时，需要在unsafe代码块中执行，当一个方法为不安全的时候，
// 需要将其声明为unsafe，同时调用它时也需要在unsafe中执行
// unsafe函数和方法与常规函数方法十分类似，只是其开头有一个额外的 unsafe
// unsafe函数体也是有效的 unsafe 块，所以在不安全函数中进行另一个不安全操作时无需新增额外的 unsafe 块
// 通过 unsafe 块，我们向 Rust 保证了我们已经阅读过函数的文档，理解如何正确使用，并验证过其满足函数的契约。

unsafe fn unsafe_func() {
    println!("unsafe_func");
}

unsafe fn dangerous1() {
    println!("dangerous1");
}

unsafe fn dangerous2() {
    dangerous1();
}

fn u2() {
    unsafe {
        unsafe_func();
        dangerous2();
    }
}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // 使用 as_mut_ptr 方法访问 slice 的裸指针
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // 这里使用此函数从 ptr 中创建了一个有 mid 个项的 slice
            // slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。
            // 裸指针上的 add 方法也是不安全的，因为其必须确信此地址偏移量也是有效的指针。因此必须将
            // slice::from_raw_parts_mut 和 add 放入 unsafe 块中以便能调用它们。通过观察代码，和增加
            // mid 必然小于等于 len 的断言，我们可以说 unsafe 块中所有的裸指针将是有效的 slice 中数据的指针。
            // 这是一个可以接受的 unsafe 的恰当用法。
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 之所以会有 unsafe 的特征，是因为该特征至少有一个方法包含有编译器无法验证的内容

// 可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，
// 同时 trait 的实现也必须标记为 unsafe
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// 再回到刚提到的 Send 特征，若我们的类型中的所有字段都实现了 Send 特征，那该类型也
// 会自动实现 Send。但是如果我们想要为某个类型手动实现 Send ，例如为裸指针，那么就
// 必须使用 unsafe

// 总之，Send 特征标记为 unsafe 是因为 Rust 无法验证我们的类型是否能在线程间安全的
// 传递，因此就需要通过 unsafe 来告诉编译器，它无需操心，剩下的交给我们自己来处理。

// 截止目前，我们还没有介绍过 union ，原因很简单，它主要用于跟 C 代码进行交互。
// 访问 union 的字段是不安全的，因为 Rust 无法保证当前存储在 union 实例中的数据类型。

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

// 上从可以看出，union 的使用方式跟结构体确实很相似，但是前者的所有字段都共享同一个存储空间
// 意味着往 union 的某个字段写入值，会导致其它字段的值会被覆盖。

fn main() {
    // u1();
    // u2();
}

// 由于 unsafe 和 FFI 在 Rust 的使用场景中是相当常见的(例如相对于 Go 的 unsafe 来说)，
// 因此社区已经开发出了相当一部分实用的工具，可以改善相应的开发体验。

// https://github.com/rust-lang/rust-bindgen
// https://github.com/eqrion/cbindgen/
// 其中 rust-bindgen 用于在 Rust 中访问 C 代码，而 cbindgen则反之

// 如果需要跟 C++ 代码交互，非常推荐使用 cxx，它提供了双向的调用，最大的优点就是安全
// 是的，你无需通过 unsafe 来使用它！
// https://github.com/dtolnay/cxx

// miri 可以生成 Rust 的中间层表示 MIR，对于编译器来说，我们的 Rust 代码首先会被编译为
// MIR ，然后再提交给 LLVM 进行处理。
// 可以通过 rustup component add miri 来安装它，并通过 cargo miri 来使用，同时还
// 可以使用 cargo miri test 来运行测试代码。
// https://github.com/rust-lang/miri
