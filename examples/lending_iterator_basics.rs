use gat_lending_iterator::{LendingIterator, ToLendingIterator};

fn main() {
    println!("=== Converting Iterator to LendingIterator ===\n");

    // Use into_lending to convert a regular iterator
    let data = vec![1, 2, 3, 4, 5];
    let sum: i32 = data.into_iter().into_lending().fold(0, |acc, x| acc + x);
    println!("Sum using lending iterator: {}", sum);

    println!("\n=== Using lend_refs ===\n");

    // lend_refs creates a lending iterator that yields references
    let data = vec![String::from("hello"), String::from("world")];
    data.iter()
        .lend_refs()
        .for_each(|s| println!("  {}", s));

    println!("\n=== Chaining Methods ===\n");

    // Chain multiple lending iterator methods
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = data
        .into_lending()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .take(3)
        .into_iter()
        .collect();

    println!("Even numbers squared, first 3: {:?}", result);

    println!("\n=== Using enumerate ===\n");

    let data = vec!["a", "b", "c"];
    data.into_lending()
        .enumerate()
        .for_each(|(i, val)| println!("  Index {}: {}", i, val));

    println!("\n=== Using zip ===\n");

    let numbers = vec![1, 2, 3];
    let letters = vec!["one", "two", "three"];
    numbers
        .into_lending()
        .zip(letters.into_lending())
        .for_each(|(n, l)| println!("  {} -> {}", n, l));
}
