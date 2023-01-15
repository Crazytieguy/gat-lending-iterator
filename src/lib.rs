mod adapters;
mod to_lending;
mod traits;
pub use self::adapters::*;
pub use self::to_lending::*;
pub use self::traits::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn second(slice: &[usize]) -> &usize {
        &slice[1]
    }

    #[test]
    fn playground() {
        let debug = |x: &usize| println!("{:?}", *x);
        (0..5)
            .windows(3)
            .filter(|x| x[0] % 2 == 0)
            .map(second)
            .for_each(debug);

        println!();

        for sum in (0..7).windows_mut(2).map(|slice: &mut [usize]| {
            slice[1] += slice[0];
            slice[1]
        }) {
            println!("{}", sum);
        }

        println!();

        for n in (0..5).windows(3).map(second).cloned() {
            println!("{}", n);
        }
    }
}
