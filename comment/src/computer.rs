//! 计算一些你口算算不出来的复杂算术题

/// `sub_one`将指定值减 1
///
pub fn sub_one(a: i32) -> i32 {
    a - 1
}

// 文档测试中的用例还可以造成 panic
// 如果想要通过这种测试，可以添加 should_panic
// 通过 should_panic，告诉 Rust 我们这个用例会导致 panic，这样测试用例就能顺利通过

/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// comment::computer::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

// 在某些时候，我们希望保留文档测试的功能，但是又要将某些测试用例的内容从文档中隐藏起来

/// ```
/// # // 使用#开头的行会在文档中被隐藏起来，但是依然会在文档测试中运行
/// # fn try_main() -> Result<(), String> {
/// let res = comment::computer::try_div(10, 0)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() {
/// #    try_main().unwrap();
/// #
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
