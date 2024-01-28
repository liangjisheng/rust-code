// 浮点数在某些特性上是反直觉的 例如大家都会觉得浮点数可以进行比较，对吧？是的，它们确实可以使用 >，>=
// 等进行比较，但是在某些场景下，这种直觉上的比较特性反而会害了你。因为 f32 ， f64 上的比较运算实现的是
// std::cmp::PartialEq 特征(类似其他语言的接口)，但是并没有实现 std::cmp::Eq 特征，但是后者在其它
// 数值类型上都有定义

// 需要遵守以下准则：
// 避免在浮点数上测试相等性
// 当结果在数学上可能存在未定义时，需要格外的小心

fn f1() {
    // 断言0.1 + 0.2与0.3相等, 代码会 panic
    // assert!(0.1 + 0.2 == 0.3);

    // 不会 panic
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    // 正常通过
    assert!(abc.0 + abc.1 == abc.2);
    // 因为 f64 精度高很多，因此在小数点非常后面发生了一点微小的变化，0.1 + 0.2 以 4 结尾，
    // 但是 0.3 以3结尾，这个细微区别导致 f64 下的测试失败了，并且抛出了异常。
    assert!(xyz.0 + xyz.1 == xyz.2);
}

// 对于数学上未定义的结果，例如对负数取平方根 -42.1.sqrt() ，会产生一个特殊的结果：
// Rust 的浮点数类型使用 NaN (not a number)来处理这些情况。
// 所有跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较，下面的代码会崩溃：
fn f2() {
    let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);

    // 出于防御性编程的考虑，可以使用 is_nan() 等方法，可以用来判断一个数值是否是 NaN
    if x.is_nan() {
        println!("未定义的数学行为")
    }
}

fn f3() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}

fn f4() {
    let f = 3.3_f32;
    let g = -3.3_f32;
    let h = -3.7_f32;
    let i = 3.5_f32;
    let j = 4.5_f32;

    assert_eq!(f.round(), 3.0);
    assert_eq!(g.round(), -3.0);
    assert_eq!(h.round(), -4.0);
    assert_eq!(i.round(), 4.0);
    assert_eq!(j.round(), 5.0);
}

fn main() {
    // f1();
    // f2();
    // f3();
    f4();
}
