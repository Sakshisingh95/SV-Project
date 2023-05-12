use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let numbers = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    let mut handles = Vec::new();

    // Spawn two threads, each working on half of the array
    for _ in 0..2 {
        let numbers = numbers.clone();
        let handle = thread::spawn(move || {
            let chunk = numbers.lock().unwrap();
            let sum: i32 = chunk.iter().sum();
            sum
        });
        handles.push(handle);
    }

    // Wait for the threads to finish and collect their results
    let result: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    println!("Sum of the array is: {}", result);
}

