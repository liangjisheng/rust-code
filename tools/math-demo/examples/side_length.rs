// 计算直角三角形斜边的长度，其中斜边的角度为 2 弧度，对边长度为 80

fn main() {
    let angle: f64 = 2.0;
    let side_length = 80.0;

    let hypotenuse = side_length / angle.sin();

    println!("Hypotenuse: {}", hypotenuse);
}
