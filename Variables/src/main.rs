fn main() {
    
    let a; // Declaration; no data type
    a = 5; // assignment

    let b: i8; // Declaration; with data type
    b = 5;

    let t = true;   // declaration + assignment; no data type
    let f: bool = false;  // declaration + assignment; with data type

    let (x,y) = (1,2); // setting x = 1 and y = 2

    let mut z = 5;
    z = 6; // since z is mut we can change it

    let z = {x + y}; // z = 3 now

    let z = {
        let x = 1;
        let y = 4;
        //println!("{y}"); statements here use the inner version of x an y
        x + y
    }; // z = 5
    // here we use the outer versions of x and y
    println!("z = {z}");
    println!("x = {x}, y = {y}");




    // constants - not allowed to change, live for the entire duration of the program, no fixed address in memory
    const C: i32 = 5;
    
    // statics - global variable, one instance, at a fixed location in memory
    static S: i32 = 5;

    // variable shadowing
    let num: f64 = -20.48; // float
    let num: i64 = num.floor() as i64; // int

    println!("{num}"); // -21

    let string: &str = "hello"; // &str
    let string: String = string.to_uppercase(); // String

    println!("{string}"); // HELLO

}

// #Notes
/* variables are immutable by default (constants), use mut keyword to make them mutable */ 