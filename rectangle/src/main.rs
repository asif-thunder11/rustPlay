#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {       
        self.width * self.height        
    }

    // &self is shorthand for self: &Self 

    fn can_hold(&self, r2: &Rectangle) -> bool {
        self.width > r2.width && self.height > r2.height        
    }

    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r = Rectangle{
        width: 40, 
        height:  40,
    };

    println!("rectangle: {r:?}", );
    println!("rectangle: {r:#?}", );
    println!("Area: {}", r.area());

    let r2 = Rectangle {
        width: 30,
        height: 36,
    };

    println!("can hold: {}", r.can_hold(&r2));

    let square = Rectangle::square(20);
    println!("Square: {square:#?}");
}
