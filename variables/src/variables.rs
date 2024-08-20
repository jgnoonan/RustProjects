/*
The Variable Rule Book in Rust

When it comes to Rust, there are different ways to name a variable

1.  You can name a variable with letters, digits, and the underscore character
2.  Variables must begin with either an underscore or a letter
3.  Since Rust is case sensitive, uppercase and lowercase varaibles are distinct

Data types do not have to be statically declared and are optional.
The data type is applied from the value assigned to the variable.

Immutability

Variables are immutable by default.  We can apply the mut keyword to make
variables mutable

*/

fn main() {
  let mut x = 5;
  x = x+1;
  println!("{}", x);

  let mut y = 12;
  y = y+3;
  println!("{}", y);

  let mut switch = false;
  let volume = 10;

  if volume >= 0 {
    switch = true;
  }
println!("The state of switch is: {}", switch);
println!("The volume level is set to: {}", volume);
  

}
