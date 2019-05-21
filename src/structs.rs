/// Return value of [`pipe`](fn.pipe.html)
pub struct PipeBox<'a, Value: 'a> {
    pub(crate) func: Box<'a + FnOnce() -> Value>,
}
