use futures::executor::block_on;
use rand::{thread_rng, Rng};

mod school;

fn main() {
    // test_future();
    // test_rand();

    // school::student::say();
    // school::student::util::hello_util();
}

async fn print_async() {
    println!("Hello from print_async");
}

fn test_future() {
    let future = print_async();
    block_on(future);
}

fn test_rand() {
    let x: i32 = rand::random();
    println!("random x = {}", x);

    // rand::thread_rng().gen_range(1..101);
    let num: u32 = rand::thread_rng().gen();
    println!("num {}", num);

    let mut rng = thread_rng();
    let x: u32 = rng.gen();
    println!("{}", x);
    println!("{:?}", rng.gen::<(f64, bool)>());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
