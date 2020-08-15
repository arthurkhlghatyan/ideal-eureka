pub fn shape_area(n: i32) -> i32 {
  let mut area = 1;

  for i in 1..n {
    area = area + i * 4
  }

  area
}