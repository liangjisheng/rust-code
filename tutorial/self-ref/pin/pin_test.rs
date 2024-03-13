#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}

// Test 提供了方法用于获取字段 a 和 b 的值的引用。这里b 是 a 的一个引用，但是
// 我们并没有使用引用类型而是用了裸指针，原因是：Rust 的借用规则不允许我们这样用，
// 因为不符合生命周期的要求。 此时的 Test 就是一个自引用结构体。

// 如果不移动任何值，那么上面的例子将没有任何问题，例如:

fn p1() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b()); // test1 test1
    println!("a: {}, b: {}", test2.a(), test2.b()); // test2 test2
}

fn p2() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b()); // test1 test1
    std::mem::swap(&mut test1, &mut test2); // 交换对象内存数据
    println!("a: {}, b: {}", test2.a(), test2.b()); // test1 test2

    // 出现 test2 的原因是 swap 后 test2.b 指针指向了 test1.a，而该地址对应的值现在是 test2
}

fn p3() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b()); // test1 test1
    std::mem::swap(&mut test1, &mut test2);
    test1.a = "I've totally changed now!".to_string();
    println!("a: {}, b: {}", test2.a(), test2.b()); // test1, I've ...
}

fn main() {
    // p1();
    // p2();
    p3();
}
