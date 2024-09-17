/*
How to loop in Rust

If you want to repeat a block of code multiple times, which there can be many instances in your program
such as basic calculations for example, you can use loops.

In general, programming instructions happen sequentially:
This means that the first statement in a function would be executed first followed by the next and so on
and so forth.

Languages in programming generally allow us to manipulate the execution paths and customize them more for
preference.  This has gotta be the worst description I've ever seen!

Rust has three types of for loops for executing blocks of code
1. while
2. loop
3. for
*/

fn main() {
    // For loop example - we call this a definite loop because we run it until...
    for a in 1..20 {
        if a == 2 {
            continue;
        }
        println!("Current value of a is : {}", a);
    }

    // an indefinte loop is used when the number of loop iterations
    // is indeterminite or unknown

    let mut b = 0;
    while b < 5 {
        b += 1;
        println!("Value of b = {}", b);
    }

    // the loop also acts while something is true
    let mut c = 0;
    loop {
        c -= 1;
        if c == -10 {
            break;
        } else {
            println!("The value of c is {}", c);
        }
    }

/*
Exercise:  Hope to loop in Rust

1.  Create a unsigned mutable variable call count of 32 bits
2.  Write an infinite loop that increments count +1 and stores the value in count and has the following conditions:
3.  If count is equal to 3 then print the string literal "welcome to miami" to the console
4.  If the count equals 5, then print the string literal "Time to call it a day!" and then exit the loop
*/

    for mut count in 0..5 {
        count += 1;
        if count == 3 {
            println!("Welcome to Miami!")
        } else if count == 5 {
            println!("Time to call it a day");
            break;
        }
    }

}
