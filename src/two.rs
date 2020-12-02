use std::fs;

pub fn get_occurrences(sequence: Vec<String>, requirement: String) -> usize {
  sequence.iter().filter(|&a| a.to_string() == requirement).count()
}

pub fn get_char_at_index(sequence: Vec<String>, index: usize) -> String {
  return sequence.into_iter().nth(index - 1).unwrap();
}

pub fn password_policy() {
  let passwords = fs::read_to_string("./input/two.txt")
  .expect("Something went wrong reading the file");

let passwords: Vec<(bool, bool)> = passwords
  .lines()
  .map(|password| {
    let mut strings = password.split_whitespace();
    let rules = strings.next().unwrap().to_string();

    let mut rules_iter = rules.split("-").map(|s| s.to_string());
    let min = rules_iter.next().unwrap();
    let max = rules_iter.next().unwrap();

    let requirement = strings.next().unwrap().to_string().replace(":", "");
    let mut password: Vec<String> = strings.next().unwrap().to_string().split("").map(|s| s.to_string()).collect::<Vec<String>>();
    password.remove(0);
    password.remove(password.len()-1);

    // Part 1
    let occurrences = get_occurrences(password.clone(), requirement.clone());
    let has_min = occurrences >= min.parse().unwrap();
    let has_max = occurrences <= max.parse().unwrap();

    // Part 2
    let ok = (get_char_at_index(password.clone(), min.parse().unwrap()) == requirement) ^ (get_char_at_index(password.clone(), max.parse().unwrap()) == requirement);

    (has_min && has_max, ok)
  })
  .collect();

  println!("Day 2, Task 1, Method 1: {:?}", passwords.iter().filter(|p| p.0 == true).count());
  println!("Day 2, Task 2, Method 1: {:?}", passwords.iter().filter(|p| p.1 == true).count());
}