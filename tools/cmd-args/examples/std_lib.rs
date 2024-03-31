use std::env;

// cargo run -- -v -h
// cargo run -- -v -h test

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("program name is {}", args[0]);

    for arg in args.iter().skip(1) {
        println!("Argument: {}", arg);
        match arg.as_str() {
            "-v" => println!("version is xxx"),
            "-h" => println!("Help message"),
            _ => println!("Unknown argument: {}", arg),
        }
    }
}
