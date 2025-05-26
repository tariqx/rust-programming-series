# Threads: Rust Programming Series

In Rust, running concurrent tasks can be achieved by using threads, and threads can be created by calling the thread::spawn function.

Here is an example of creating a simple thread in Rust:

``` rust 
use std::thread;

fn main(){
     thread::spawn(|| {
          println!("New Thread: Msg from new thread!");
     });
     println!("Main thread: running!");
}

```

**Output**:
```
Main thread: running!
New Thread: Msg from new thread!
```

Let's try to understand this simple code because we will be using this later on.

***thread::spawn( ...)***

First, thread is a module in Rust's standard library that provides functionality for working with parallel tasks. Spawn is a function within the thread module. Its purpose is to create and start a new thread of execution. When spawn is called, it returns a ***JoinHandle***, which can be used to wait for the new thread to finish.

***|| { ... }***

This is a closure, also known as an anonymous function or lambda in other languages. The double pipes || indicate that the closure can take arguments, quite similar to the parentheses () of a function. Later on, we will be passing an argument, such as move, into the closure for better handling of values. Also, we will talk about borrowing and ownership concepts that are unique to Rust. For now, the code inside the curly braces {...} is the code that will be executed by the newly spawned thread. In this case, it prints a statement.

**Note**: Another possible output could be that the new thread gets executed first, then the main thread, like so:

```
New Thread: Msg from new thread!
Main thread: running!
```

**Note**: This would be a classic example of a race condition in multithreaded programming, where the outcome depends on the unpredictable timing of multiple threads. Since this is a simple example, in more complex scenarios, race conditions can lead to bugs and incorrect program behavior.

Now let's try to expand our code so that we can increment a counter inside the new thread.

```rust
use std::thread;

fn main(){

    // declare a mutable variable in the main thread
    // and try to modify it in a new thread
    let mut counter = 0;

    // Spawn a new thread
    // and attempt to modify the counter variable.
    // Spawning a thread requires the variable to be moved 
    // or shared safely. Here, we pass the 'move' keyword in 
    // the closure to take ownership of the counter variable.
    let _handle = thread::spawn(move || {
        counter += 1; 
        println!("New Thread: counter = {}", counter);
    }); 

    // print the value of counter in the main thread
    println!("Main thread: counter = {}", counter);
}
```

**Output**:

```
Main thread: counter = 0
New Thread: counter = 1
```

If we removed the move keyword and try to run the code again:

```
error[E0373]: closure may outlive the current function, but it borrows `counter`, which is owned by the current function
  --> src\main.rs:14:33
   |
14 |     let _handle = thread::spawn(|| {
   |                                 ^^ may outlive borrowed value `counter`
15 |         counter += 1;
   |         ------- `counter` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> src\main.rs:14:19
   |
14 |       let _handle = thread::spawn(|| {
   |  ___________________^
15 | |         counter += 1;
16 | |         println!("New Thread: counter = {}", counter);
17 | |     });
   | |______^
help: to force the closure to take ownership of `counter` (and any other referenced variables), use the `move` keyword
   |
14 |     let _handle = thread::spawn(move || {
   |                                 ++++

error[E0502]: cannot borrow `counter` as immutable because it is also borrowed as mutable
  --> src\main.rs:20:43
   |
14 |       let _handle = thread::spawn(|| {
   |                     -             -- mutable borrow occurs here
   |  ___________________|
   | |
15 | |         counter += 1;
   | |         ------- first borrow occurs due to use of `counter` in closure
16 | |         println!("New Thread: counter = {}", counter);
17 | |     });
   | |______- argument requires that `counter` is borrowed for `'static`
...
20 |       println!("Main thread: counter = {}", counter);
   |                                             ^^^^^^^ immutable borrow occurs here

```

Rust compiler is pretty good at providing us with the details about the error, and also it gives us some hints on how to solve that error. When the new thread is spawned, it might outlive the scope where the closure was defined. If the closure only borrowed the variables, those borrowed variables become "dangling references" once the original owner (the main thread) goes out of scope. By using ***move***, we can ensure that the new thread takes ownership of all the data it needs, therefore guaranteeing that the data remains valid for the lifetime of the new thread.

Let's update our code to increment the counter one more time, add a delay to simulate a long process, and print the new updated counter value. We will also want to make sure that our new thread is completed safely before the program ends.

```rust
use std::thread;

fn main(){

    // declare a mutable variable in the main thread
    // and try to modify it in a new thread
    let mut counter = 0;

    // Spawn a new thread
    // and attempt to modify the counter variable
    // spawning a thread requires the variable to be moved 
    // or shared safely. Here, we use the move keyword in 
    // the closure to take ownership of the counter variable.
    let handle = thread::spawn(move || {
        counter += 1; 
        println!("New Thread: counter = {}", counter);
        thread::sleep(std::time::Duration::from_secs(5)); // simulate some work
        counter += 1; // increment the counter again
        println!("New Thread: counter after increment = {}", counter);
        // Note: This will not affect the counter in the main thread
    }); 

    handle.join().unwrap(); // wait for the thread to finish    
    // print the value of counter in the main thread
    println!("Main thread: counter = {}", counter);
}
```

**Output**:

```
New Thread: counter = 1
New Thread: counter after increment = 2
Main thread: counter = 0
```

As mentioned earlier, sharing variables between threads has its challenges, such as race conditions, deadlocks, and other hard-to-reproduce issues. Rust provides a solution for this common shared-state concurrency, which is part of its ownership principle and type safety. To enable safe shared-state concurrency, Rust offers the std::sync module: **Arc** (Atomic Reference Counted) and **Mutex** (Mutual Exclusion). 

**Arc**: Is a smart pointer that allows multiple owners of a single piece of data. It works by keeping a count of how many Arc pointers currently point to the data. When the last Arc pointer is dropped, the data is deallocated. 

**Mutex**: Is a synchronization primitive that ensures only one thread can access a shared resource at a time. It prevents data race conditions by providing a locking mechanism. If another thread already holds the lock, the current thread will block (wait) until the lock is released. 

**Specific code explanation**: 

```
Arc::clone(&counter);
```

This doesn't perform a deep copy of the original value, rather, it returns a new ***Arc*** smart pointer that points to the same value inside the counter variable. This is how the ***Arc*** enables multiple threads to "own" and share the same underlying data. Each ***Arc*** clone is an independent smart pointer, but they all refer to the same resource. 

```
*counter.lock().unwrap()
```

This attempts to acquire a lock on the ***Mutex***, and when it is successfully locked, it returns ***MutexGuard***, which is a smart pointer that dereferences to the data inside the variable. 

```
*num += 1; 

```

Dereferences the ***MutexGuard*** to get the actual i32 value of the counter variable and increment it by one. 

Here is the rest of updated code:

```Rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main(){

    // Create a counter variable that will be shared between threads
    // Arc (Atomic Reference Counted) is used to share ownership of 
    // the counter variable between threads. Mutex is used to protect
    // across threads safely and it is used to ensure that only one 
    // thread can access the counter at a time.
    let counter = Arc::new(Mutex::new(0));

    // Here, we use Arc::clone to create a new reference to the counter variable
    // that can be moved into the new thread.
    let counter_for_thread = Arc::clone(&counter);

    // Print the initial value of the counter
    println!("Main thread: counter = {}", *counter.lock().unwrap());


    // Spawn a new thread
    // and attempt to modify the counter variable
    // spawning a thread requires the variable to be moved 
    // or shared safely. Here, we use the move keyword in 
    // the closure to take ownership of the counter variable.
    let handle = thread::spawn(move || {
        // lock the counter to get mutable access
        // pointing to the counter variable
        let mut num = counter_for_thread.lock().unwrap();  
        *num += 1; // increment the counter
        println!("New Thread: counter = {}", *num);

        println!("New Thread: sleeping for 5 seconds...");
        thread::sleep(std::time::Duration::from_secs(5)); // simulate some work
        *num += 1; // increment the counter again
        println!("New Thread: counter after increment = {}", *num);

    }); 

    handle.join().unwrap(); // wait for the thread to finish    
    // print the value of counter in the main thread
    println!("Main thread: counter = {}", *counter.lock().unwrap());
}

```

**Output**:

```
Main thread: counter = 0
New Thread: counter = 1
New Thread: sleeping for 5 seconds…
New Thread: counter after increment = 2
Main thread: counter = 2

```
Comparing the latest output with the previous examples, the main thread counter is now synced with the changes made inside the new thread. Till next time!