#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{} days ", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("Base 10:                {}", 69420);
    println!("Base  2:(binary)        {:b}", 69420);
    println!("Base  8:(octal)         {:o}", 69420);
    println!("Base 16:(hexadecimal)   {:x}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 10);

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
