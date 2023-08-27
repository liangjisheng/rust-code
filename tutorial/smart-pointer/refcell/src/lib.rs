pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Cannot borrow immutable local variable `self.sent_messages` as mutable
            // 错误是因为 send 方法获取了 self 的不可变引用。我们也不能参考错误文本的建议使用
            // &mut self 替代，因为这样 send 的签名就不符合 Messenger trait 定义中的签名了

            // 这正是内部可变性的用武之地！我们将通过 RefCell 来储存 sent_messages
            // 然后 send 将能够修改 sent_messages 并储存消息
            // self.sent_messages.push(String::from(message));
        }
    }

    // #[test]
    // fn it_sends_an_over_75_percent_warning_message() {
    //     let mock_messenger = MockMessenger::new();
    //     let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    //
    //     limit_tracker.set_value(80);
    //
    //     assert_eq!(mock_messenger.sent_messages.len(), 1);
    // }

    use std::cell::RefCell;

    struct MockMessenger1 {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger1 {
        fn new() -> MockMessenger1 {
            MockMessenger1 {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger1 {
        fn send(&self, message: &str) {
            // borrow_mut 方法来获取 RefCell 中值的可变引用
            self.sent_messages.borrow_mut().push(String::from(message));

            // 如果是下面这样, 编译时不会有问题，运行时会 panic
            // RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();
            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger1::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // borrow() 获取 RefCell 中值的不可变引用
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
