struct SelfRef<'a> {
    value: String,

    // 该引用指向上面的value
    pointer_to_value: &'a str,
}

fn main() {
    let s = "aaa".to_string();
    let v = SelfRef {
        value: s,
        // 报错，因为我们试图同时使用值和值的引用，最终所有权转移和借用一起发生了
        pointer_to_value: &s,
    };
}
