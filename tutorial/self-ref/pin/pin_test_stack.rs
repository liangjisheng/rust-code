use std::marker::PhantomPinned;
use std::pin::Pin;

// 将值固定到栈上, 用 Pin 来解决指针指向的数据被移动的问题

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

// 上面代码中，我们使用了一个标记类型 PhantomPinned 将自定义结构体 Test
// 变成了 !Unpin (编译器会自动帮我们实现)，因此该结构体无法再被移动。

// 一旦类型实现了 !Unpin ，那将它的值固定到栈( stack )上就是不安全的行为，
// 因此在代码中我们使用了 unsafe 语句块来进行处理，你也可以使用 pin_utils
// 来避免 unsafe 的使用。

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}

fn p1() {
    let mut test1: Test = Test::new("test1");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2: Test = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );

    // 再去尝试移动被固定的值，就会导致编译错误
    // std::mem::swap(test1.get_mut(), test2.get_mut());

    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}

fn p2() {
    let mut test1 = Test::new("test1");
    let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1_pin.as_mut());

    drop(test1_pin);
    println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

    let mut test2 = Test::new("test2");
    std::mem::swap(&mut test1, &mut test2);
    // test2 并没有调用 init, 所以这里输出 0x0
    println!("... and now it points nowhere: {:?}", test1.b);
}

fn main() {
    // p1();
    p2();
}
