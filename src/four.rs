use std::fs;

pub fn str_to_string(s: &str) -> String {
  s.to_string()
}

pub fn marshall_passport_key_value(string: &str) -> (String, String) {
  let values: Vec<String> = string
    .to_string()
    .split(":")
    .map(str_to_string)
    .collect();

  (values[0].to_string(), values[1].to_string())
}

pub fn find_passports_missing_fields(passport: Vec<(String, String)>) -> Vec<String> {
  let required_fields: Vec<String> = vec!["byr".to_string(), "iyr".to_string(), "eyr".to_string(), "hgt".to_string(), "hcl".to_string(), "ecl".to_string(), "pid".to_string(), "cid".to_string()];

  required_fields
    .clone()
    .into_iter()
    .filter(|rfield| !passport.clone().into_iter().any(|passport| passport.0 == *rfield))
    .collect::<Vec<String>>()
}

pub fn validate_passport(pmf: Vec<String>) -> bool {
  pmf
    .clone()
    .into_iter()
    .any(|f| (pmf.len() == 1 && f == "cid")) || pmf.len() == 0
}

pub fn passport_validation() {
  let contents = fs::read_to_string("./input/four.txt").expect("Something went wrong reading the file");

  let valid_passports: usize = contents
    .split("\n\n")
    .map(|passport| passport
      .split(" ")
      .flat_map(|passport_data| passport_data
        .split("\n")
        .map(marshall_passport_key_value))
      .collect::<Vec<(String, String)>>())
    .map(find_passports_missing_fields)
    .map(validate_passport)
    .filter(|&x| x == true)
    .collect::<Vec<bool>>()
    .len();

    println!("Day 4, Task 1: {:?}", valid_passports);
}