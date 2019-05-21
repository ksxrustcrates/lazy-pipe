use super::impls::PipeVal;
use super::structs::PipeBox;

/// Starts a pipeline
///
/// # Examples
///
/// ```rust
/// use lazy_pipe::*;
/// # let expected = ((3 + 1) / 2).to_string();
///
/// let received = pipe(3)
///     .map(|x| x + 1)
///     .map(|x| x / 2)
///     .map(|x| x.to_string())
///     .unwrap();
///
/// # assert_eq!(received, expected);
/// ```
pub fn pipe<'a, Value: 'a>(value: Value) -> PipeBox<'a, Value> {
    PipeBox::from_val(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn pipe_works() {
        let expected = ((3 + 1) / 2).to_string();

        let received = pipe(3)
            .map(|x| x + 1)
            .map(|x| x / 2)
            .map(|x| x.to_string())
            .unwrap();

        assert_eq!(received, expected);
    }
}
