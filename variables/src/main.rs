use core::panic;

fn main() {

    let x = 5; // immutable

    let mut y = 6;

    println!("X: {x}, y: {y}");
    y = 3;
    println!("X: {x}, y: {y}");

    let x = "shadow";   // x is shadowed; 

    {
        let y:&str = "six";
        println!("inside scope, shadowed y: {y}");
    }

    const ONE_HOUR: u32 = 60; 

    println!("x: {x}, const ONE_HOUR: {ONE_HOUR}");

    // chapter 3 Practice
    // prog_celcius_to_farenheit();
    // prog_fibonacci();
    prog_4_days_of_christmas();
    
}

fn prog_celcius_to_farenheit() {
    let mut celcius = String::new();
    println!("Enter Celcius: ");

    std::io::stdin()
    .read_line(&mut celcius)
    .expect("Failed to read line");

    let celcius: f32 = match celcius.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Error")
    };

    let farenheit: f32 = celcius_to_farenheit(celcius);
    println!("Celcius to Farenheit: {farenheit}")
}

fn celcius_to_farenheit(celcius: f32) -> f32 {
    celcius * 9.0 / 5.0 + 32.0
}

fn prog_fibonacci() {
    let mut input = String::new();
    println!("Enter n: ");

    std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Error")
    };

    println!("the {input}th fibonacci number is {}", fibonacci(input));
}

fn fibonacci(n: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    let mut i = 3;
    let mut result = first + second;

    while i < n {
        first = second;
        second = result;
        result = first + second;
        i+=1;
    }

    result
}

fn prog_4_days_of_christmas() {
    let days: [&str; 4] = ["first", "second", "third", "fourth"];
    let counting_words: [&str; 4] = [
        "a partridge in a pear tree", 
        "two turtle-doves", 
        "three french hens", 
        "four calling birds"
        ];


    println!("On the {} day of christmas\nMy true love sent to me\n{}\n\n", days[0], counting_words[0]);

    let mut i = 1;
    while i < 4 {
        println!("On the {} day of christmas\nMy true love sent to me", days[i]);
        for n in (0..=i).rev() {
            if n == 0 {
                print!("And ");
            }
            print!("{}\n", counting_words[n])
        }

        println!("\n");
        i+=1;
    }

}
