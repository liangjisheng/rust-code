enum Gender {
    Male,
    Female,
}

// 参数类型为Gender
fn is_male(g: Gender) -> bool {
    match g {
        Gender::Male => true,
        Gender::Female => false,
    }
}

// 前文定义的枚举类型，其每个成员都有对应的数值。默认第一个成员对应的数值为0，
// 第二个成员的对应的数值为1，后一个成员的数值总是比其前一个数值大1。并且，
// 可以使用=为成员指定数值，但指定值时需注意，不同成员对应的数值不能相同
enum E {
    A,      // 对应数值0
    B,      // 自动加1，对应1
    C = 33, // 对应33
    D,      // 自动加1，对应34
}

// 可使用as将enum成员转换为对应的数值
#[derive(Copy, Clone)]
enum Week {
    Monday = 1, // 1
    Tuesday,    // 2
    Wednesday,  // 3
    Thursday,   // 4
    Friday,     // 5
    Saturday,   // 6
    Sunday,     // 7
}

// 和Struct类型一样，也可以使用impl关键字为枚举类型定义方法
impl Week {
    fn is_weekend(&self) -> bool {
        if (*self as u8) > 5 {
            return true;
        }
        false
    }
}

// 可在enum定义枚举类型的前面使用#[repr]来指定枚举成员的数值范围，超出范围后将编译错误
// 当不指定类型限制时，Rust尽量以可容纳数据大小的最小类型。例如，最大成员值为100，
// 则用一个字节的u8类型，最大成员值为500，则用两个字节的u16
// 最大数值不能超过255
#[repr(u8)] // 限定范围为`0..=255`
enum E1 {
    A,
    B = 254,
    C,
    D, // 256，超过255，编译报错
}

fn main() {
    // 可传递Gender已枚举的值作为参数
    assert!(is_male(Gender::Male));
    assert!(!is_male(Gender::Female));

    // mon等于1
    let mon = Week::Monday as i32;
    println!("{}", mon);

    let mut d = Week::Thursday;
    println!("{}", d.is_weekend());
    d = Week::Saturday;
    println!("{}", d.is_weekend());
}

// enum创建枚举类型有多种方式，其每个成员的定义都类似于创建Struct结构的语法
enum E2 {
    F1,                    // 该成员类似于unit-like struct
    F2(i32, u64),          // 该成员类似于tuple struct
    F3 { x: i32, y: u64 }, // 该成员类似于named struct
}
