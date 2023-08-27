fn main() {
    let s1 = format!("test");
    let s2 = format!("hello {}", "world!");
    let s3 = format!("x = {}, y = {y}", 10, y = 30);
    let (x, y) = (1, 2);
    let s4 = format!("{x} + {y} = 3");

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
}
