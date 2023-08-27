use clap::Parser;
use clap::{Arg, Command, Subcommand};

// https://github.com/clap-rs/clap/tree/master/examples
// https://blog.csdn.net/yhb_csdn/article/details/131162434

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Parser)]
#[command(name="MyApp", author="ljs", version="1.0", about="Does awesome things", long_about=None)]
// #[command(next_line_help = true)] // 注释单独输出一行
struct Cli {
    /// 123456
    #[arg(long)]
    two: String,

    /// 123456
    #[arg(long)]
    one: String,

    // 如果不赋值, 就使用空值None
    /// name 是可选参数
    #[arg(short, long)]
    name: Option<String>,

    // 将元素的类型设置成布尔值, 但是布尔值的特性是, 只能被设置一次, 第二次设置时会报错
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
}

fn main() {
    // let args = Args::parse();
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }

    let cli = Cli::parse();
    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);

    // cargo run -- --one 1 --two 2 --name ljs
    println!("name: {:?}", cli.name.as_deref());

    // cargo run -- --one 1 --two 2 --verbose
    println!("verbose: {:?}", cli.verbose);

    // cargo run -- --one 1 --two 2 add ljs
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {name:?}")
        }
    }
}
