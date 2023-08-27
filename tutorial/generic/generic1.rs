use std::ops::Add;

struct Value<T> {
    a: T,
    b: T,
}

impl<T> Value<T> {
    fn get_a(&self) -> &T {
        &self.a
    }
}

fn add<T: Add<Output = T> + Default + Copy + Clone>(list: &[T]) -> T {
    let mut c: T = T::default();
    for item in list.iter() {
        c = c + *item;
    }
    return c;
}

fn g1() {
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![2.3, 3.3, 4.3, 5.3];
    let result = add(&a);
    let result1 = add(&b);
    println!("The value of result is {}", result);
    println!("The value of result1 is {}", result1);

    let integer = Value { a: 2, b: 3 };
    let float = Value { a: 7.8, b: 12.3 };
    println!("integer values : {},{}", integer.a, integer.b);
    println!("Float values :{},{}", float.a, float.b);
    println!("v.a {}", integer.get_a());
}

fn main() {
    // g1();
}
