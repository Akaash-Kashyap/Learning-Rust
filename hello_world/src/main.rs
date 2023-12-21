fn main() {
    println!("Hello, world!");

    println!("{}, {}!", "Hello", "World"); // another way to do this

    println!("{1}, {0}!", "Hello", "World"); // custom order

    println!("{greeting}, {name}!", greeting = "Hello", name = "world"); // can use variables within println

    // Variables
    let (greeting, name) = ("Hello", "world"); // creating variables (without explicitly setting the type) and initializing them 

    println!("{greeting}, {name}!");

    // printing an array
    println!("{:?}", [1, 2, 3]);
    println!("{:#?}", [1, 2, 3]);

    // format! macro can store formatted strings
    let x: String = format!("{}, {}!", "Hello", "world");

    println!("{}", x);
    // also has a print! macro for printing without a new line

    print!("{}", x);
    print!("{}", x);

}