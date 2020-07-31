pub fn check_palindrome(input_string: String) -> bool {
  let str_len = input_string.len();
  let mut start = 0;
  let mut end = str_len - 1;
  let s_vec: Vec<char> = input_string.chars().collect();

  while start <= str_len / 2 {
    if s_vec[start] != s_vec[end] {
      return false
    }

    start += 1;
    end -= 1;
  }

  true
}