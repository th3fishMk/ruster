// use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    println!("{THREE_HOURS_IN_SECONDS}");
    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("{quotient}");

    let truncated = -5 / 3; // Results in -1
    println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{0}", tup.0);

    let a = [1, 2, 3, 4, 5];

    for i in a {
        println!("{i}");
    }

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");

    another_function(5);
    print_labeled_measurement(4, 'k');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    _ = five();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {
    5
}
