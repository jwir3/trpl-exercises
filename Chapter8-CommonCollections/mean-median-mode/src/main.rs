use std::io;
use std::collections::HashMap;

fn main() {
    // This hash map will hash a value to the number of times it was encountered.
    let mut hash_map = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Enter an unsigned integer or a character when done:");

        io::stdin()
        .read_line(&mut input)
        // .read_line returns an io::Result, which is an enum
        // The possible values are Ok or Err. This is to encode failure conditions more succinctly.
        // If the value is Err, then expect() will crash and report the message to the stdout.
        .expect("Failed to read line");

        // We need to convert guess to a number in order to perform the comparison later on. Note that
        // the name 'guess' is _shadowed_, allowing us to basically replace it without coming up with
        // another variable name.
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let entry = hash_map.entry(input).or_insert(0);
        *entry += 1;
    }

    // println!("{:?}", hash_map);
    println!("Mean: {}", get_mean(&hash_map));
    println!("Median: {}", get_median(&hash_map));
    println!("Mode: {:?}", get_mode(&hash_map));
}

fn get_mean(hash_map: &HashMap<u32, u32>) -> f64 {
    let mut total: f64 = 0.0;
    let mut divisor: f64 = 1.0; // Avoid divide by zero
    for (key, value) in hash_map {
        total = total + (*key as f64 * *value as f64);
        divisor = divisor + *value as f64;
    }

    if divisor > 1.0 {
        divisor -= 1.0;
    }

    total / divisor
}

fn get_median(hash_map: &HashMap<u32, u32>) -> f64 {
    // First, convert hash map to a (sorted) vector
    let vector = get_hash_map_as_vector(hash_map);

    // Now, find the middle value
    let middle_index: f64 = (vector.len() as f64) / 2.0;
    let result: f64;
    if vector.len() % 2 == 1 {
        let middle_index: usize = middle_index as usize;
        result = vector[middle_index] as f64;
    } else {
        let lower_index = (middle_index - 1.0) as usize;
        let upper_index = middle_index as usize;
        result = (vector[lower_index] as f64 + vector[upper_index] as f64) / 2.0;
    }

    result
}

fn get_mode(hash_map: &HashMap<u32, u32>) -> Vec<u32> {
    let mut result_vec: Vec<u32> = Vec::new();
    let mut max_value = 0;
    for (key, value) in hash_map {
        if *value > max_value {
            result_vec.clear();
            max_value = *value;
            result_vec.push(*key);
        } else if *value == max_value {
            result_vec.push(*key);
        }
    }

    result_vec.sort_unstable();
    result_vec
}

fn get_hash_map_as_vector(hash_map: &HashMap<u32, u32>) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();

    for (key, value) in hash_map {
        let mut num_entries = *value;
        while num_entries > 0 {
            vec.push(*key);
            num_entries -= 1;
        }
    }

    vec.sort_unstable();
    vec
}
