fn main() {
    println!("Hello, world!");
}

/// This module is conditionally compiled only when running tests
#[cfg(test)]
mod tests {
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
}
