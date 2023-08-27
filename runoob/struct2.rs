#[derive(Debug)]
// 导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体

struct Dog {
    name: String,
    age: i8,
}

fn main() {
    let mydog = Dog {
        name: String::from("wangcai"),
        age: 3,
    };
    // 用mydog.name给str赋值时，所有权就move到的str变量
    // 应该改为
    // let str = mydog.name.clone();
    let str = mydog.name;
    println!("str={}", str);
    // 编译错误 value borrowed here after move
    println!("mydog: name={},age={}", mydog.name, mydog.age);
}
