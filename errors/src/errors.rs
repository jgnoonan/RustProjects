/*
Error Handling in Rust

In programming there are recoverable errors.  Recoverable errors are errors which do not
cause the program to fail for example: 'The file type you specified is incorrect'

Unrecoverable errors caust the program to fail such as trying to access locations that are
outside of a data structure.

This allows a program to terminate immediately and provide feedback to the caller of the 
program.

This macro is the perfect way to assert conditions in example code and in tests.  panic! is 
closely tied with the unwrap method of both Option and Result enums.  Both implementations
call panic! when they are set to None or Err.

1.  unwrap
unwrap(self): T
Expects self to be Ok/Some and returns the value contained within.  If it is Err or None
instead, it raises a panic with the contents of the error.

2.  expect
expect(slef, msg: &str): T
Behaves like unwrap, except that it outputs a custom message before panicking in addition
to the contents of the error.
*/

/*
Exercise - Write Error Handling for a program in Rust

1.  Write a function called is_seven which checks whether or not the input is the number 7 and
    returns true if so or an error (i.e., return Err("...")) if it is not true.  If it is true Ok(?)
    these variants Err and Ok come from the enum Result.  The function can return multiple data
    types.
2.  Create a variable "solution" in the main function which is assigned by calling the is_seven()
    function testing various inputs.
3.  Print the solution variable in the program.
*/

fn is_seven(x:i32) -> Result<bool, String> {
    if x == 7 {
        return Ok(true)
    }
    else {
        return Err("Input value is not 7".to_string())
    }
}

fn main() {    
   let solution = is_seven(9).unwrap();
   println!("Solution: {}", solution);
}
