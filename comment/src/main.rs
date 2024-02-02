use comment::add;
use comment::computer::sub_one;

use comment::art::kinds::PrimaryColor;
use comment::art::utils::mix;

fn main() {
    let res = add(1, 2);
    println!("res {}", res);

    let a = 5;
    let res1 = sub_one(a);
    println!("res1 {}", res1);

    let blue = PrimaryColor::Blue;
    let yellow = PrimaryColor::Yellow;
    println!("{:?}", mix(blue, yellow));
}
