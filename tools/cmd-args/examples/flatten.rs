// 在处理控制台参数时，我们通常需要处理一些复杂的结构参数，例如具有嵌套字段的结构体或向量。
// 在这种情况下，我们可以使用 structopt 库的#[structopt(flatten)]和
// #[structopt(skip)]属性来解决问题

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Address {
    city: String,
    state: String,
    country: String,
}

#[derive(Debug, StructOpt)]
struct Animal {
    name: String,
    age: u8,
    #[structopt(flatten)]
    address: Address,
}

// cargo run -- Tom 30 Chengdu Chengdu China

fn main() {
    let args = Animal::from_args();
    println!("{:?}", args);
}
