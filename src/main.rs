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
}
