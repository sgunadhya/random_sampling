# Random Sampling Utilities in Rust

A Rust library providing data sampling utilities. Includes support for sampling strategies like reservoir sampling, sorted sampling, and a general-purpose sampler.

## Features

- **General-Purpose Sampler**: Flexible interface for sampling.
- **Reservoir Sampling**: Randomly selects a fixed-size subset from a larger population.
- **Sorted Sampling**: Samples data while preserving sort order.

## Usage

Add the library to your `Cargo.toml` dependencies:

```toml
[dependencies]
random_sampling = "0.1.0"
```

Then, import and use as needed:

```rust
use random_sampling::{Sampler, ReservoirSampler, SortedSampler};

fn main() {
    // Example usage of ReservoirSampler
    let reservoir = ReservoirSampler::new(3);
    let samples = reservoir.sample(vec![1, 2, 3, 4, 5]);
    println!("Reservoir Samples: {:?}", samples);
}
```

## Installation

You can include this library in your Rust project by adding it as a dependency in your `Cargo.toml`.

## Modules

- **sampler**: Core abstractions for sampling.
- **reservoir**: Efficient reservoir sampling implementation.
- **sorted**: Sampling while maintaining sorted order.

## Contributions

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the [MIT License](LICENSE).