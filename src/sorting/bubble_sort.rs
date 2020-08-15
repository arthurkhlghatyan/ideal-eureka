pub fn bubble_sort(sequence: &mut Vec<i64>) {
  let len = sequence.len();
  let mut swapped;

  loop {
    swapped = false;

    for i in 0..len - 1 {
      if sequence[i] > sequence[i + 1] {
        let tmp = sequence[i];
        sequence[i] = sequence[i + 1];
        sequence[i + 1] = tmp;
        swapped = true;
      }
    }

    if !swapped {
      break;
    } 
  }
}