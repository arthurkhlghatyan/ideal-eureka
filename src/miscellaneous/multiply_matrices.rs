pub fn multiply_matrices(m1: Vec<Vec<i64>>, m2: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
  // Resulting vec size is m1.len() X m2[0].len()
  let mut result: Vec<Vec<i64>> = Vec::new();
  let result_rows = m1.len();
  let result_cols = m2[0].len();


  // Check if valid matrices are given
  if m1[0].len() != m2.len() {
    panic!("Invalid matrices are given");
  }

  for i in 0..result_rows {
    let mut row: Vec<i64> = Vec::new();
    let m1_row_len = m1[i].len();

    for j in 0..result_cols {
      let mut sum = 0;

      for k in 0..m1_row_len {
        sum += m1[i][k] * m2[k][j];
      }

      row.push(sum);
    }

    result.push(row);
  }

  result
}