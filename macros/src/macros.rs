fn main() {

    // in general, {} will be automatically replaced with any argument
    // and stringified
    println!("{} years old", 63);

    // named arguments also work
    println!("{user1}{action}{user2}",
            user1 = "Mary Lewis",
            user2 = "Jon Wick",
            action = " karated kicked ");

    println!("{x}, {y}",
        x = "hello",
        y = "my friend");

}
