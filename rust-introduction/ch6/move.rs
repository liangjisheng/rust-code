fn m1() {
    // 所有权的转移并不仅仅只发生在这两种相对比较明显的情况下。例如，解引用操作也需要转移所有权
    let v = &vec![11, 22];
    // move occurs because `*v` has type `Vec<i32>`, which does not implement the `Copy` trait
    // let vv = *v;

    // 当产生了一个位置，且需要向位置中放入值，就会发生移动
    // 对于上面的示例来说，&vec![11, 22]中间产生了好几个临时变量，但最终有一个临时变量是vec的所有者
    // 然后对这个变量进行引用，将引用赋值给变量v。使用*v解引用时，也产生了一个临时变量保存解引用得到的值
    // 而这里就出现了问题。因为变量v只是vec的一个引用，而不是它的所有者，它无权转移值的所有权

    let a = &"alice".to_string();
    // let b = *a; // (1).取消注释将报错
    let c = (*a).clone(); // (2).正确
    let d = &*a; // (3).正确

    let x = &3;
    let y = *x; // (4).正确

    // 注意，不要使用println!("{}", *a);或类似的宏来测试，这些宏不是函数
    // 它们真实的代码中使用的是&(*a)，因此不会发生所有权的转移
}

fn m2() {
    let x = "hello".to_string();
    // 它等价于let _tmp = x;，即将值移动给了一个临时变量
    x; // 发生Move

    // println!("{}", x); // 报错：value borrowed here after move
}

fn m3() {
    struct User {
        name: String,
    }
    let user = User {
        name: "alice".to_string(),
    };
    // let name = (&user).name; // 报错，想要移动name字段，但user正被引用着，此刻不允许移走它的一部分

    // let user1 = *(&user); // 报错，解引用临时变量时触发移动，此时user正被引用着
    let u = &user;
    let user2 = &(*u); // 不报错，解引用得到值后，对这个值创建引用，不会消耗值

    impl User {
        fn func(&self) {
            // let xx = *self; // 报错，解引用报错，self自身不是所有者，例如user.func()时，user才是所有者

            if (*self).name < "hello".to_string() {} // 不报错，比较时会转换为&((*self).name) < &("hello".to_string())
        }
    }
}

fn main() {
    // m1();
    // m2();
    m3();
}
