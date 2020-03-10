use std::io;

//  Calculate Area of a Circle  

// fn main() {
//     // Setting value of pie
//     const PI: i32 = 22/7;

//     // Taking input from user
//     let mut radius = String::new();
//     io::stdin().read_line(&mut radius)
//         .expect("Something went wrong");
//     let radius: i32 = match radius.trim().parse() {
//         Ok(num) => num,
//         Err(_) => panic!("Only enter numbers!!!"),
//     };

//     // Calculating Area
//     let area = PI * (radius*radius);
//     println!("Area of Circle with radius {} is {}", radius, area);
// }

// Check Number either positive, negative or zero 
// fn main() {
//     // Taking input from user
//     let mut number = String::new();
//     io::stdin().read_line(&mut number)
//         .expect("Enter something");
//     let number: i32 = match number.trim().parse() {
//         Ok(num) => num,
//         Err(_) => panic!("Only enter numbers!!!"),
//     };

//     // Logic to check the state of number
//     if number < 0 { println!("Negative number entered"); }
//     else if number > 0 { println!("Positive number entered"); }
//     else { println!("Zero entered"); }
// }

// Divisibility Check of two numbers 
fn main() {
    // Take first input
    let mut inp1 = String::new();
    io::stdin().read_line(&mut inp1)
        .expect("Error");
    let inp1: i32 = inp1.trim().parse().unwrap();

    // Take second input
    let mut inp2 = String::new();
    io::stdin().read_line(&mut inp2)
        .expect("Error");
    let inp2: i32 = inp2.trim().parse().unwrap();

    // Logic
    if (inp1 % inp2) == 0 { println!("Number {} is completely divisible by {}", inp1, inp2); }
    else { println!("Number {} is not completely divisible by {}", inp1, inp2); }
}