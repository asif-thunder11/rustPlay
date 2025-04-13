use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop { 
        println!("Enter number: ");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number)  {
            Ordering::Greater => { println!("too big"); }
            Ordering::Less => { print!("too little"); }
            Ordering::Equal => {
                print!("You win");
                break; 
            }
        }
    }

}
