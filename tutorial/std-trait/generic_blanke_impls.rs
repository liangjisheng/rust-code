// 泛型覆盖实现是一种在泛型类型而不是具体类型上的实现，为了解释为什么以及如何使用它，
// 让我们从为整数类型实现一个 is_even 方法开始

mod m1 {
    trait Even {
        fn is_even(self) -> bool;
    }

    impl Even for i8 {
        fn is_even(self) -> bool {
            self % 2_i8 == 0_i8
        }
    }

    impl Even for u8 {
        fn is_even(self) -> bool {
            self % 2_u8 == 0_u8
        }
    }

    impl Even for i16 {
        fn is_even(self) -> bool {
            self % 2_i16 == 0_i16
        }
    }

    pub fn test_is_even() {
        assert!(2_i8.is_even());
        assert!(4_u8.is_even());
        assert!(6_i16.is_even());
        // etc
    }
}

// 很明显，上面的实现十分啰嗦。而且，所有我们的实现几乎都是一样的。此外，如果 Rust
// 决定在未来增加更多的整数类型，我们必须回到这段代码中，用新的整数类型来更新它。
// 我们可以通过使用泛型覆盖实现来解决所有的问题

mod m2 {
    use std::convert::TryInto;
    use std::fmt::Debug;
    use std::ops::Rem;

    trait Even {
        fn is_even(self) -> bool;
    }

    // generic blanket impl
    impl<T> Even for T
    where
        T: Rem<Output = T> + PartialEq<T> + Sized,
        u8: TryInto<T>,
        <u8 as TryInto<T>>::Error: Debug,
    {
        fn is_even(self) -> bool {
            // these unwraps will never panic
            self % 2.try_into().unwrap() == 0.try_into().unwrap()
        }
    }

    // 不同于默认实现，泛型覆盖实现提供了方法的实现，所以它们不能被重写
    // impl Even for u8 {
    //     fn is_even(self) -> bool {
    //         self % 2_u8 == 0_u8
    //     }
    // }

    // trait 一致性是指，对于任意给定的类型，最多存在某一 trait 的一个实现

    pub fn test_is_even() {
        assert!(2_i8.is_even());
        assert!(4_u8.is_even());
        assert!(6_i16.is_even());
        // etc
    }
}

fn main() {
    // m1::test_is_even();
    m2::test_is_even();
}
