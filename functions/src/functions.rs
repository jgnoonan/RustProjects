/*
Functions are the shells that contain the blocks of maintainable and reusable code.
Functions are "self contained" modules of code that accomplish a specific task.
Functions usually "take in" data, process it and "returns" a result.

Once defined. functions may be called to access code.  This makes the code reusable.
Moreover, functions make it easy to read and maintain the program's code.

Many programming languages have built-in functions that you can access in their libraries,
but you can also create your own functions.

Function and Description
1) Building a function
A Function contains the instructions on what and how to perform code

2) Calling or invoking a function
A function must be called to execute it.

3) Returning functions
Functions may also return values back to the caller

4)  Parameterize Functions
Parameters are a mechanism to pass values to functions


*/

fn fn_function () {
    println!("Hello!  I am inside fn_function");
}

// some functions can have return statements that return value back to the caller

fn fn_return() -> bool {
    return true;
}

// function with parameters
fn calculate_age(birthyear:i32, currentyear:i32) -> i32 {
    return currentyear - birthyear;
}

fn main() {
    fn_function();
    let fn_return: bool = fn_return();
    println!("Function returned: {}", fn_return);
    println!("My current age is: {}", calculate_age(1961, 2024));
}
