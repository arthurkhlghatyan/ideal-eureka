// Declare modules
// mod miscellaneous;
// mod sorting;
mod arrays;

fn main() {
  let v = vec![2, 1, 3, 5, 3, 2, 3, 3, 2];

  let f_dup = arrays::first_duplicate::first_duplicate(v);

  println!("{} is occurence", f_dup);
}
