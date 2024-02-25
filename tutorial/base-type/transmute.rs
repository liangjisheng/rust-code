#[repr(i32)]
enum MyEnum {
    A = 1,
    B,
    C,
}

// 当你知道数值一定不会超过枚举的范围时(例如枚举成员对应 1，2，3，传入的整数也在这个范围内)，
// 就可以使用这个方法完成变形。

fn main() {
    let x = MyEnum::C;
    let y = x as i32;
    // 直接完成整数到枚举的转换
    let z: MyEnum = unsafe { std::mem::transmute(y) };

    // match the enum that came from an int
    match z {
        MyEnum::A => {
            println!("Found A");
        }
        MyEnum::B => {
            println!("Found B");
        }
        MyEnum::C => {
            println!("Found C");
        }
    }
}
