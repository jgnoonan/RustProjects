fn main() {
    /*  Integers in Rust
        Integers in Rust are numbers withouyt decimals or a number that is not a fraction.
        Simply put, integers are data types that represent whole numbers.

        Within the umbrella of integers, you have signed and unsigned versions that you can specify

        Signed integers store both negative and postive values
        Unsigned integers store positive values only

        In addition, the size of an integer can be set to arch.
        Setting an integer to arch derives the size of the integer from the architecture of the machine, 
        i.e., if you set if to 64bits then x64 is the machine type.

    */

    let total = 4;
    let height: u32 = 72;
    let deduction = 2-200;
    let overflow: u16 = 65535;
    let overtime_1: u16 = 65535;
    let overtime_2: u16 = 65535;

    println!("The total is: {}", total);
    println!("My height is {} inches", height);
    println!("My deduction is $ {}.00", deduction);
    println!("Overflow of u16: {}", overflow);
    println!("Overtime 1 value is {}", overtime_1);
    println!("Overtime 2 value is {}", overtime_2);
}

