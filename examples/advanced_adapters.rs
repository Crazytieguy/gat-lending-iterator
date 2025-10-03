use gat_lending_iterator::{LendingIterator, ToLendingIterator};

fn main() {
    println!("=== Filter and Map ===\n");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = data
        .into_lending()
        .filter(|&x| x > 5)
        .map(|x| x * 2)
        .into_iter()
        .collect();

    println!("Numbers > 5, doubled: {:?}", result);

    println!("\n=== Take While ===\n");

    let data = vec![1, 2, 3, 4, 5, 4, 3, 2, 1];
    let result: Vec<i32> = data
        .into_lending()
        .take_while(|&x| x < 5)
        .into_iter()
        .collect();

    println!("Take while less than 5: {:?}", result);

    println!("\n=== Skip and Step By ===\n");

    let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result: Vec<i32> = data
        .into_lending()
        .skip(2)
        .step_by(2)
        .into_iter()
        .collect();

    println!("Skip 2, then every 2nd element: {:?}", result);

    println!("\n=== Scan with State ===\n");

    let data = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = data
        .into_lending()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .into_iter()
        .collect();

    println!("Running sum: {:?}", result);

    println!("\n=== Cloned and Copied ===\n");

    let data = vec![1, 2, 3, 4, 5];
    let refs: Vec<&i32> = data.iter().collect();

    // Use copied to convert &i32 to i32
    let values: Vec<i32> = refs
        .into_lending()
        .copied()
        .into_iter()
        .collect();

    println!("Copied values: {:?}", values);

    println!("\n=== Find and Position ===\n");

    let data = vec![10, 20, 30, 40, 50];
    let mut iter = data.into_lending();

    if let Some(value) = iter.find(|&x| x > 25) {
        println!("First value > 25: {}", value);
    }

    let data = vec![10, 20, 30, 40, 50];
    let mut iter = data.into_lending();

    if let Some(pos) = iter.position(|x| x > 25) {
        println!("Position of first value > 25: {}", pos);
    }
}
