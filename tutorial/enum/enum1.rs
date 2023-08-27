// 定义枚举类型的时候可以在不同的变体后面给出自定义的类型
// 则在使用该枚举类型的不同变体时，可以将数据附加到变体上

struct StructName {}

enum EnumName {
    NUMBER0,                // 不带附带信息
    NUMBER1(String),        // 单值类型
    NUMBER2(i32, i32, i32), // 元组等多只类型
    NUMBER3(StructName),    // 结构体等复合类型
    NUMBER4 { x: i32, y: i32 }, // 匿名结构体(注意没有圆括号包着)
                            // 甚至可以嵌入其他枚举类型
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 为枚举类型定义方法
impl IpAddrKind {
    fn local_host(&self) -> Self {
        Self::V4(127, 0, 0, 1)
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    println!("{:?}", four.local_host());
    let six = IpAddrKind::V6(String::from("::1"));
}
