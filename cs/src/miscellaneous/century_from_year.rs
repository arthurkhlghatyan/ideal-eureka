pub fn century_from_year(year: i32) -> i32 {
  if year % 100 == 0 {
    return year / 100
  }
    
  year / 100 + 1
}
