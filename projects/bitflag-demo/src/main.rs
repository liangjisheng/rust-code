// 如下实例在 bitflags! 宏的帮助下创建类型安全的位域类型 MyFlags，并为其实现基本的
// 清理操作（clear 方法）以及 Display trait。随后，展示了基本的按位操作和格式化

use bitflags::{bitflags, Flags};
use std::borrow::BorrowMut;
use std::fmt;
use std::ops::DerefMut;

bitflags! {
    #[derive(Debug, PartialEq)]
    struct MyFlags: u32 {
        const FLAG_ZERO    = 0b00000000;
        const FLAG_A       = 0b00000001;
        const FLAG_B       = 0b00000010;
        const FLAG_C       = 0b00000100;
        const FLAG_ABC     = Self::FLAG_A.bits()
                           | Self::FLAG_B.bits()
                           | Self::FLAG_C.bits();
    }
}

impl MyFlags {
    pub fn clear(&mut self) -> &mut MyFlags {
        // self.bits = 0;
        self
    }
}

impl fmt::Display for MyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits())
    }
}

fn main() {
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 | e2), MyFlags::FLAG_ABC);

    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 & e2), MyFlags::FLAG_C);

    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 - e2), MyFlags::FLAG_A);

    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!(!e2, MyFlags::FLAG_A);

    let mut flags = MyFlags::FLAG_ABC;
    assert_eq!(format!("{}", flags), "00000000000000000000000000000111");
    assert_eq!(
        format!("{}", flags.clear()),
        "00000000000000000000000000000111"
    );
    assert_eq!(format!("{:?}", MyFlags::FLAG_B), "MyFlags(FLAG_B)");
    assert_eq!(
        format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B),
        "MyFlags(FLAG_A | FLAG_B)"
    );
}
