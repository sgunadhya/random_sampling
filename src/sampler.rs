pub trait Sampler<T> {
    /// Sample k items from the input data
    fn sample(&self, data: &[T], k: usize) -> Vec<T>
    where
        T: Clone;
}