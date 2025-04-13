struct Person {
    name: String,
    age: u32,
}


fn main() {

    let thunder = Person {
        name: String::from("thunder"),
        age: 28,
    };

    println!("Hello, {}, age: {}!", thunder.name, thunder.age);
}
