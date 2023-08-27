// 根据Rust本身的设计哲学, 建议在设计某个变量时, 如果预计该变量某时刻可能会是空值(null/None)的话,
// 那么尽量用Option/Result来包裹它, 反过来说, 只有你可以肯定该变量不可能为空值时, 才无须这么搞

// Option的枚举情况有两种，分别是代表有的Some()和代表无的None。 如果是有返回值，则可以通过
// if let，match，unwrap，？等多种方法对应情况取出Some包裹的值，如果没有则是None。

// Result的枚举情况也是有两种，表示正确的Ok()和表示错误的Err()。同样也是match，unwrap等等对应方法去提取。分别提取对应情况的内容

// Option的定义如下
// pub enum Option<T> {
//     None,
//     Some(T),
// }

// 每当我们引入一个可能为空的值时，我们必须先把它放到Option里。当我们使用这个值时，我们必须先处理值为空的情况。
// 也就是说，只要一个值不是Option类型的，我们就可以认定它的值不为空。

// Rust在写代码和编译时就发现所有的错误，而不是运行时才发现。理解了这一点，你就能理解 Rust 的各种设定

// 对于Option来说，你必须：
// 第一步： 你必须先判断是Some value，还是None，如果是None，那么简单，直接处理完事。
// 第二步，如果判断出来的是Some value，那么你还需要把具体的value取出来再用。
// 这样的打包方式，核心还是利用编译器帮忙来消除忘记处理没有返回值或者无效返回值的问题。
// 本质上还是为了实践Rust强调的“安全性”

fn o1() {
    let a: i32 = 1;
    let b: Option<i32> = Some(5);
    // unwrap() 做的事就是有值返回值，如果值是 None 则直接 panic!，程序就挂了
    let c = a + b.unwrap();
    println!("{}", c);

    let some_value = Some(10);
    match some_value {
        Some(3) => println!("three"),
        _ => println!("other"),
    }

    // 但是，当我们只关心等于3时的情况，用match就感觉代码太多了，那么我们就可以使用if let
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("other");
    }

    // let v = vec![1, 2];
    // let res: Option<Vec<_>> = v.iter().map(|x| Some(x * 2)).collect();
    // assert_eq!(res, Some([2, 4]))

    // some单个值可以map
    let num = Some(10);
    let var = num.map(|n| 2 * n);
    println!("var:{:?}", var);

    // Some,None
    let x = Some("string");
    let v: Vec<&str> = x.into_iter().collect();
    assert_eq!(v, ["string"]);

    let x = None;
    let v: Vec<&str> = x.into_iter().collect();
    assert!(v.is_empty());

    // clone: Option<&T> => Option
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));

    // take 之后所有权转移
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, None);
}

#[derive(Debug)]
enum CarType {
    Hatch,
    Sedan,
    SUV,
}

fn print_size(car: CarType) {
    match car {
        CarType::Hatch => {
            println!("Small sized car");
        }
        CarType::Sedan => {
            println!("medium sized car");
        }
        CarType::SUV => {
            println!("Large sized Sports Utility car");
        }
    }
}

fn o2() {
    print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);
}

fn main() {
    // o1();
    o2();
}
