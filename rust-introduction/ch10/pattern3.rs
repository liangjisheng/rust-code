// ref和mut修饰模式中的变量
// 当进行解构赋值时，很可能会将变量拥有的所有权转移出去，从而使得原始变量变得不完整或直接失效

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn p1() {
    let p = Person {
        name: String::from("alice"),
        age: 23,
    };
    let Person { name, age } = p;

    println!("{}", name);
    println!("{}", age);
    // println!("{}", p.name); // 错误，name字段所有权已转移
}

fn p2() {
    let p = Person {
        name: String::from("alice"),
        age: 23,
    };

    // 如果想要在解构赋值时不丢失所有权，有以下几种方式
    // 方式一：解构表达式的引用
    let Person { name, age } = &p;
    println!("name: {}, age: {}", name, age);

    // 方式二：解构表达式的克隆，适用于可调用clone()方法的类型
    // 但Person struct没有clone()方法

    // 方式三：在模式的某些字段或元素上使用ref关键字修饰变量
    let Person { ref name, age } = p;
    println!("name: {}, age: {}", name, age);

    let Person { name: ref n, age } = p;
    println!("name: {}, age: {}", n, age);

    // 在模式中使用ref修饰变量名相当于对被解构的字段或元素上使用&进行引用
    let x = 5_i32; // x的类型：i32
    let x = &5_i32; // x的类型：&i32
    let ref x = 5_i32; // x的类型：&i32
    println!("x {}", x);
    let ref x = &5_i32; // x的类型：&&i32

    // 输出的时候解引用, 会解到最外层
    println!("x {}", x); // 5
}

fn p3() {
    // 如果想要对解构赋值的变量具有数据的修改权，需要使用mut关键字修饰模式中的变量
    // 但这样会转移原值的所有权，此时可不要求原变量是可变的
    let p = Person {
        name: String::from("alice"),
        age: 23,
    };
    match p {
        Person { mut name, age } => {
            name.push_str(" bob");
            println!("name: {}, age: {}", name, age)
        }
    }
    // println!("{:?}", p); // 错误

    // 如果不想在可修改数据时丢失所有权，可在mut的基础上加上ref关键字，就像&mut xxx一样
    // 这里要改为 mut p
    let mut p = Person {
        name: String::from("alice"),
        age: 23,
    };
    match p {
        // 这里要改为ref mut name
        Person { ref mut name, age } => {
            name.push_str(" bob");
            println!("name: {}, age: {}", name, age)
        }
    }
    println!("{:?}", p);
}

fn p4() {
    // 注意，使用ref修饰变量只是借用了被解构表达式的一部分值
    // 而不是借用整个值。如果要匹配的是一个引用，则使用&

    let a = &(1, 2, 3); // a是一个引用
    let (t1, t2, t3) = a; // t1,t2,t3都是引用类型&i32
    assert_eq!(t1, &1);
    assert_eq!(t2, &2);
    assert_eq!(t3, &3);
    let &(x, y, z) = a; // x,y,z都是i32类型
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);
    let &(ref xx, yy, zz) = a; // xx是&i32类型，yy,zz是i32类型
    assert_eq!(xx, &1);
    assert_eq!(yy, 2);
    assert_eq!(zz, 3);

    // 最后，也可以将match value{}的value进行修饰，例如match &mut value {}，
    // 这样就不需要在模式中去加ref和mut了。这对于有多个分支需要解构赋值，
    // 且每个模式中都需要ref/mut修饰变量的match非常有用
    let mut s = "hello".to_string();
    match &mut s {
        // 对可变引用进行匹配
        // 匹配成功时，变量也是对原数据的可变引用
        x => x.push_str(" world"),
    }
    println!("{}", s);
}

fn main() {
    // p1();
    // p2();
    // p3();
    p4();
}
