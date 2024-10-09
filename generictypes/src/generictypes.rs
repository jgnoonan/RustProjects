/*

Generic Data Types in Rust

We use generics to create definitions for items like function signatures or 
structs, which we can then use with many different concrete data types.

Let's first look at how to define functions, structs, enums, and methods using
generics.  Then we'll discuss how generics affect code performance.

The <T> syntax known as the type parameter, is used to declare a generic construct.
T represents any data type.

**** Traits ****

Trait defines functionality a particular type has and can share with other types.

We can use traits to define shared behavior in an abstract way (overriding functions).
We can use trait bounds to specify that a generic type can be any type that has certain
behavior (concretely).

Traits are often used to implement a standard set of actions (methods) across multiple 
structures.  Traits act like interfaces in OOP.

*/

struct GenericStruct<T> {
    value:T,
}

struct Game {
    weapon: &'static str,
    power_level:u32
}

trait Stats {
    fn character_stats(&self);
    
}

impl Stats for Game {
    fn character_stats(&self) {
        println!("Printing stats of power level: {}, weapon: {}", self.power_level, self.weapon);
    }
}

fn main() {
    let t1:GenericStruct<i32> = GenericStruct{ value:100 };
    println!("The value is: {}", t1.value);

    let t2:GenericStruct<String> = GenericStruct{ value:"String Data Type".to_string() };
    println!("The value is: {}", t2.value);

    let g1=Game {
        power_level:100,
        weapon: "Sword of Fire"
    };
    g1.character_stats();
}
