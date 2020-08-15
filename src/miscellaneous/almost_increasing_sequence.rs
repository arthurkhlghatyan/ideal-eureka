// 1 3 2 1
// 1 2 1 2
// 3, 6, 5, 8, 10, 20, 15
// 1, 1, 2, 3, 4, 4
// 1, 3, 2

pub fn almost_increasing_sequence(sequence: Vec<i32>) -> bool {
  let len = sequence.len();
  let mut rm_c = 0;
  
  for i in 0..len - 1 {
    let mut lt_c = 0;

    for k in i+1..len {
      if sequence[k] <= sequence[i] {
        lt_c += 1;
      }
    }

    if lt_c > 1 {
      return false;
    }

    if lt_c > 0 {
      rm_c += 1;
    }
    
  }

  rm_c == 0
}
