/*
What are arrays in Rust?

An array is a collection of objects of the same type T, 
stored in contiguous memory.  Arrays are created using brackets [], 
and their length, which is know at compile time, is part of their
type signature [T; length]

Defining Arrays

An array consists of sequential memory blocks.
Arrays are static.  Arrays cannot be resized once they are initialized.
Each memory block represents an element in the array.
Array elements are identified by the index.
Array element values can be updated or modified but cannot be deleted.
*/

/*
Loop and mutate an array in Rust

1.  Create an array with the following integer values: 12,2,3,4,5
2.  Write a for loop which can loop through the array and replace all the 
    integer values of 2 with 0.  Print the amended results taged along with
    the index value of each integer.
*/

fn main() {
    let mut int_array = [12,2,3,4,5,2,2,3,0];
    for (index,value) in int_array.iter_mut().enumerate() {
        if *value == 2 {
            *value = 0;
        }
        println!("Value {:?} in at Index {}", value, index);
    }
    println!("{:?}", int_array);
}
