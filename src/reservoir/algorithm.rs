
// src/reservoir/algorithm.rs
use rand::Rng;
use crate::sampler::Sampler;

pub struct ReservoirSampler<T> {
    rng: rand::rngs::ThreadRng,
}

impl<T> ReservoirSampler<T> {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl<T> Sampler<T> for ReservoirSampler<T> {
    fn sample(&self, data: &[T], k: usize) -> Vec<T>
    where
        T: Clone,
    {
        let mut rng = rand::thread_rng();
        let n = data.len();

        if k == 0 || n == 0 {
            return Vec::new();
        }
        if k >= n {
            return data.to_vec();
        }

        let mut reservoir: Vec<T> = data[..k].to_vec();

        for i in k..n {
            let j = rng.gen_range(0..=i);
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
        let sampler = ReservoirSampler::new();
        let sample = sampler.sample(&data, 10);
        assert_eq!(sample.len(), 10);
    }

    #[test]
    fn test_empty_input() {
        let data: Vec<i32> = vec![];
        let sampler = ReservoirSampler::new();
        let sample = sampler.sample(&data, 5);
        assert!(sample.is_empty());
    }
}
