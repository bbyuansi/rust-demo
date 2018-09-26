#![allow(dead_code)]

#[derive(Debug)]
enum Person {
    Engineer,
    Scientist,
    Height(f32),
    Weight(f32),

    Info { name: String, age: i32 },
}

enum Work {
    Programmer,
    Architect,
}

enum Condition {
    Poor,
    Rich,
}

enum Number {
    Zero,
    Three,
    Two
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("Engineer"),
        Person::Scientist => println!("Scientist"),
        Person::Height(i) => println!("height is {}.", i),
        Person::Weight(i) => println!("weight is {}.", i),

        Person::Info { name, age } => println!("name: {}, age: {}.", name, age)
    }
}

fn enums() {
    println!("{:#?}", Person::Height(32f32));
    println!("{:#?}", Person::Scientist);
    let height = Person::Height(20.5);
    let weight = Person::Weight(40f32);
    let info = Person::Info { name: "queen".to_owned(), age: 12 };
    let rebecca = Person::Engineer;
    let rohan = Person::Scientist;

    inspect(height);
    inspect(weight);
    inspect(info);
    inspect(rebecca);
    inspect(rohan);

    use Work::{Architect, Programmer};
    use Condition::*;
    let work = Architect;
    let cond = Rich;

    match work {
        Architect => println!("noble"),
        Programmer => println!("struggle"),
    }

    match cond {
        Poor => println!("no money"),
        Rich => println!("a lot of money"),
    }

    println!("{}", Number::Zero as i32);
    println!("{}", Number::Three as i32);
    println!("blue #{:06x}", Color::Blue as i32);
    println!("red #{:06x}", Color::Red as i32);
}