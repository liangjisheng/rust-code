mod m1 {
    #[derive(Debug)]
    struct A {
        f1: u32,
        f2: u32,
        f3: u32,
    }

    #[derive(Debug)]
    struct B<'a> {
        f1: u32,
        a: &'a mut A,
    }

    pub fn f1() {
        let mut a: A = A {
            f1: 0,
            f2: 1,
            f3: 2,
        };
        // b不可变
        let b: B = B { f1: 3, a: &mut a };

        // error[E0594]: cannot assign to `b.a.f1`, which is behind a `&` reference
        // let b: &B = &B { f1: 3, a: &mut a };

        // 但是b中的字段a可以变
        b.a.f1 += 1;

        println!("b is {:?} ", &b);
        // 在这里，虽然变量b被设置为不可变，但是b的其中一个字段a被设置为可变的结构体，
        // 因此我们可以通过b.a.f1 += 1来修改a的值
    }
}

// 结构体可变时，里面的字段都是可变的，例如&mut a
// 结构体不可变时，里面的某个字段可以单独设置为可变，例如b.a

mod m2 {
    #[derive(Debug)]
    struct A {
        f1: u32,
        f2: u32,
        f3: u32,
    }

    #[derive(Debug)]
    struct B<'a> {
        f1: u32,
        a: &'a mut A,
    }

    impl B<'_> {
        // this will not work
        pub fn changeme(&self) {
            // self.a.f1 += 1;
        }
    }

    pub fn f2() {
        let mut a: A = A {
            f1: 0,
            f2: 1,
            f3: 2,
        };
        // b is immutable
        let b: B = B { f1: 3, a: &mut a };

        // 编译失败
        // error[E0594]: cannot assign to `self.a.f1`, which is behind a `&` reference
        b.changeme();

        println!("b is {:?} ", &b);
    }
}

// 观察第一个例子，我们调用的b.a实际上是用b的值直接调用的，在这种情况下，由于所有权规则，
// 编译器可以认定，只有一个可变引用指向了a，因此这种使用是非常安全的。

// 但是，在第二个例子中，b被藏在了&后面，根据所有权规则，同时可能存在多个b的借用，那么就
// 意味着可能会存在多个可变引用指向a,因此编译器就拒绝了这段代码。

mod m3 {
    #[derive(Debug)]
    struct A {
        f1: u32,
        f2: u32,
        f3: u32,
    }

    #[derive(Debug)]
    struct B<'a> {
        f1: u32,
        a: &'a mut A,
    }

    pub fn f3() {
        let mut a: A = A {
            f1: 0,
            f2: 1,
            f3: 2,
        };
        let b: B = B { f1: 3, a: &mut a };

        // 这里b.a.f1 += 1和a.f1 = 10只能有一个存在，否则就会报错
        b.a.f1 += 1;
        // a.f1 = 10;

        println!("b is {:?} ", &b);
    }
}

fn main() {
    m1::f1();
}

// 根据之前的观察和上面的小提示，可以得出一个结论：可变性的真正含义是你对目标对象的独占修改权。
