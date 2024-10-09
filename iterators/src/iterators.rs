/*

Iterators in Rust

Iterators travers and iterate over a different collection of values from types
such as arrays, vectors, maps, etc.  The Iterator trait is invoked from iterators
defined in the Rust standard library.  The iter() method returns an iterator object
of the collection of items.  The next() method traverses through items and returns
none once it reaches the end of items in the collection.

*/

/*
Exercise - Iterate through a vector in Rust

1.  Write a vector called pets which contains the 3 string items cat, dog, goldfish.
Uswthe for in iter_mut() signature to match dog so that the program prints "cute doggy!"
The default underscore _ should print hello! and each pet name for each iteration.
*/ 

fn main() {
    let x = [1,2,3,4,5];
    let iter = x.iter();
    for items in iter {
        print!("{}\t", items);
    }
    println!("");

    // into_iter method moves values in the collection into an iter object via ownership
    let values = vec!["a","b","c","d","e"];
    for value in values.into_iter() {
        match value {
            "c" => println!("c is a good time"),
            "e" => println!("e has its problems!"),
            _ => println!("iteration: {}", value),
        }
    }
    let mut pets = vec!["cat", "dog", "goldfish"];
    for pet in pets.iter_mut() {
        match *pet {
            "dog" => println!("cute doggy!"),
            _ => println!("Hello, {}!", pet),
        }
    }
    println!("{:?}", pets);
}
