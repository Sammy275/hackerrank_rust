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
// fn main() {
//     // Take first input
//     let mut inp1 = String::new();
//     io::stdin().read_line(&mut inp1)
//         .expect("Error");
//     let inp1: i32 = inp1.trim().parse().unwrap();

//     // Take second input
//     let mut inp2 = String::new();
//     io::stdin().read_line(&mut inp2)
//         .expect("Error");
//     let inp2: i32 = inp2.trim().parse().unwrap();

//     // Logic
//     if (inp1 % inp2) == 0 { println!("Number {} is completely divisible by {}", inp1, inp2); }
//     else { println!("Number {} is not completely divisible by {}", inp1, inp2); }
// }

// Calculate Volume of a sphere 
// fn main() {
//     // Defining Pi
//     const PI: f32 = 22.0/7.0;

//     // Taking input from user
//     let mut radius = String::new();
//     io::stdin().read_line(&mut radius)
//         .expect("Something went wrong");
//     let radius: f32 = radius.trim().parse().unwrap();

//     // Calculating volume
//     let volume: f32 = (4.0/3.0) * PI * (radius * radius * radius);
//     println!("Volume of the Sphere with Radius {} is {}", radius, volume);
// }

// Copy string n times 
// fn main() {
//     // User input of String
//     let mut string = String::new();
//     io::stdin().read_line(&mut string).unwrap();
//     let string: String = string.trim().parse().unwrap();

//     // Inputting duplication Number
//     let mut copy = String::new();
//     io::stdin().read_line(&mut copy).unwrap();
//     let copy: i32 = copy.trim().parse().unwrap();
//     if copy < 0 { panic!("Enter a number greater than 0!!!"); }

//     //  printing the string
//     for _i in 0..copy {
//         print!("{}", string);
//     }
// }

// Check if number is Even or Odd 
// fn main() {
//     // Input Number
//     let mut number = String::new();
//     io::stdin().read_line(&mut number)
//         .expect("Enter something");
//     let number: f32 = number.trim().parse().unwrap();

//     // Checking if number is even or odd
//     if (number % 2.0) == 0.0 { println!("{} is Even", number); }
//     else { println!("{} is Odd", number); }
// }

// Vowel Tester  
// fn main() {
//     // Input character
//     let mut character = String::new();
//     io::stdin().read_line(&mut character).unwrap();
//     let character: char = character.trim().to_lowercase().parse().unwrap();

//     match character {
//         'a'|'e'|'i'|'o'|'u' => println!("{} is a vowel", character),
//         _ => println!("{} is not a vowel", character),
//     }
// }

//. Sum of n Positive Integers  
fn main() {
    // Input Number
    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Enter something");
    let number: i32 = number.trim().parse().unwrap();

    // Total
    let mut total: i32 = 0;
    for i in 1..=number {
        total = total + i;
    }

    println!("{}", total);
}