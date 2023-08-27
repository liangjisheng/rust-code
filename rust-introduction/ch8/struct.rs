// 具名Struct(named Struct)表示有字段名称的Struct
// Struct的字段(Field)也可以称为Struct的属性(Attribute)

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

// 除了named struct外，Rust还支持没有字段名的struct结构体，称为元组结构体(tuple struct)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体(unit-like struct)是没有任何字段的空struct
// struct St;

fn main() {
    let user1 = Person {
        name: String::from("alice"),
        email: String::from("example@xx.com"),
        age: 23,
    };

    // 访问user1实例name字段、age字段和email字段的值
    println!(
        "name: {}, age: {}, email: {}",
        user1.name, user1.age, user1.email
    );

    // 当要构造的Struct实例的字段值来自于变量，且这个变量名和字段名相同，则可以简写该字段
    let name = String::from("alice");
    let email = String::from("example@xx.com");
    let user1 = Person {
        name,  // 简写，等价于name: name
        email, // 简写，等价于email: email
        age: 23,
    };

    // 有时候会基于一个Struct实例构造另一个Struct实例
    // Rust允许通过..xx的方式来简化构造struct实例的写法
    let mut user2 = Person {
        name: String::from("alice"),
        email: String::from("example@yy.com"),
        ..user1
    };

    // 上面的..user1表示让user2借用或拷贝user1的某些字段值，
    // 由于user2中已经手动定义了name和email字段，因此..user1只借用了user1的age字段
    // 即user2.age也是23

    // 注意，如果..base借用于base的字段是可Copy的，那么在借用时会自动Copy，
    // 这样在借用字段之后，base中的字段仍然有效。但如果借用的字段不是Copy的，
    // 那么在借用时会将base中字段的所有权转移走，使得base中的该字段无效
    let mut user3 = Person {
        name: String::from("alice"),
        ..user1
    };

    // 报错，user1.email字段值的所有权已借给 user3
    // println!("{}", user1.email); // error
    // println!("{:?}", user1); // 报错 value partially moved here
    println!("{}", user1.name); // 正确
    println!("{}", user1.age); // 正确

    // 如果确实要借用user1的email属性，可以使用..user1.clone()先拷贝堆内存中的user1，
    // 这样就不会借用原始的user1中的email所有权
    let user4 = Person {
        name: String::from("ggg"),
        ..user2.clone()
    };
    println!("{:?}", user2);
    println!("{:#?}", user2);

    // black和origin值的类型不同，因为它们是不同的结构体的实例
    // 在其他方面，元组结构体实例类似于元组：可以将其解构
    // 也可以使用.后跟索引来访问单独的值，等等
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);
}
