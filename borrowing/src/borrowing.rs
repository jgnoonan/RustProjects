/*
Borrowing in Rust

It can be quite a hastle transferring ownership of a variable to another function
and then returning ownership.  Through temporarily transferring ownership of a value,
Rust supports borrowing ownership in which the ownership is then returned to the 
original owner.

Functions can transfer their control of a value to another function temporarily and
that is what we call borrowing in Rust.  You can pass a reference to the variable with 
(& var_name) instead of passing the variable itself.  The ownership of the variable is
transferred to the original owner of the variable.

fn main() {    
    let vector = vec![1,2,3];
    display(&vector);
    println!("{}", vector[1]);
}

fn display(x:&Vec<i32>) {
    println!("{:?}", x);
}
*/

/*
Borrowing and References in Rust exercise:

Create a function called display2 which takes a string object as a parameter
and pushes onto the string an "F8 Tributo" which is an awesome type of Ferrari!

Create a variable car which is a string object with the default "Ferrari". Call
display2 passing in the car variable as the argument and print the modified
version of the variable car.

*/

fn display2(x: &mut String) {
    x.push_str("F8 Tributo");
    println!("Inside display2() function: {}", x);
}

fn main() {

    let mut car = String::from("Ferrari ");
    println!("{}", car);
    display2(&mut car);
    println!("Modified value from display2() function: {}", car);
}