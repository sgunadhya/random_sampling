// src/sorted/algorithm.rs
use std::marker::PhantomData;
use rand::Rng;
use crate::sampler::Sampler;

pub struct SortedSampler<T> {
    rng: rand::rngs::ThreadRng,
    _phantom: PhantomData<T>,
}

impl<T> SortedSampler<T> {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Sampler<T> for SortedSampler<T>
where
    T: Clone + Ord,
{
    fn sample(&mut self, data: &[T], k: usize) -> Vec<T>  // Note: changed to &mut self
    where
        T: Clone,
    {
        let n = data.len();

        if k == 0 || n == 0 {
            return Vec::new();
        }
        if k >= n {
            return data.to_vec();
        }

        let mut result = Vec::with_capacity(k);
        let mut current_index = 0;
        let mut remaining = n;

        while result.len() < k && current_index < n {
            let prob = (k - result.len()) as f64 / remaining as f64;

            if self.rng.gen::<f64>() < prob {  // Use self.rng instead of creating new
                result.push(data[current_index].clone());
            }

            current_index += 1;
            remaining -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_sampling() {
        let mut data: Vec<i32> = (1..=100).collect();
        data.sort();
        let mut sampler = SortedSampler::new();
        let sample = sampler.sample(&data, 10);
        assert_eq!(sample.len(), 10);
    }

    #[test]
    fn test_maintains_order() {
        let data = vec![1, 2, 3, 4, 5];
        let mut sampler = SortedSampler::new();
        let sample = sampler.sample(&data, 3);
        assert!(sample.as_slice().is_sorted());  // Using built-in is_sorted()
    }
}