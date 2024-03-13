// 对于自引用结构体，三方库也有支持的
// https://github.com/joshua-maros/ouroboros
// https://github.com/jpernst/rental
// https://github.com/Kimundi/owning-ref-rs

use ouroboros::self_referencing;

#[self_referencing]
struct SelfRef {
    value: String,

    #[borrows(value)]
    pointer_to_value: &'this str,
}

fn main() {
    let v = SelfRefBuilder {
        value: "aaa".to_string(),
        pointer_to_value_builder: |value: &String| value,
    }
    .build();

    // 借用value值
    let s = v.borrow_value();
    // 借用指针
    let p = v.borrow_pointer_to_value();
    // value值和指针指向的值相等
    assert_eq!(s, *p);
}
