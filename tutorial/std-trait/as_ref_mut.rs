// trait AsRef<T: ?Sized> {
//     fn as_ref(&self) -> &T;
// }

// trait AsMut<T: ?Sized> {
//     fn as_mut(&mut self) -> &mut T;
// }

// AsRef 被用于轻量级的引用到引用之间的转换。然而，它最常见的一个用途是使函数在是否获取所有权上具有通用性
// AsRef 用于语义相等的事物之间引用到引用的转换

mod m1 {
    // 接受:
    //  - &str
    //  - &String
    fn takes_str(s: &str) {
        // use &str
    }

    // 接受:
    //  - &str
    //  - &String
    //  - String
    fn takes_asref_str<S: AsRef<str>>(s: S) {
        let s: &str = s.as_ref();
        // 使用 &str
    }

    fn example(slice: &str, borrow: &String, owned: String) {
        takes_str(slice);
        takes_str(borrow);
        // takes_str(owned); // ❌
        takes_asref_str(slice);
        takes_asref_str(borrow);
        takes_asref_str(owned); // ✅
    }
}

// 下面是一个好的示例，其中，我们会引入一个新类型Moderator，它只包裹了一个User并添加了特定的审核权限
mod m2 {
    struct User {
        name: String,
        age: u32,
    }

    //不幸地是，标准库并没有提供一个generic blanket impl来避免这种重复的实现
    impl AsRef<User> for User {
        fn as_ref(&self) -> &User {
            self
        }
    }

    enum Privilege {
        BanUsers,
        EditPosts,
        DeletePosts,
    }

    //尽管 Moderators 有一些特殊权限，它们仍然是普通的 User
    //并且应该做相同的事情
    struct Moderator {
        user: User,
        privileges: Vec<Privilege>,
    }

    impl AsRef<Moderator> for Moderator {
        fn as_ref(&self) -> &Moderator {
            self
        }
    }

    impl AsRef<User> for Moderator {
        fn as_ref(&self) -> &User {
            &self.user
        }
    }

    //使用 User 和 Moderators （也是一种User）应该都是可以调用的
    fn create_post<U: AsRef<User>>(u: U) {
        let user = u.as_ref();
        // etc
    }

    fn example(user: User, moderator: Moderator) {
        create_post(&user);
        create_post(&moderator); // ✅
    }

    pub fn t2() {
        let u = User {
            name: "alice".to_string(),
            age: 18,
        };
        let m = Moderator {
            user: User {
                name: "bob".to_string(),
                age: 19,
            },
            privileges: vec![Privilege::BanUsers, Privilege::DeletePosts],
        };

        example(u, m);
    }
}

fn main() {
    m2::t2();
}

// Deref 解引用强制转换是类型间的隐式转换，这就为人们制定错误的想法并对其行为方式的期望留下了空间。
// AsRef能够工作是因为它让类型之间的转换变为显式的，并且没有给开发者错误的想法和期望留有余地
