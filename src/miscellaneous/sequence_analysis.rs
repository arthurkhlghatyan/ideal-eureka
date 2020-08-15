/**
 * Given a list of integers, use a vector and return
 * The mean (the average value),
 * median (when sorted, the value in the middle position),
 * and mode (the value that occurs most often).
 */
use std::collections::HashMap;
use std::option::Option;

pub fn sequence_analysis(sorted_sequence: &Vec<i64>) {
  println!("The mean of sequence is {}", calculate_mean(sorted_sequence));
  println!("The median of sequence is {}", calculate_median(sorted_sequence));
  println!("The mode of sequence is {:?}", calculate_mode(sorted_sequence));
}
 
fn calculate_mean(sorted_sequence: &Vec<i64>) -> i64 {
  let mut mean: i64 = 0;
  
  for i in sorted_sequence {
    mean += i;
  }

  mean / sorted_sequence.len() as i64
}

fn calculate_median(sorted_sequence: &Vec<i64>) -> i64 {
  let len = sorted_sequence.len();
  let mid = len / 2;

  match len % 2 {
    1 => sorted_sequence[mid],
    0 => sorted_sequence[mid] + sorted_sequence[mid - 1],
    _ => 0
  }
}

fn calculate_mode(sorted_sequence: &Vec<i64>) -> Option<i64> {
  let mut occurrence_map: HashMap<&i64, i64> = HashMap::new();
  let mut mode: i64 = 0;
  let mut max_occurences: i64 = 0;

  for i in sorted_sequence {
    let count = occurrence_map.entry(i).or_insert(0);
    *count += 1;
  }

  // Iterate through map and find mode
  for (&key, &value) in &occurrence_map {
    if value > max_occurences {
      mode = *key;
      max_occurences = value;
    }
  }

  if max_occurences == 1 {
    return Option::None
  }

  Option::Some(mode)
}