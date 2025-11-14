use gat_lending_iterator::{LendingIterator, ToLendingIterator};

fn main() {
    println!("=== Fold Example ===\n");

    let data = vec![1, 2, 3, 4, 5];
    let sum = data.into_lending().fold(0, |acc, x| acc + x);
    println!("Sum using fold: {}", sum);

    let data = vec![1, 2, 3, 4, 5];
    let product = data.into_lending().fold(1, |acc, x| acc * x);
    println!("Product using fold: {}", product);

    println!("\n=== Sum and Product Methods ===\n");

    let data = vec![1, 2, 3, 4, 5];
    let sum: i32 = data.clone().into_lending().sum();
    println!("Sum using sum(): {}", sum);

    let product: i32 = data.into_lending().product();
    println!("Product using product(): {}", product);

    println!("\n=== Max and Min ===\n");

    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let max: i32 = data.clone().into_lending().max().unwrap();
    let min: i32 = data.into_lending().min().unwrap();
    println!("Max: {}, Min: {}", max, min);

    println!("\n=== Max/Min by Key ===\n");

    let words = vec!["a", "bb", "ccc", "dd", "e"];
    let longest: &str = words
        .clone()
        .into_lending()
        .max_by_key(|s| s.len())
        .unwrap();
    let shortest: &str = words.into_lending().min_by_key(|s| s.len()).unwrap();
    println!("Longest word: '{}', Shortest word: '{}'", longest, shortest);

    println!("\n=== All and Any ===\n");

    let data = vec![2, 4, 6, 8, 10];
    let all_even = data.clone().into_lending().all(|x| x % 2 == 0);
    println!("All even: {}", all_even);

    let any_greater_than_5 = data.into_lending().any(|x| x > 5);
    println!("Any > 5: {}", any_greater_than_5);

    println!("\n=== Count ===\n");

    let data = vec![1, 2, 3, 4, 5];
    let count = data.into_lending().filter(|&x| x > 2).count();
    println!("Count of elements > 2: {}", count);
}
