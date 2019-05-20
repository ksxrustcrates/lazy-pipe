use super::structs::PipeBox;

pub trait Pipe<'a>: PipeFunc<'a> + PipeVal<'a> + PipeMap<'a> + PipeUnwrap {}

pub trait PipeFunc<'a> {
    type Value: 'a;

    fn from_func(func: impl 'a + FnOnce() -> Self::Value) -> Self;
}

pub trait PipeVal<'a> {
    type Value: 'a;

    fn from_val(val: Self::Value) -> Self;
}

pub trait PipeMap<'a> {
    type Value: 'a;

    fn map<Next: 'a>(self, func: impl 'a + FnOnce(Self::Value) -> Next) -> PipeBox<'a, Next>;
}

pub trait PipeUnwrap {
    type Value;

    fn unwrap(self) -> Self::Value;
}
