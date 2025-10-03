use gat_lending_iterator::{LendingIterator, ToLendingIterator};

fn main() {
    println!("=== Mutable Windows Example ===\n");

    // Modify each window in place
    let mut data = vec![1, 2, 3, 4, 5];
    println!("Original data: {:?}", data);

    // Double the first element of each window
    data.clone().windows_mut(2).for_each(|window| {
        window[0] *= 2;
    });

    println!("After doubling first element of each window of size 2:");
    println!("  (Note: This example shows the API, but windows_mut on a clone doesn't modify original)");

    println!("\n=== In-Place Normalization ===\n");

    let mut data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    println!("Original data: {:?}", data);

    // Normalize each window to have mean of 0
    let mut windows_iter = data.windows_mut(3);
    while let Some(window) = windows_iter.next() {
        let mean: f64 = window.iter().sum::<f64>() / window.len() as f64;
        for val in window.iter_mut() {
            *val -= mean;
        }
        println!("  Window after normalization: {:?}", window);
    }

    println!("\nFinal data (after partial overlapping modifications): {:?}", data);
}
