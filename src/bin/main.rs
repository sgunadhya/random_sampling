use clap::{Arg, Command};
use random_sampling::{ReservoirSampler, Sampler};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Reads data from stdin, one number per line
fn read_from_stdin() -> io::Result<Vec<i32>> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let num = line.parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))?;
        numbers.push(num);
    }

    Ok(numbers)
}

/// Reads data from a file, one number per line
fn read_from_file(path: &str) -> io::Result<Vec<i32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let num = line.parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))?;
        numbers.push(num);
    }

    Ok(numbers)
}

/// Parses comma-separated values from a string
fn parse_csv_string(input: &str) -> io::Result<Vec<i32>> {
    let mut numbers = Vec::new();

    for item in input.split(',') {
        let num = item.trim().parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))?;
        numbers.push(num);
    }

    Ok(numbers)
}

fn main() -> io::Result<()> {
    let matches = Command::new("Random Sampling CLI")
        .version("0.1.0")
        .about("Performs random sampling using the random_sampling library")
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .value_name("NUMBER")
                .help("The size of the subset to sample")
                .required(true),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input file containing numbers (one per line)"),
        )
        .arg(
            Arg::new("data")
                .short('d')
                .long("data")
                .value_name("NUMBERS")
                .help("Comma-separated list of numbers"),
        )
        .get_matches();

    // Retrieve and parse the size value
    let size: usize = matches
        .get_one::<String>("size")
        .expect("Subset size is required")
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))?;

    // Get the data based on input method
    let data = if let Some(input_file) = matches.get_one::<String>("input") {
        // Read from file
        read_from_file(input_file)?
    } else if let Some(data_str) = matches.get_one::<String>("data") {
        // Parse comma-separated values
        parse_csv_string(data_str)?
    } else {
        // Read from stdin
        println!("Enter numbers (one per line). Press Ctrl+D (Unix) or Ctrl+Z (Windows) when done:");
        read_from_stdin()?
    };

    if data.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No data provided",
        ));
    }

    if size > data.len() {
        println!("Warning: Requested sample size ({}) is larger than data size ({})", size, data.len());
    }

    let mut reservoir = ReservoirSampler::new();
    let samples = reservoir.sample(&data, size);

    println!("Input data size: {}", data.len());
    println!("Randomly sampled subset: {:?}", samples);

    Ok(())
}