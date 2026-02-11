// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// Rust code uses snake case as the conventional style
// for function and variable names

// Note that we defined another_function
// after the main function in the source code
// Rust doesn’t care where you define your functions,
// only that they’re defined somewhere in a scope
// that can be seen by the caller.


// fn main() {
//     another_function(5); // 5 is argument
// }

// fn another_function(x: i32) { // here is th parameter
//     println!("The value of x is: {x}");
// }

// In function signatures, you must declare the type of each parameter.

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }



// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

// fn main() {
//     let y = 6;
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }


// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
