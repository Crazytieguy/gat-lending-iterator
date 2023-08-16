//! Placeholders for the unstable fn_traits

pub trait SingleArgFnOnce<Arg>: FnOnce(Arg) -> <Self as SingleArgFnOnce<Arg>>::Output {
    type Output;
}

impl<F, Arg, O> SingleArgFnOnce<Arg> for F
where
    F: FnOnce(Arg) -> O,
{
    type Output = O;
}

pub trait SingleArgFnMut<Arg>:
    SingleArgFnOnce<Arg> + FnMut(Arg) -> <Self as SingleArgFnOnce<Arg>>::Output
{
}

impl<F, Arg, O> SingleArgFnMut<Arg> for F where F: FnMut(Arg) -> O {}

pub trait SingleArgFn<Arg>:
    SingleArgFnMut<Arg> + Fn(Arg) -> <Self as SingleArgFnOnce<Arg>>::Output
{
}

impl<F, Arg, O> SingleArgFn<Arg> for F where F: Fn(Arg) -> O {}
