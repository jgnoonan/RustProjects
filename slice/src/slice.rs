/*

Slices in Rust

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

Slices essential point to data.  They are passed by reference to functions (i.e., borrowing).  You can use 
them to fetch portions of data and customize what you want to slice.

*/

fn main() {
    let game = "Mario Brothers".to_string();
    println!("Length of the game name is: {}", game.len());

    let slice = &game[0..5];
    println!("{}", slice);

    let mut nums: [i32;5] = [1,2,3,4,5];
    println!("{:?}", nums);
    slice_and_dice( &mut nums[1..3]);
    println!("Array after slice_and_dice(): {:?}", nums);
}

fn slice_and_dice(arr: &mut [i32]) {
    println!("The length of the array is: {}", arr.len());
    println!("Sliced array values: {:?}", arr);
    arr[0] = 200;
    println!("Updated array values: {:?}", arr);
}

/*

Exercise 1

1.  Create an array with values 1 through 5 called nums
2.  Print the values of nums
3.  Write a function called slice_and_dice which takes an array as a parameter and prints
    the length of the array.  It should also print the updated value of the array.  The function
    should also replace the first index value in the array with 200.
4.  Call the slice_dice function in the main function and slice out the values 2 and 3 from the
    array.

    There are a few little tricks here so be vigilent and good luck!
*/
