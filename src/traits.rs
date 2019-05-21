use super::structs::PipeBox;

/// This trait is combination of all `Pipe*` traits
pub trait Pipe<'a>: PipeFunc<'a> + PipeVal<'a> + PipeMap<'a> + PipeUnwrap {}

/// This trait provides `from_func` method
pub trait PipeFunc<'a> {
    type Value: 'a;

    /// Start a pipeline by a function
    fn from_func(func: impl 'a + FnOnce() -> Self::Value) -> Self;
}

/// This trait provides `from_val` method
pub trait PipeVal<'a> {
    type Value: 'a;

    /// Start a pipeline by a value
    fn from_val(val: Self::Value) -> Self;
}

/// This trait provides `map` method and its `to` alias
pub trait PipeMap<'a> {
    type Value: 'a;

    /// Move to another node in the pipeline
    fn map<Next: 'a>(self, func: impl 'a + FnOnce(Self::Value) -> Next) -> PipeBox<'a, Next>;

    /// Alias to [`map`](#tymethod.map)
    fn to<Next: 'a>(
        self,
        func: impl 'a + FnOnce(<Self as PipeMap>::Value) -> Next
    ) -> self::PipeBox<'a, Next> {
        self.map(func)
    }
}

/// This trait provides `unwrap` method and its `get` alias
pub trait PipeUnwrap {
    type Value;

    /// Execute the pipeline and get the value
    fn unwrap(self) -> Self::Value;

    /// Alias to [`unwrap`](#tymethod.unwrap)
    fn get(self) -> <Self as PipeUnwrap>::Value {
        self.unwrap()
    }
}
