fn main() {
    // TODO: Change the line below to fix the compiler error.
    let x: i32; // This is the type declaration (32-bit signed integer)
    x = 55; // We then need to let the compiler now what value we are assigning it. 

    // Another way, according to the compiler is: `let x: i32 = 55;`

    println!("Number {x}");

    // This will lead to a `mut` compiler error!
    x = 10; 
    println!(" changing the value of {x} will lead to an error.");
}
