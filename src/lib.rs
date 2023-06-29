/// Deletes the output of a function
pub trait Consume {
    fn consume(&self) {
        return;
    }
}

impl<T> Consume for T {}