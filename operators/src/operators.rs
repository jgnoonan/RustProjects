/*
What are Operators in Rust

An operator is a character or characters that determine the action that is to be performed or considered.
The data on which operators are actioned  are called operands.  Consider the following expression -
a + b = c
Here, the values of a and be and c are operands, while + and = are operators

The major Operations in Rust to consider are the following:

Arithmetic
Bitwise
Comparison
Logical
Conditional

Arithmetic Operators
x = 12
y = 2

Sr.No   Operator    Description/Example
1       +(Addition) returns the sum of the operands x+y
2       -(Subtract) returns the difference of the values x - y
3       *(Multiply) returns the product of the values x * y
4       /(Division) performs the division operation and returns the quotient x / y
5       %(Modulus)  performs the division operation and returns the remainder x % y

Relational Operators
Relational Operators check or define the relationship equivalency
between two elements.  Relational operators are used to compare two or more values.
Relationship operators return a Boolean value: either true or false.

x = 5
y = 2

1   >   Greater than                (x > y)     returns true
2   <   Less than                   (x < y)     returns false
3   >=  Greater than or equal to    (x >= y)    returns true
4   <=  Less thank or equal to      (x <= y)    returns false
5   ==  Equality                    (x == y)    returns false
6   !=  Not equal                   (x != y)    returns true

Logical Operators are used to combine and check two or more conditions.

Logical operators return a Boolean value

x = 1
y = 2

1   && (AND)    The operator returns true as long as all the expressions specified are true
                (x > 2 && y > 3) returns false
2   || (OR)     The operator returns true if at least one of the expressions returns true
                (x > 2 || x > 3)  returns false
3   ! (NOT)     The operator returns the inverse of the expression's result
                !(x < 4) returns false
*/


fn main() {

    let sum = 5+9;
    let dif = 9-5;
    let prod = 9*5;
    let quot = 10/5;
    let modulus = 9%5;
    println!("5 + 9 = {}", sum);
    println!("9 - 5 = {}", dif);
    println!("5 * 9 = {}", prod);
    println!("10 / 5 = {}", quot);
    println!("9 % 5 = {}", modulus);
}
