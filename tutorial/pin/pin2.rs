use std::marker::PhantomPinned;
use std::pin::{pin, Pin};

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征 !Unpin
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

fn main() {
    let mut test1: Test = Test::new("hello");
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
    std::mem::swap(test1.get_mut(), test2.get_mut());
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}

// 代码中使用了一个标记类型 PhantomPinned 将自定义结构体 Test 变成了 !Unpin
// (编译器会自动帮我们实现)，因此该结构体无法再被移动。一旦类型实现了 !Unpin
// 那将它的值固定到栈上就是不安全的行为，因此在代码中我们使用了 unsafe 语句块来
// 进行处理，这里也可以使用 pin_utils 来避免 unsafe 的使用。此时，再去尝试
// 移动被固定的值，就会导致编译错误
