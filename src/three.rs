use std::fs;

pub fn walk_items(grid: Vec<Vec<String>>, row: usize, column: usize, trees: usize, right: usize, down: usize) -> usize {
  let mut tree_count = trees;
  let mut next_column = column + right;
  let next_row = row + down;

  let column_will_overflow = next_column > grid[row].len();
  if column_will_overflow { next_column = next_column - grid[row].len() } 

  let at_end_of_column = next_column == grid[row].len();
  if at_end_of_column { next_column = 0 }

  let is_tree = grid[row][column] == "#";
  if is_tree { tree_count = tree_count + 1 }

  let no_more_rows = next_row > grid.len() - 1;
  if no_more_rows { return tree_count } 
   
  walk_items(grid, next_row, next_column, tree_count, right, down)
}

pub fn compute_tree_encounter() {
  let contents = fs::read_to_string("./input/three.txt")
    .expect("Something went wrong reading the file");

  let grid: Vec<Vec<String>> = contents
    .lines()
    .map(|position| {
      let mut positions = position.split("").map(|s| s.to_string()).collect::<Vec<String>>();
      positions.remove(0);
      positions.remove(positions.len()-1);
      return positions;
    }).collect();

    let a = walk_items(grid.clone(), 1, 1, 0, 1, 1);
    let b = walk_items(grid.clone(), 1, 3, 0, 3, 1);
    let c = walk_items(grid.clone(), 1, 5, 0, 5, 1);
    let d = walk_items(grid.clone(), 1, 7, 0, 7, 1);
    let e = walk_items(grid.clone(), 2, 1, 0, 1, 2);
    
    println!("Day 3, Task 1: {}", b);
    println!("Day 3, Task 2: {}", a*b*c*d*e);
}