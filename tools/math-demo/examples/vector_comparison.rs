// 下面的实例按元素方式比较两个浮点型 vector。浮点数的存储通常不精确，因此很难进行精确的比较
// 但是，approx crate 中的 assert_abs_diff_eq! 宏允许方便地比较浮点型元素

use approx::abs_diff_eq;
use approx::assert_abs_diff_eq;
use ndarray::Array;

fn main() {
    abs_diff_eq!(1.0, 1.0);

    let a = Array::from(vec![1., 2., 3., 4., 5.]);
    let b = Array::from(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    println!("z = {}", z);
    let w = &c + &d;

    // assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

    println!("c = {}", c);
    c[0] = 10.;
    d[1] = 10.;
    println!("c = {}", c);
    println!("w = {}", w);

    // assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));
}
