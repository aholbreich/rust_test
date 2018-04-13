// This is the main function
fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");

     // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{0} as binery is {0:b} ",  2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    let pi = 3.141592;

    println!("Debug pi {:?}", pi);

    //https://doc.rust-lang.org/std/fmt/
    println!("Debug pi {:.*}",2, pi);

    let formatted_number = format!("{:.*}", 2, 1.234567);

    assert_eq!("1.23", formatted_number);


}