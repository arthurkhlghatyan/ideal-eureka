pub fn adjacent_elements_product(input_array: Vec<i32>) -> i32 {
  let mut m_prod = input_array[0] * input_array[1];
  let vec_len = input_array.len();

  for i in 2..vec_len {
    let prod = input_array[i - 1] * input_array[i];

    if prod > m_prod {
      m_prod = prod
    }
  }
  
  m_prod
}