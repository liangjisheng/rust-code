// 为你的结构体实现一些例如Debug, Default等通用的 Traits

#[derive(Debug, Default)]
struct SomeStruct {
    inner: Option<Box<SomeStruct>>,
}

fn main() {
    let nested_struct = SomeStruct {
        inner: Some(Box::new(SomeStruct {
            inner: Some(Box::new(SomeStruct { inner: None })),
        })),
    };

    // 连缀在一起的打印
    println!("{:?}", nested_struct);
    // 格式化的打印
    println!("{:#?}", nested_struct);

    // 如果我们还想看到我们打印的位置和变量的名称，那么可以使用dbg!宏来打印
    dbg!(nested_struct);

    // 使用默认值来初始化这个变量
    let nested_struct = SomeStruct {
        ..Default::default()
    };
    dbg!(nested_struct);
}
