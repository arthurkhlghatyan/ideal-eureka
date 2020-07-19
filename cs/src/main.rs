// Declare modules
// mod miscellaneous;
// mod sorting;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    },
  };

  // let m1: Vec<Vec<i64>> = vec![
  //   vec![1, 2, 3],
  //   vec![2, 4, 1],
  //   vec![5, 6, 2]
  // ];

  // let m2: Vec<Vec<i64>> = vec![
  //   vec![2, 3, 4],
  //   vec![1, 2, 10],
  // ];

  // miscellaneous::multiply_matrices(m1, m2);
}
