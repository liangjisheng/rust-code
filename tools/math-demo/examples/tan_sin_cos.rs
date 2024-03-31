// 验证 tan(x) 是否等于 sin(x)/cos(x)，其中 x=6

fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    assert_eq!(a, b);
}
