// trait Subtrait: Supertrait {}
// 上面的代码是一种语法糖，展开来应该是
// trait Subtrait where Self: Supertrait {}

// 这是一个微妙而重要的区别，要明白约束在Self上，也就是实现Subtrait的类型而非Subtrait自身。
// 后者也没有意义，因为 trait 约束只能作用于能够实现 trait 的具体类型，trait 本身不能实现其他的 trait

mod m1 {
    trait Supertrait {
        fn method(&self) {
            println!("in supertrait");
        }
    }

    trait Subtrait: Supertrait {
        // this looks like it might impl or
        // override Supertrait::method but it
        // does not
        fn method(&self) {
            println!("in subtrait")
        }
    }

    struct SomeType;

    // adds Supertrait::method to SomeType
    impl Supertrait for SomeType {}

    // adds Subtrait::method to SomeType
    impl Subtrait for SomeType {}

    // both methods exist on SomeType simultaneously
    // neither overriding or shadowing the other

    pub fn t1() {
        // SomeType.method(); // ambiguous method call

        let st = SomeType;
        // must disambiguate using fully-qualified syntax
        <SomeType as Supertrait>::method(&st); // prints "in supertrait"
        <SomeType as Subtrait>::method(&st); // prints "in subtrait"
    }
}

// 对于一个类型如何同时实现一个 subtrait 和一个 supertrait，也没有明确的规则。
// 它可以在另一个类型的实现中实现其他的方法
mod m2 {
    trait Supertrait {
        fn super_method(&mut self);
    }

    trait Subtrait: Supertrait {
        fn sub_method(&mut self);
    }

    struct CallSuperFromSub;

    impl Supertrait for CallSuperFromSub {
        fn super_method(&mut self) {
            println!("in super");
        }
    }

    impl Subtrait for CallSuperFromSub {
        fn sub_method(&mut self) {
            println!("in sub");
            self.super_method();
        }
    }

    struct CallSubFromSuper;

    impl Supertrait for CallSubFromSuper {
        fn super_method(&mut self) {
            println!("in super");
            self.sub_method();
        }
    }

    impl Subtrait for CallSubFromSuper {
        fn sub_method(&mut self) {
            println!("in sub");
        }
    }

    struct CallEachOther(bool);

    impl Supertrait for CallEachOther {
        fn super_method(&mut self) {
            println!("in super");
            if self.0 {
                self.0 = false;
                self.sub_method();
            }
        }
    }

    impl Subtrait for CallEachOther {
        fn sub_method(&mut self) {
            println!("in sub");
            if self.0 {
                self.0 = false;
                self.super_method();
            }
        }
    }

    pub fn t2() {
        CallSuperFromSub.super_method(); // prints "in super"
        CallSuperFromSub.sub_method(); // prints "in sub", "in super"

        CallSubFromSuper.super_method(); // prints "in super", "in sub"
        CallSubFromSuper.sub_method(); // prints "in sub"

        CallEachOther(true).super_method(); // prints "in super", "in sub"
        CallEachOther(true).sub_method(); // prints "in sub", "in super"
    }
}

fn main() {
    // m1::t1();
    m2::t2();
}
