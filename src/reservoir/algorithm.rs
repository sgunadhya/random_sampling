use std::marker::PhantomData;
use rand::Rng;
use crate::sampler::Sampler;

pub struct ReservoirSampler<T> {
    rng: rand::rngs::ThreadRng,
    _phantom: PhantomData<T>,
}

impl<T> ReservoirSampler<T> {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Sampler<T> for ReservoirSampler<T> {
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

        let mut reservoir: Vec<T> = data[..k].to_vec();

        for i in k..n {
            let j = self.rng.gen_range(0..=i);  // Use self.rng instead of creating new
            if j < k {
                reservoir[j] = data[i].clone();
            }
        }

        reservoir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reservoir_sampling() {
        let data: Vec<i32> = (1..=100).collect();
        let mut sampler = ReservoirSampler::new();  // Add mut here
        let sample = sampler.sample(&data, 10);
        assert_eq!(sample.len(), 10);
    }

    #[test]
    fn test_empty_input() {
        let data: Vec<i32> = vec![];
        let mut sampler = ReservoirSampler::new();  // Add mut here
        let sample = sampler.sample(&data, 5);
        assert!(sample.is_empty());
    }
}