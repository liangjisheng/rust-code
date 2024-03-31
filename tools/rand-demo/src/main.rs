// 通过 rand::thread_rng，在随机数生成器 rand::Rng 的帮助下生成随机数。
// 每个线程都有一个初始化的生成器。整数在该类型的范围（最大值 ～ 最小值）内，均匀分布。
// 还有浮点数是从 0 到 1，但不包括 1 的 均匀分布

use rand::distributions::{Alphanumeric, Distribution, Standard, Uniform};
use rand::Rng;

fn number_demo() {
    // 由系统创建的本地线程，是延迟初始化的随机数生成器
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    // 用 Rng::gen_range ，在半开放 [0, 10) 范围（不包括 10 ）生成随机值
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));

    println!("{:?}", rng.gen::<(f64, bool)>());
    let tuple: (u8, i32, char) = rng.gen(); // arbitrary tuple support
    println!("{:?}", tuple);

    let arr1: [f32; 32] = rng.gen(); // array construction
    println!("{:?}", arr1);
    let mut arr2 = [0u8; 128];
    rng.fill(&mut arr2);
    println!("{:?}", arr2);
}

fn uniform_demo() {
    // Uniform 可以用来获得均匀分布的值。下面的代码是相同作用，
    // 但是当在相同范围内，重复生成数字时可能更快
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 把 泛型 具象化为 Point， (定义实现)
impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn r3() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    // 主要是 : Point，类型标签，让编译器知道 (调用上面的实现定义)
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

fn r4() {
    // 从一组字母+数字的字符，创建随机密码
    // rand::distributions::Alphanumeric 是字母数字样本，范围限定为 A-Z，a-z，0-9
    let rand_string: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{:?}", rand_string);
    let str = String::from_utf8(rand_string).unwrap_or("hello".to_string());
    println!("{}", str);
}

fn r5() {
    // 使用用户自定义的字节字符串，随机生成给定长度的 ASCII 字符串，运用 gen_range
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: Vec<char> = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            // 这是安全的，因为 `idx` 会在 `CHARSET` 的范围内s
            // 来自用户的所有输入，最好都定义为不安全的
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        })
        .collect();
    println!("{:?}", password);

    let str = String::from_iter(password.iter());
    println!("{}", str);
}

fn main() {
    // number_demo();
    // uniform_demo();
    // r3();
    // r4();
    r5();
}
