/*
Decision making structures in Rust

Decision making structures check statment evaluations and make various outputted decisions programattically based
on the conditions of the statements - whether they may be true or alternatives if they are false.

if statement
An if statement consists of a true or false expression followed by on or more statements.

if ... else statement
An if else statement in programming is a conditional statement that runs a different set of statements 
depending on whether an expression is true or false - Boolean

else ... if statement
You can nest one if or else if statement inside  another if or else if statement(s) and so on

match statement
A match statement, similar to the Switch statement in C, allows a variable to be tested against a list of values.

*/

/*
Exercise:  Decision Making Structures in Rust

1.  Write two signed 32 bit constants x and y and assign x the value of 3 and y the value 4.
2.  Check to see whether or no the x is less than y and x is greater than 6
3.  If the check passes, print to the console the following string: "fail"
4.  If the check provides a fals boolean, conduct the following tests:
    check if x is less than y or x is greater than 5
    If the second test passes, print the following string to the console: "success"
    if the secone test fails, print the following string to the console: "please try again"
*/



fn main() {

    const X:i32 = 3;
    const Y:i32 = 4;

    if (X < Y) && (X > 6) {
        println!("fail");
    } else if (X < Y) || (X > 6) {
        println!("success");
    } else {
        println!("Please try again");
    }
}
