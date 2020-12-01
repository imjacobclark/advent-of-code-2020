use std::env;
use std::fs;
use itertools::Itertools;

pub fn fix_expenses() {
  let contents = fs::read_to_string("./input/one.txt")
    .expect("Something went wrong reading the file");

  let numbers: Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();

  // -- Star 1 --
  
  // Method 1
  let mut result = 0;
  for x in &numbers {
    for y in &numbers {
      if x == y {
        continue;
      }

      if x + y == 2020 {
        result = x*y;
      }
    }
  }

  println!("Task 1, Method 1: {:?}", result);

  // Method 2
  let z = numbers
    .iter()
    .map(|x| {
      numbers
        .iter()
        .find(|y| if x != *y { 
          *x + *y == 2020 
        } else { 
          false 
        })
        .unwrap_or(&0)
  })
  .filter(|&x| x != &0)
  .collect::<Vec<&i32>>();

  println!("Task 1, Method 2: {:?}", z[0] * z[1]);

  // Method 3
  let result = numbers
    .iter()
    .copied()
    .combinations(2)
    .find(|x| x.iter().sum::<i32>() == 2020)
    .unwrap_or([].to_vec());
    
  println!("Task 1, Method 3: {:?}", result[0] * result[1]);

  // -- Star 2 --

  // Method 1
  let mut result = 0;
  for x in &numbers {
    for y in &numbers {
      for z in &numbers {
        if x == y || x == z || z == y {
          continue;
        }

        if x + y + z == 2020 {
          result = x*y*z;
        }
      }
    }
  }

  println!("Task 2, Method 1: {:?}", result);

  // Method 2
  let result = numbers
  .iter()
  .copied()
  .combinations(3)
  .find(|x| x.iter().sum::<i32>() == 2020)
  .unwrap_or([].to_vec());
    
  println!("Task 2, Method 2: {:?}", result[0] * result[1] * result[2]);
}