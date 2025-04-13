fn main() {
    let a = "hello";

    {
        let a1 = "hi";
        println!("a1: {a1}");
    }

    // a1's scope has ended

    let s1 = String::from("Hello");

    let s2 = s1; 

    // S1 is now invalid because s1's ownership is moved to s2.
    println!("s2: {s2}");

    function_takes_ownership(s2);

    // s2 is also invalid now, as ownership is moved to function
    // if you want it back, return s2 from the function and assign it to a variable
    

    // primitive types are copied to stack so their ownership is not moved.
    // while for String, the reference to the data on heap is copied to stack.
    // so the metadata is copied to stack but the pointer points to the same actual data.
    // when an object goes out of scope, it is dropped

    // references: you give reference of on object to others, 
    // they can only read the data and not write. Ownership is not moved.
    // this is basically immutable reference

    // there are 2 types of reference: immutable and mutable

    let a = String::from("hulk");

    function_accepting_reference(&a);
    function_takes_ownership(a);

    // error, ownership moved and data dropped as function/scope has ended
    // println!("a: {}", a);

}

fn function_takes_ownership(s: String) {
    println!("I own: {}, now", s);
}

fn function_accepting_reference(s: &String) {
    println!("I'm only using {}, can't own", s);
}
