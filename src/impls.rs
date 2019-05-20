pub use super::structs::PipeBox;
pub use super::traits::{Pipe, PipeFunc, PipeMap, PipeUnwrap, PipeVal};

impl<'a, PipeBox> Pipe<'a> for PipeBox
where PipeBox:
    PipeFunc<'a> +
    PipeVal<'a> +
    PipeMap<'a> +
    PipeUnwrap
{}

impl<'a, Value: 'a> PipeFunc<'a> for PipeBox<'a, Value> {
    type Value = Value;

    fn from_func(func: impl 'a + FnOnce() -> Self::Value) -> Self {
        PipeBox {
            func: Box::from(func),
        }
    }
}

impl<'a, Value: 'a> PipeVal<'a> for PipeBox<'a, Value> {
    type Value = Value;

    fn from_val(val: Value) -> Self {
        Self::from_func(|| val)
    }
}

impl<'a, Value: 'a> PipeMap<'a> for PipeBox<'a, Value> {
    type Value = Value;

    fn map<Next: 'a>(self, func: impl 'a + FnOnce(Value) -> Next) -> PipeBox<'a, Next> {
        PipeBox::<'a, Next>::from_func(|| func(self.unwrap()))
    }
}

impl<'a, Value: 'a> PipeUnwrap for PipeBox<'a, Value> {
    type Value = Value;

    fn unwrap(self) -> Value {
        (*self.func)()
    }
}
