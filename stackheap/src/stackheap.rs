/*
** Ownership in Rust **

The Stack vs the Heap
Each method has its own stack designed for primitives (when the method is done, you can clean the stack)
Pointer on the stack will point to an object

Structs could be on the heap (not as efficient and fast and has to find spece to allocate for it)

Rust does not use a garbage collector, but rather achieves these properties through a sophisticated 
but complex, type system.  Doing so makes Rust very efficient, but makes Rust very hard to learn and use.


**********

The stack is very fast and is where memory is allocated in Rust by default, but the 
allocation is local to a function call, and is limited in in size.  The heap, on the
other hand is slower, and is explicity allocated by your program.  It's effectively
unlimited in size, and is globally accessible.  Note this meaning of heap, which allocates 
arbitrary-sized blocks of memory in arbitrary order, is quite different from the heap data
structure.

When a function gets called, some memory gets allocated for all of the function's local
variables and some other information.  This is called the "stack frame."  

So if the stack is faster and easier to manage, why do we need the heap?  A big reason
is that stack allocation alone means you only have "Last In, First Out (LIFO)" semantics for
reclaiming storage.  Heap allocation is strictly more general, allowing storage to be taken from
and returned to the pool in arbitrary order, but at a complexity cost

Generally, you should prefer stack allocations, and so Rust stack-allocates by default.
The LIFO model of the stack is simpler at a fundamental level.  This has two big impacts:
runtime efficiency and semantic impact.

*/

/*
Variables of a value have an owner in Rust.
All the data that is stored in Rust has an owner associated with it.

let time = 20  in this case, time is the owner of the value 20

Data has one owner at a time.
Multiple variables cannot point to the same memory location.
Variables always point to different memory locations.

That being said, you can transfer ownership in Rust:

Assigning one variable's value to another variable.
Inputting value to a function.
Outputting value from a function.

A vector is a data structure that contains zero-or-more items of the same type.
The items have an order, and you can access those items by the index (0,1, ...)
You can add and remove items.  The items themselves are stored in a contiguous heap-allocated 
area.
*/

fn main() {
    // vector vector1 owns the object in the heap
    // only a single variable owns the heap memory 
    // at a given time.
    let vector1 = vec![2, 4, 6];
    let vector2 = vector1;

    // Rust is going to check for memory access and that's
    // really a key selling point for Rust.

    println!("{:?}", vector2);
}
