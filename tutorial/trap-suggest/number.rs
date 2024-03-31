// 算术溢出导致的 panic
// 在 Rust 中，溢出后的数值被截断是很正常的

fn n1() {
    let x: u16 = 65535;
    let v = x as u8;
    println!("{}", v); // 255

    // 实际上，如果是因为算术操作符导致的溢出，就会让整个程序 panic
    // let x: u8 = 10;
    // 编译失败 attempt to compute `10_u8 + u8::MAX`, which would overflow
    // let v = x + u8::MAX;
    // println!("{}", v)

    // 那么当我们确实有这种需求时，该如何做呢？可以使用 Rust 提供的checked_xxx系列方法
    let x: u8 = 10;
    let v = x.checked_add(u8::MAX).unwrap_or(0);
    println!("{}", v)
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

// 上述代码中，speed * cph就会直接 panic:

fn main() {
    // n1();

    let v = production_rate_per_hour(5);
    println!("{}", v);
}
