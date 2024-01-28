fn main() {
    let mut counter = 3;
    while counter != 0 {
        println!("counter: {}", counter);
        counter -= 1;
    }
    println!("end");

    let mut n = 0;
    while n <= 5 {
        println!("{}!", n);
        n = n + 1;
    }
    println!("我出来了！");
}
