use std::collections::HashMap;

pub fn first_duplicate(a: Vec<i32>) -> i32 {
  let mut occurences = HashMap::new();

  for i in a {
    match occurences.insert(i, 0) {
      Some(_k) => {
        return i
      },
      None => {}
    };
  }

  -1
}