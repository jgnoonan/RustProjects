/*
What are Modules in Rust

Rust provides a powerful module system that can be used to hierarchically split code
into logical units (modules), and manage visibility (public/private) between them.  A 
module is a collection of items: functions, structs, traits, impl blocks, and even other
modules.

Multiple modules are compiled into a unit call a crate.  The Cargo tool is used to manage
crates in Rust.

**************************
crate
Compliling unit in Rust compiled to binary or library

2
cargo
The official Rust package management tool for crates.

3
module
A collection of items: functions, structs, traits, impl blocks, and even other modules

4
crates.io
The official Rust package registry.

*/

/*
Exercise - Nested modules in Rust

1.  Create a module named tracks that contains a module named rock that contains
    another module called indie which contains a function called select() which
    takes a String object as a parameter and prints the string.
2.  Import select and in the main function call select three times print the following
    song titles: "Serenade", "French Bagette", and "Pineapple Blues".
*/

pub mod tracks {
    pub mod rock {
        pub mod indie {
            pub fn select(x:String) {
                println!("{}", x);
            }
        }
    }
}

use tracks::rock::indie::select;

fn main() {
    select("Serenade".to_string());
    select("French Bagette".to_string());
    select("Pineapple Blues".to_string());
}
