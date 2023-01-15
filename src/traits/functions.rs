pub trait SingleArgFnOnce<Arg> {
    type Output;

    fn call_once(self, arg: Arg) -> Self::Output;
}

impl<F, Arg, O> SingleArgFnOnce<Arg> for F
where
    F: FnOnce(Arg) -> O,
{
    type Output = O;

    fn call_once(self, arg: Arg) -> O {
        self(arg)
    }
}

pub trait SingleArgFnMut<Arg>: SingleArgFnOnce<Arg> {
    fn call_mut(&mut self, arg: Arg) -> Self::Output;
}

impl<F, Arg, O> SingleArgFnMut<Arg> for F
where
    F: FnMut(Arg) -> O,
{
    fn call_mut(&mut self, arg: Arg) -> Self::Output {
        self(arg)
    }
}

pub trait SingleArgFn<Arg>: SingleArgFnMut<Arg> {
    fn call(&self, args: Arg) -> Self::Output;
}

impl<F, Arg, O> SingleArgFn<Arg> for F
where
    F: Fn(Arg) -> O,
{
    fn call(&self, args: Arg) -> Self::Output {
        self(args)
    }
}
