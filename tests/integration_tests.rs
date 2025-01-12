use random_sampling::{ReservoirSampler, SortedSampler, Sampler};

#[test]
fn test_both_samplers() {
    let data = vec![1, 2, 3, 4, 5];
    let reservoir = ReservoirSampler::new();
    let sorted = SortedSampler::new();

    assert_eq!(reservoir.sample(&data, 2).len(), 2);
    assert_eq!(sorted.sample(&data, 2).len(), 2);
}