/*
Smart Pointers in Rust

A pointer is a general concept for a variable that contains an address in memory.
This address refers to, or 'points at', some other data.  The most common kind of
pointer in Rust is a reference.  References are indicated by the & symbol and borrow
the value they point to.  They don't have any special capabilities other than 
referring to the data, and have no overhead.

Smart pointers, on the other hand, are data structures that act like a pointer, but also
have additional metadata and capabilities.

The concept of smart pointers isn't unique to Rust: smart pointers originated in C++ and 
exist in other languages as well.  Rust has a variety of smart pointers defined in the standard
library. that provide functionality beyond that provided by references.

Box
Box is a smart pointer that allows you to store memory on the heap explicitly rather than the 
stack and the stack contains the pointer to the heap data.

Two Important Traits implemented by Smart Pointers

1.  Deref (std::ops::Deref)
Immutable dereferencing operations

2.  Drop (std::ops::Drop)
When a value goes out of scope, you can run some code also know as a destructor.

*/

/*
Exercise - Customize your own Smart Pointer in Rust

In Rust, you can achieve automatic memory deallocation using the Drop trait
1.  Implement the Drop trait for our custom pointer.  It should include a 
    function called specifically drop and print in the function a completion
    run of the drop object from memory.
*/
use std::ops::{Drop,Deref};
struct CustomSmartPointer<T>(T);

impl<T>CustomSmartPointer<T> {
    // Generic Structure with static method
    fn heap_alloc(value:T) -> CustomSmartPointer<T> {
        return CustomSmartPointer(value);
    }
}

// implement deref trait

impl<T> Deref for CustomSmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        return &self.0; // syntax for taking the first argument which is 0
    }
}

impl<T> Drop for CustomSmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data!");
    }
}

fn main() {

    let color = "green";
    let ref_color = CustomSmartPointer::heap_alloc(color);
    println!("green value is the same as color, which is {}","green"==color);
    println!("green value is the same as color, which is {}","green"==*ref_color);


    // What is box and derefencing with the unary * symbol?
    let greeting = "hello";
    let greeting_heap = Box::new(greeting); //Points to a new variable in the heap
    
    println!("{}", "hello"==greeting);
    println!("{}", "hello"==*greeting_heap);
}
