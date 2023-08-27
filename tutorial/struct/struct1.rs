// Debug 表示调试信息
// Default 提供默认值
#[derive(Debug, Default)]
struct Employee {
    name: String,
    company: String,
    age: u32,
}

fn display(emp: Employee) {
    println!(
        "Name is :{} company is {} age is {}",
        emp.name, emp.company, emp.age
    );
}

fn struct1() {
    let emp = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Alice"),
        age: 50,
    };
    println!(
        "Name is :{} company is {} age is {}",
        emp.name, emp.company, emp.age
    );

    let mut emp1 = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Alice"),
        age: 50,
    };
    emp1.age = 40;
    println!(
        "Name is :{} company is {} age is {}",
        emp1.name, emp1.company, emp1.age
    );

    display(emp);
    display(emp1);
}

// Rust 中的结构体不仅仅可以作为函数的参数，还可以作为 函数的返回值
fn who_is_elder(emp1: Employee, emp2: Employee) -> Employee {
    return if emp1.age > emp2.age { emp1 } else { emp2 };
}

fn struct2() {
    let emp = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Alice"),
        age: 30,
    };

    let mut emp1 = Employee {
        company: String::from("TutorialsPoint"),
        name: String::from("Alice"),
        age: 20,
    };

    let elder = who_is_elder(emp, emp1);
    println!("elder is:");
    display(elder);
}

fn main() {
    // struct1();
    struct2();
}
