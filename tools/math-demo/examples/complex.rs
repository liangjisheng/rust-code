use num::complex::Complex;
use std::f64::consts::PI;

fn main() {
    // 创建类型 num::complex::Complex 的复数，复数的实部和虚部必须是同一类型
    let complex_integer = Complex::new(10, 20);
    let complex_float = Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);

    // 对复数执行数学运算与对内置类型执行数学运算是一样的：计算的数字必须是相同的类型（如浮点数或整数）
    let complex_num1 = Complex::new(10.0, 20.0); // 必须为浮点数
    let complex_num2 = Complex::new(3.1, -4.2);
    let sum = complex_num1 + complex_num2;
    println!("Sum: {}", sum);

    // 在与其它数学函数交互时，复数有一系列有趣的特性，尤其是和自然常数 e（欧拉数）类似的正弦相关函数
    // 要将这些函数与复数一起使用，复数类型有几个内置函数，详细请参阅 num::complex::Complex
    let x = Complex::new(0.0, 2.0 * PI);
    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
