use std::cmp;
use std::collections::HashSet;

pub fn make_array_consecutive2(statues: Vec<i32>) -> i32 {
  let len = statues.len();
  let mut range_pairs: HashSet<i32> = HashSet::new();
  let mut num_c = 0;

  for i in 1..len {
    let comp_1 = statues[i - 1];
    let comp_2 = statues[i];
    let max = cmp::max(comp_1, comp_2);
    let mut min = cmp::min(comp_1, comp_2);

    for k in 0..len {
      if max > statues[k] && min < statues[k] {
        min = statues[k];
      }
    }

    if max - min > 1 {
      // https://en.wikipedia.org/wiki/Pairing_function
      let pair = ((min + max) * (min + max + 1) / 2) + max;

      if !range_pairs.contains(&pair) {
        num_c += max - min - 1;
        range_pairs.insert(pair);
      }
    }
  }

  num_c
}