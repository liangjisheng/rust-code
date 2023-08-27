use std::io;

enum Type {
    Fahrenheit,
    Celsius,
    None,
}

struct Temp {
    number: f64,
    temp_type: Type,
}

fn title_print() {
    println!("Please select one to enter a number in Fahrenheit and Celsius.");
}

fn get_temp() -> Temp {
    let mut input = String::new();
    let mut temp_input: Temp = Temp {
        number: 0.0,
        temp_type: Type::None,
    };

    println!("Please enter a key(Key F or f is Fahrenheit, Key C or c is Celsius)");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    match input {
        mut input_01 if (input_01.trim() == "F") || (input_01.trim() == "f") => {
            // ch if (&ch[0..1] == "F") || (&ch[0..1] == "f") => {
            // 与上面条件等价
            loop {
                println!("Enter current temperature in Fahrenheit(°F):");
                input_01.clear();

                io::stdin()
                    .read_line(&mut input_01)
                    .expect("Failed to read line.");

                temp_input.temp_type = Type::Fahrenheit;
                temp_input.number = match input_01.trim().parse::<f64>() {
                    Ok(temp) => temp,
                    Err(_) => {
                        println!("Invalid number!");
                        continue;
                    }
                };
                break;
            }
        }
        mut input_02 if (input_02.trim() == "C") || (input_02.trim() == "c") => {
            // ch if (&ch[0..1] == "C") || (&ch[0..1] == "c") => {
            // 与上面代码条件等
            loop {
                println!("Enter current temperature in Celsius(°C):");

                input_02.clear();

                io::stdin()
                    .read_line(&mut input_02)
                    .expect("Failed to read line.");

                temp_input.temp_type = Type::Celsius;
                temp_input.number = match input_02.trim().parse::<f64>() {
                    Ok(temp) => temp,
                    Err(_) => {
                        println!("Invalid number!");
                        continue;
                    }
                };
                break;
            }
        }
        _ => println!("Invalid input."),
    }

    temp_input
}

fn print_temp(temp: Temp) {
    println!(
        "Current temperature: {} {}",
        temp.number,
        match temp.temp_type {
            Type::Fahrenheit => "°F",
            Type::Celsius => "°C",
            _ => "Error",
        }
    );
}

fn convert_temp(mut temp: Temp) {
    match temp.temp_type {
        Type::Fahrenheit => {
            temp.number = (temp.number - 32.0) / 1.8;
            temp.temp_type = Type::Celsius;
        }
        Type::Celsius => {
            temp.number = 32.0 + temp.number * 1.8;
            temp.temp_type = Type::Fahrenheit;
        }
        _ => println!("Error."),
    }

    print_temp(temp);
}

fn main() {
    title_print();

    convert_temp(get_temp());
}
