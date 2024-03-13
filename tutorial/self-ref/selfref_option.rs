// 使用 Option 解决结构体自引用

#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}

fn s1() {
    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.nickname = Some(&tricky.name[..4]);

    println!("{:?}", tricky);
}

fn main() {
    // s1();
    s2();
}

// 在某种程度上来说，Option 这个方法可以工作，但是这个方法的限制较多，例如从一个函数创建
// 并返回它是不可能的：

// fn creator<'a>() -> WhatAboutThis<'a> {
//     let mut tricky = WhatAboutThis {
//         name: "Annabelle".to_string(),
//         nickname: None,
//     };
//     tricky.nickname = Some(&tricky.name[..4]);
//
//     tricky
// }

impl<'a> WhatAboutThis<'a> {
    fn tie_the_knot(&'a mut self) {
        self.nickname = Some(&self.name[..4]);
    }
}

fn s2() {
    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.tie_the_knot();

    // cannot borrow `tricky` as immutable because it is also borrowed as mutable
    // println!("{:?}", tricky);
}
