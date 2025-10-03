use gat_lending_iterator::{LendingIterator, ToLendingIterator};

fn main() {
    println!("=== Basic Windows Example ===\n");

    // Create overlapping windows of size 3
    let data = vec![1, 2, 3, 4, 5];
    let mut windows_iter = data.windows(3);

    println!("Data: {:?}", data);
    println!("Windows of size 3:");
    while let Some(window) = windows_iter.next() {
        println!("  {:?}", window);
    }

    println!("\n=== Computing Sums of Windows ===\n");

    // Use map to compute sum of each window
    let data = vec![10, 20, 30, 40, 50];
    let sums: Vec<i32> = data
        .windows(3)
        .map(|window| window.iter().sum())
        .into_iter()
        .collect();

    println!("Data: {:?}", data);
    println!("Sums of windows of size 3: {:?}", sums);

    println!("\n=== Finding Maximum in Each Window ===\n");

    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let maxes: Vec<i32> = data
        .windows(3)
        .map(|window| *window.iter().max().unwrap())
        .into_iter()
        .collect();

    println!("Data: {:?}", data);
    println!("Maximum in each window of size 3: {:?}", maxes);
}
