fn main() {
    use std::io::{stdin,stdout, Write};

    print!("Enter a number! ");
    let _=stdout().flush();
    // create a string called number
    let mut input_number = String::new();
    

    stdin().read_line(&mut input_number).expect("Failed to read line");
    
    let mut num:i32 = input_number.trim().parse().expect("Input not integer");

    // println! ("Your number is: {}",num);
    println!("generating factorial representation ~");
    
    let mut sum:i32 = 1;

    while num > 0{
        sum = sum * num;
        num -= 1;
    }
    println! ("Your factorial is: {}",sum);
}
