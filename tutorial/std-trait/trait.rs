mod m1 {
    pub fn t1() {
        // Trait 函数可以通过 trait 或者实现类型的命名空间来调用
        let zero: i32 = Default::default();
        println!("zero {}", zero);

        let zero = i32::default();
        println!("zero {}", zero);

        // Trait 方法可以通过在实现类型上使用点（.）操作符来调用
        let five = 5.to_string();
        println!("five {}", five);

        // trait 方法还可以像函数那样由 trait 或者实现类型通过命名空间来调用
        let five = ToString::to_string(&5);
        println!("five {}", five);
        let five = i32::to_string(&5);
        println!("five {}", five);
    }
}

mod m2 {
    // 关联类型（Associated Types）
    // Trait 可以有关联类型。当我们需要在函数签名中使用Self以外的某个类型，
    // 但是希望这个类型可以由实现者来选择而不是硬编码到 trait 声明中，这时关联类型就可以发挥作用了
    trait Trait {
        type AssociatedType;
        fn func(arg: Self::AssociatedType);
    }

    struct SomeType;
    struct OtherType;

    // any type implementing Trait can
    // choose the type of AssociatedType

    impl Trait for SomeType {
        type AssociatedType = i8; // chooses i8
        fn func(arg: Self::AssociatedType) {
            println!("arg {}", arg);
        }
    }

    impl Trait for OtherType {
        type AssociatedType = u8; // chooses u8
        fn func(arg: Self::AssociatedType) {
            println!("arg {}", arg);
        }
    }

    pub fn t2() {
        SomeType::func(-1_i8); // can only call func with i8 on SomeType
        OtherType::func(1_u8); // can only call func with u8 on OtherType
    }
}

mod m3 {
    // “泛型参数”泛指泛型类型参数（generic type parameters）、
    // 泛型生命周期参数（generic lifetime parameters）、以及泛型常量参数（generic const parameters）
    // 可以使用参数来对一个 trait 声明进行泛化（generalize ）

    // trait declaration generalized with lifetime & type parameters
    trait Trait<'a, T> {
        // signature uses generic type
        fn func1(arg: T);

        // signature uses lifetime
        fn func2(arg: &'a i32);

        // signature uses generic type & lifetime
        fn func3(arg: &'a T);
    }

    struct SomeType;

    impl<'a> Trait<'a, i8> for SomeType {
        fn func1(arg: i8) {}
        fn func2(arg: &'a i32) {
            println!("trait i8");
        }
        fn func3(arg: &'a i8) {}
    }

    impl<'b> Trait<'b, u8> for SomeType {
        fn func1(arg: u8) {}
        fn func2(arg: &'b i32) {
            println!("trait u8");
        }
        fn func3(arg: &'b u8) {}
    }

    pub fn t3() {
        let a: i8 = 0;
        let b: i32 = 32;
        SomeType::func1(a);
        <SomeType as Trait<i8>>::func2(&b);
        SomeType::func3(&a);

        let a: u8 = 1;
        let b: i32 = 32;
        SomeType::func1(a);
        <SomeType as Trait<u8>>::func2(&b);
        SomeType::func3(&a);
    }
}

mod m4 {
    // 泛型可以具有默认值，最常用的默认值是Self，但是任何类型都可以作为默认值
    // make T = Self by default
    trait Trait<T = Self> {
        fn func(t: T) {}
    }

    // any type can be used as the default
    trait Trait2<T = i32> {
        fn func2(t: T) {}
    }

    struct SomeType;

    // omitting the generic type will
    // cause the impl to use the default
    // value, which is Self here
    impl Trait for SomeType {
        fn func(t: SomeType) {}
    }

    // default value here is i32
    impl Trait2 for SomeType {
        fn func2(t: i32) {}
    }

    // the default is overridable as we'd expect
    impl Trait<String> for SomeType {
        fn func(t: String) {}
    }

    // overridable here too
    impl Trait2<String> for SomeType {
        fn func2(t: String) {}
    }

    // 除了可以对 trait 进行参数化之外，我们还可以对单个函数和方法进行参数化
    // trait Trait {
    //     fn func<'a, T>(t: &'a T);
    // }
}

fn main() {
    // m1::t1();
    // m2::t2();
    m3::t3();
}
