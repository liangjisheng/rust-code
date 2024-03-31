use std::fmt::Display;

struct Animal<T: Display> {
    name: String,
    age: u8,
    is_male: bool,
    species: String,
    weight: f32,
    animal_type: std::marker::PhantomData<T>,
}

impl<T: Display> Animal<T> {
    fn new(name: String, age: u8, is_male: bool, species: String, weight: f32) -> Animal<T> {
        Animal {
            name,
            age,
            is_male,
            species,
            weight,
            animal_type: std::marker::PhantomData,
        }
    }

    fn print_info(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Is male: {}", self.is_male);
        println!("Species: {}", self.species);
        println!("Weight: {}", self.weight);
    }
}

struct Cat;
struct Dog;

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cat")
    }
}

impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dog")
    }
}

fn main() {
    let cat = Animal::<Cat>::new("Tommy".to_string(), 3, true, Cat.to_string(), 4.5);
    let dog = Animal::<Dog>::new("Buddy".to_string(), 5, true, Dog.to_string(), 12.3);

    cat.print_info();
    dog.print_info();
}
