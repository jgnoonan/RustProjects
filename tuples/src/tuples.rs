/*
Tuples in Rust

Tuples are compound data types.  A scalar type, or the variables
we've seen so far, can only store one type of data.  For example,
a u32 variable will only store a single integer value.  Compound
types can store multiple values at the same time of different types.

Tuples have a fixed length - once declared, they cannot grow or shrink
in size.  The Tuple index starts at 0.
*/

/*
Destructung tuples in Rust
1.  Create a function user_data which takes the tuple x as a parameter containing a
signed integer 32 bits, a boolean, and a string literal (hint: use the keyword &str to
point to the reference).
2.  In the function assign a tuple to distinct variables by naming the integer: age, 
boolean: active, string literal name.  (BIG HINT: let  (integer, bool and string) = x - 
what you're doing here is assigning a tuple to distinct types).
3.  Write instructions in the function to print age, active and name.
4.  In the main function, create a new tuple, user2 and set the user data so that the age 
is 30, active status is true, and his name is Jack.
5.  Invoke the user_data function, passing in user2 as the argument and check the console
for your results.
*/

fn user_data(x:(i32,bool,&str)) {
    let (age, active, name) = x;
    println!("Age: {}, isActive: {}, Name: {}", age, active, name);
}

fn main() {
    let user_data2:(i32,bool,&str) = (30,true,"Jack");
    user_data(user_data2);
}
