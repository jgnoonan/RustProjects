/*
Strings: String Literal

String literals are a set of characters, which are hardcoded into a variable.

let user="The Incredible Hulk".  String literals are found in module std:str.

String literals can also be called string slices.

The string datatype in Rust can be applied as such:

String Literal (&str)
String Object (String)

String literals are static by default.  This ensures that the string is valid
for the entire duration of the program.  You can explicitly declare a string as 
static.

*/


fn main() {
    let greeting: &'static str= "Hello, world!";
    println!("{}", greeting);

    // Use a String Object
    let great_movie = String::from("Empire of the Sun");
    println!("{}", great_movie);

    // push Example
    let mut greeting = String::from("Julia says, ");
    greeting.push_str("hello!");
    println!("{}", greeting);

    // Convert a string literal into a String object
    let random_string = "Please make me into an object!".to_string();
    println!("{}",random_string);

    /* 
    Exercise: Manipulating String objects in Rust

    1.  In the main function, create a string literal named password and assign it the value of "pokemon,"
    2.  Using the String object push method, modify the password so that it includes " gotta catch them all!"
    3.  Print the results and share your solution on the discord in the Rust channel!
     */

    let mut password = "Pokemon,".to_string();
    password.push_str(" gotta catch them all!");
    println!("{}", password);

}
