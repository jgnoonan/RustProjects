/*
What are enums in Rust?

An enum type is a special data type that enables a variable to be a set of predefined constants.
The variable must be equal to one of the values that have been poredefined for it.  Common
examples include compass directions (values of NORTH, SOUTH, EAST, and WEST), and the days of the
week.

*/

// Example Enum
// the derive attribute makes the enum printable

/*
Matching Enums in Rust Exercise

1.  Write an Enum for three different types of shoes:  loafers, nike, and vans
2.  Using the match keyword write a function that takes the enum as a value and can
    match each shoe with the following printed statements:  nike: "Great for running", 
    loafers: "Great for loafing around", vans: "Great for Skateboarding"
3.  Call the function and print each string match by passing in each enum value and printing
    the matched result.
*/
#[derive(Debug)]
enum Shoes {
    Loafer, Nike, Vans
}

fn shoe_match(x:Shoes) {
    match x {
        Shoes::Loafer => println!("Great for loafing around"), 
        Shoes::Nike => println!("Great for running"),
        Shoes::Vans => println!("Great for skateboarding"),
    }
}

fn main() {
    let mut my_shoes:Shoes = Shoes::Loafer;
    shoe_match(my_shoes);
    my_shoes = Shoes::Nike;
    shoe_match(my_shoes);
    my_shoes = Shoes::Vans;
    shoe_match(my_shoes);
}
