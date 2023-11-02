//! Placeholders for the unstable `fn_traits` feature.

/// Placeholder for [`FnOnce`]
pub trait SingleArgFnOnce<Arg>: FnOnce(Arg) -> <Self as SingleArgFnOnce<Arg>>::Output {
    /// The output type of the function.
    type Output;
}

impl<F, Arg, O> SingleArgFnOnce<Arg> for F
where
    F: FnOnce(Arg) -> O,
{
    type Output = O;
}

/// Placeholder for [`FnMut`]
pub trait SingleArgFnMut<Arg>:
    SingleArgFnOnce<Arg> + FnMut(Arg) -> <Self as SingleArgFnOnce<Arg>>::Output
{
}

impl<F, Arg, O> SingleArgFnMut<Arg> for F where F: FnMut(Arg) -> O {}

/// Placeholder for [`Fn`]
pub trait SingleArgFn<Arg>:
    SingleArgFnMut<Arg> + Fn(Arg) -> <Self as SingleArgFnOnce<Arg>>::Output
{
}

impl<F, Arg, O> SingleArgFn<Arg> for F where F: Fn(Arg) -> O {}

/// Used in cases that a function needs to return an [`Option`]
/// who's lifetime is tied to the input.
pub trait OptionTrait {
    /// The type of the item in the option.
    type Item;

    /// Converts `self` into an `Option`.
    fn into_option(self) -> Option<Self::Item>;
}

impl<T> OptionTrait for Option<T> {
    type Item = T;

    fn into_option(self) -> Option<Self::Item> {
        self
    }
}
