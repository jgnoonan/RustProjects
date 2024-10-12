/*
Concurrency in Rust

When you want to have different parts of a program execute independently, you are looking at
the concept of concurrency.  Whereas with parallel programming you have different parts of the
propgram or computers executing at the same time.  Both of these models stem from the core 
feature of multiple processing.

Threads

We use threads to run code blocks at the same time.  Operating systems manage multiple processes 
at once and programs execute code in a process.  When you have independent parts of your code that 
run together you are using the feature of multi threaded programming.

How to create a thread in Rust

To create a thread in Rust, you use the thread::spawn function.  This function takes a closure as
an argument and runs it in a new thread.  The closure can capture values from the scope in which
it is defined.  The thread::spawn function returns a JoinHandle type that allows you to wait for the
thread to finish.

Every thread is equipped with some basic low-level blocking support via the thread::park function.
and the thread::Thread::unpark method.  Park blocks the current thread, which then can be resumed
from another thread by calling unpark on the parked thread.
*/

/*
Execercise - Build your own threads in Rust

1.  Create two deinite loops in the main function.
2.  One loops should be set to spawn a thread z which runs from 1 to 20 with a sleep duration
    of half the time of the main thread loop.  The spawned thread should also print the index
    values of each iteration as it loops until completion.
3.  The other loop should run in the main thread with a sleep[ duration of twice the time of the
    spawned thread loop.  The loop should run one from 1 to 5 and should also print each index value
    tagged as the main thread
4.  Ensure the span thread has a full completion of its rrun even if the main thread finishes.    
    
*/

use std::thread;
use std::time::Duration;

fn main() {

    let x = thread::spawn(|| {
        for i in 1..20 {
            println!("spawned x thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let y = thread::spawn(|| {
        for i in 1..40 {
            println!("spawned y thread: {}", i);
            thread::sleep(Duration::from_millis(150));
        }
    });

    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(200));
    }

    x.join().unwrap();
    y.join().unwrap();
}
