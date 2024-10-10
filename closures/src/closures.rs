/*
    Closures in Rust

    Closures are functions within functions that are nameless

    We can assign closures to variables and then pass a function
    as a parameter to other functions.  They are also known as
    inline functions.

    Closures:  Anonymous functions that capture their environment.

    Rust's closures are anonymous functions that you can save in a variable
    or pass as an argument to other functions.  You can create the closure in
    one place and then call the closure elsewhere to evaluate it in a different 
    context.  Unlike functions, closures can capture values from the scope in 
    which they are defined.  We'll demonstrate how these closure features

*/

fn main() {
    let is_even = |n| {
        n%2==0
    };

    let num = 10;

    println!("{} is even? {}", num, is_even(num));
}
