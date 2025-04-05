fn main() {
    println!("Hello, world!");
}

/// This module is conditionally compiled only when running tests
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    // Import necessary modules for threading and sleeping
    use std::thread;
    use std::time::Duration;

    /// Marks this function as a test case
    #[test]
    fn threaded_test() {
        // Spawns a new thread
        thread::spawn(|| {
            // Loop from 0 to 5
            for i in 0..=5 {
                // Print a message from the spawned thread
                println!("hi number {} from the spawned thread!", i);
                // Pause execution for 1 second
                thread::sleep(Duration::from_secs(1));
            }
        });

        // Prints a message indicating that the main thread is joining
        println!("the main thread is being joined");

        // Sleep the main thread for 6 seconds to allow the spawned thread to complete
        thread::sleep(Duration::from_secs(6));
    }

    /*For the code below, I want to simulate the use of thread join in Rust. The join function is used to capture the returned result from a running thread's computation or value. Additionally, I will try to compare the execution time between running the function using threads and running it without using threads.*/

    // Marks this function as a test case
    #[test]
    fn join_threads_test() {
        // Spawns a new thread and returns a handle
        let handle = thread::spawn(|| {
            let mut counter = 0;
            // Loop from 0 to 5
            for i in 0..=5 {
                // Print the current counter value
                println!("counter: {}", i);
                // Pause execution for 1 second
                thread::sleep(Duration::from_secs(1));
                // Increment the counter
                counter += 1;
            }
            // Return the final counter value
            return counter;
        });

        // Print a message indicating that the main thread is waiting for the spawned thread
        println!("waiting for spawned thread");

        // Wait for the spawned thread to complete and capture its result
        let result = handle.join();

        // Match the result of joining the thread
        match result {
            // If successful, print the final counter value
            Ok(counter) => println!("the result is {}", counter),
            // If an error occurs, print the error details
            Err(error) => println!("the result is {:?}", error),
        }

        // Prints a message indicating that the main thread is being joined
        println!("the main thread is being joined");
    }

    /// Function to calculate a counter with a delay
    fn calculate_counter() -> i32 {
        let mut counter = 0;
        // Loop from 0 to 5
        for i in 0..=5 {
            // Print the current counter value
            println!("counter: {}", i);
            // Pause execution for 1 second
            thread::sleep(Duration::from_secs(1));
            // Increment the counter
            counter += 1;
        }
        // Return the final counter value
        counter
    }

    /// Test function for sequential processing
    #[test]
    fn sequential_process() {
        // Runs calculate_counter sequentially twice
        let result1 = calculate_counter();
        let result2 = calculate_counter();

        // Prints the results
        println!("Total counter 1: {}", result1);
        println!("Total counter 2: {}", result2);
        println!("Application finished!");

        // Estimated execution time: 12.05 seconds
    }

    /// Test function for parallel processing
    #[test]
    fn parallel_process() {
        // Spawns two threads that run calculate_counter concurrently
        let handle1 = thread::spawn(|| {calculate_counter()});
        let handle2 = thread::spawn(|| {calculate_counter()});

        // Print waiting message
        println!("waiting for calculation . . . ");

        // Wait for both threads to complete and capture their results
        let result1 = handle1.join();
        let result2 = handle2.join();

        // Match results for both threads
        match result1 {
            Ok(result) => println!("the result is {}", result),
            Err(error) => println!("the result is {:?}", error),
        }
        match result2 {
            Ok(result) => println!("the result is {}", result),
            Err(error) => println!("the result is {:?}", error),
        }

        // Print Application finished message
        println!("Application finished!");

        // Estimated execution time: 6.02 seconds
    }

    /// Test function for closure using `move`
    #[test]
    fn closure_using_move_test() {
        // Create a String variable
        let name = String::from("Gege");

        // Define a closure that moves ownership of `name`
        let closure = move || {
            println!("Hello, {}!", name);
            thread::sleep(Duration::from_secs(2));
        };

        // Spawn a new thread and execute the closure
        let handle = thread::spawn(closure);
        // Wait for thread to complete and capture the results
        handle.join().unwrap();

        // This line would cause a compilation error because `name` has been moved into the closure
        // println!("Hello, {}!", name);
        // Error: `name` is no longer accessible here due to ownership transfer
    }

    /// Test function for message passing between threads
    #[test]
    fn channel_test() {
        // Create a channel for sending and receiving messages
        let (sender, receiver) = mpsc::channel();

        // Spawn a thread to send a message after a delay
        let handle1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            // Send a message to the receiver
            sender.send("Hello, World!".to_string()).unwrap();
        });

        // Spawn a thread to receive the message
        let handle2 = thread::spawn(move || {
            // Wait for and receive the message
            let message = receiver.recv().unwrap();
            // Print the received message
            println!("The message is {}", message);
        });

        // Wait for both threads to complete
        handle1.join().unwrap();
        handle2.join().unwrap();
    }

    /// Test function for message queuing using a channel
    #[test]
    fn channel_queue_test() {
        // Create a channel for sending and receiving messages
        let (sender, receiver) = mpsc::channel();

        // Spawn a thread that will send messages to the receiver
        let handle1 = thread::spawn(move || {
            // Loop 5 times to send the same message repeatedly
            for _ in 0..5 {
                // Pause for 2 seconds between messages
                thread::sleep(Duration::from_secs(2));
                // Send the "Hello, World!" message via the channel, converting it to a String
                sender.send("Hello, World!".to_string()).unwrap();
            }
            // After sending messages, send a "done" signal to indicate the end of messages
            sender.send("done".to_string()).unwrap();
        });

        // Spawn a thread that will continuously receive messages from the channel
        let handle2 = thread::spawn(move || {
            loop {
                // Block until a message is received; unwrap to handle potential errors
                let message = receiver.recv().unwrap();
                // If the received message is "done", break out of the loop to end processing
                if message == "done" {
                    break;
                }
                // Otherwise, print the received message
                println!("The message is {}", message);
            }
        });

        // Wait for both threads to finish their execution
        handle1.join().unwrap();
        handle2.join().unwrap();
    }

    /// Test function for iterating over messages from a channel using its iterator implementation
    #[test]
    fn channel_iterator_test() {
        // Create a new channel for sending and receiving messages
        let (sender, receiver) = mpsc::channel();

        // Spawn a new thread that will send messages through the channel
        let handle1 = thread::spawn(move || {
            // Loop 5 times to send messages repeatedly
            for _ in 0..5 {
                // Pause execution for 2 seconds to simulate delay between messages
                thread::sleep(Duration::from_secs(2));
                // Send the "Hello, World!" message (converted to String) via the channel and unwrap to handle errors
                sender.send("Hello, World!".to_string()).unwrap();
            }
        });

        // Spawn another thread that will receive messages using the receiver's iterator implementation
        let handle2 = thread::spawn(move || {
            // Iterate over each incoming message from the receiver until the channel is closed
            for value in receiver {
                // Print the received message
                println!("The message is {}", value);
            }
        });

        // Wait for the sending thread to finish execution
        handle1.join().unwrap();
        // Wait for the receiving thread to finish execution
        handle2.join().unwrap();
    }

    /// Test function for demonstrating a multi-sender channel using mpsc::channel in Rust
    #[test]
    fn channel_multi_sender_test() {
        // Create a new channel that provides a sender and a receiver for message passing
        let (sender, receiver) = mpsc::channel();
        // Clone the original sender to allow multiple threads to send messages through the same channel
        let sender2 = mpsc::Sender::clone(&sender);

        // Spawn a thread (handle3) that will send messages using the cloned sender (sender2)
        let handle3 = thread::spawn(move || {
            // Loop 5 times to send a series of messages
            for _ in 0..5 {
                // Pause execution for 1 second to simulate some delay
                thread::sleep(Duration::from_secs(1));
                // Send a message from sender2 and immediately unwrap the result to handle potential errors
                sender2.send("Hello, World! from sender 2".to_string()).unwrap();
            }
        });

        // Spawn another thread (handle1) that will send messages using the original sender
        let handle1 = thread::spawn(move || {
            // Loop 5 times to send a series of messages
            for _ in 0..5 {
                // Pause execution for 2 seconds to simulate a different processing delay
                thread::sleep(Duration::from_secs(2));
                // Send a message from sender and immediately unwrap the result to handle potential errors
                sender.send("Hello, World! from sender 1".to_string()).unwrap();
            }
        });

        // Spawn a thread (handle2) that will receive messages from the channel
        let handle2 = thread::spawn(move || {
            // Iterate over each message received from the channel until it is closed
            for value in receiver {
                // Print each received message
                println!("The message is {}", value);
            }
        });

        // Wait for the sender and receiver threads to finish execution
        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
    }

    /// Test function for demonstrating a race condition
    ///
    // Declare a mutable static variable COUNTER with an initial value of 0
    static mut COUNTER: i32 = 0;

    #[test]
    fn race_condition_test() {
        // Create a vector to store thread handles
        let mut handles = vec![];

        // Spawn 10 threads
        for _ in 0..10 {
            let handle = thread::spawn(|| unsafe {
                // Each thread increments the COUNTER variable 1,000,000 times
                for _ in 0..1_000_000 {
                    COUNTER += 1;
                }
            });
            // Store the thread handle in the vector
            handles.push(handle);
        }

        // Wait for all spawned threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Print the final value of COUNTER
        println!("Counter: {}", unsafe { COUNTER });

        // **Race Condition Explanation:**
        // Since multiple threads modify the same shared variable (COUNTER) without synchronization,
        // it can lead to data races. The final value of COUNTER might not be as expected
        // because of concurrent read/write operations leading to lost updates.
    }

    /// Test function demonstrating the use of atomic operations to avoid race conditions
    #[test]
    fn atomic_test() {
        // Import AtomicI32 and memory ordering enum from the standard library
        use std::sync::atomic::{AtomicI32, Ordering};

        // Declare an atomic counter with an initial value of 0.
        // Using an atomic variable ensures that concurrent modifications are safe.
        static COUNTER_NEW: AtomicI32 = AtomicI32::new(0);

        // Create a vector to store thread handles
        let mut handles = vec![];

        // Spawn 10 threads to increment the atomic counter concurrently
        for _ in 0..10 {
            let handle = thread::spawn(|| {
                // Each thread increments the atomic counter 1,000,000 times
                for _ in 0..1_000_000 {
                    // Atomically add 1 to COUNTER_NEW using relaxed memory ordering.
                    // The relaxed ordering allows the counter to be incremented without additional synchronization,
                    // which is acceptable in this simple counting scenario.
                    COUNTER_NEW.fetch_add(1, Ordering::Relaxed);
                }
            });
            // Store the thread handle in the vector
            handles.push(handle);
        }

        // Wait for all spawned threads to complete their execution
        for handle in handles {
            handle.join().unwrap();
        }

        // Load and print the final value of the atomic counter using relaxed memory ordering.
        // The use of atomics here prevents data races, ensuring the correct final count.
        println!("Counter: {}", { COUNTER_NEW.load(Ordering::Relaxed) });

        // **Explanation:**
        // This code uses `AtomicI32` to safely share and update a counter across multiple threads.
        // Atomic types in Rust, such as `AtomicI32`, provide a way to perform lock-free concurrent modifications.
        // The operations such as `fetch_add` and `load` are guaranteed to be atomic,
        // meaning that they are performed as a single, indivisible operation.
        //
        // For more details, refer to the official Rust documentation on atomics:
        // https://doc.rust-lang.org/std/sync/atomic/index.html
    }

    /// Test function demonstrating the use of `Arc<AtomicI32>` to prevent race conditions in a multithreaded context.
    #[test]
    fn atomic_arc_test() {
        // Import necessary atomic types and memory ordering from the Rust standard library
        use std::sync::{Arc, atomic::{AtomicI32, Ordering}};

        // Create an `Arc<AtomicI32>` instance to allow multiple threads to share and update a single atomic counter
        let counter_new: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));

        // Create a vector to store thread handles
        let mut handles = vec![];

        // Spawn 10 threads, each of which will increment the shared atomic counter
        for _ in 0..10 {
            // Clone the `Arc` pointer to share ownership of `counter_new` across threads
            let counter_new_clone = Arc::clone(&counter_new);
            let handle = thread::spawn(move || {
                // Each thread increments the atomic counter 1,000,000 times
                for _ in 0..1_000_000 {
                    // Perform an atomic addition to ensure thread-safe updates
                    counter_new_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            // Store the thread handle in the vector
            handles.push(handle);
        }

        // Wait for all threads to complete their execution
        for handle in handles {
            handle.join().unwrap();
        }

        // Load and print the final value of the atomic counter
        println!("Counter: {}", counter_new.load(Ordering::Relaxed));

        // **Explanation:**
        // This implementation utilizes `Arc<AtomicI32>` to safely share and modify a counter across multiple threads.
        // `Arc` (Atomic Reference Counting) allows multiple threads to hold references to the same counter,
        // while `AtomicI32` ensures that increments are performed atomically, avoiding data races.
        //
        // Why use `Arc`?
        // - Unlike `static mut`, which can lead to race conditions, `Arc` ensures safe shared ownership across threads.
        // - Since `Arc` itself is thread-safe, it allows multiple threads to modify the counter without needing explicit locks.
        //
        // Why use `AtomicI32`?
        // - Atomic operations like `fetch_add` are lock-free, meaning they avoid the performance overhead of mutex locks.
        // - This ensures high-performance concurrent updates to the shared counter.
        //
        // Reference:
        // - `Arc`: https://doc.rust-lang.org/std/sync/struct.Arc.html
        // - `AtomicI32`: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI32.html
    }

    /// Test function demonstrating the use of `Arc<Mutex<i32>>` to avoid race conditions
    #[test]
    fn mutex_test() {
        // Import necessary synchronization primitives from the standard library
        use std::sync::{Arc, Mutex};
        use std::thread;

        // Create an atomic reference-counted (Arc) Mutex wrapping an i32 value.
        // `Arc` allows multiple threads to share ownership of the mutex.
        // `Mutex` ensures that only one thread can access the counter at a time.
        let counter_new: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

        // Create a vector to hold the thread handles
        let mut handles = vec![];

        // Spawn 10 threads
        for _ in 0..10 {
            // Clone the Arc to share ownership of the mutex across threads
            let counter_new_clone = Arc::clone(&counter_new);
            let handle = thread::spawn(move || {
                // Each thread increments the counter 1,000,000 times
                for _ in 0..1_000_000 {
                    // Lock the mutex to get mutable access to the data
                    // If the lock is poisoned, unwrap will panic
                    let mut data = counter_new_clone.lock().unwrap();
                    // Increment the counter by 1
                    *data += 1;
                    // Mutex is automatically unlocked when `data` goes out of scope
                }
            });
            // Save the thread handle for later joining
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Lock the mutex one last time to read and print the final counter value
        println!("Counter: {}", *counter_new.lock().unwrap());

        // **Explanation:**
        // This code uses `Arc<Mutex<i32>>` to allow multiple threads to safely update a shared counter.
        // The `Mutex` ensures that only one thread at a time can modify the counter, thus preventing race conditions.
        // `Arc` (Atomic Reference Counting) is used to share the mutex safely across threads.
        //
        // For more details, refer to the official Rust documentation on Mutex:
        // https://doc.rust-lang.org/std/sync/struct.Mutex.html
    }

}
