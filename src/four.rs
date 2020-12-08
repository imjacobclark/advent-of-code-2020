use std::fs;
use regex::Regex;

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

pub fn validate_boundary(date: i32, lower: i32, upper: i32) -> bool {
  date >= lower && date <= upper
}

pub fn validate_passport(passport: Vec<(String, String)>) -> bool {
  let required_fields: Vec<String> = vec!["byr".to_string(), "iyr".to_string(), "eyr".to_string(), "hgt".to_string(), "hcl".to_string(), "ecl".to_string(), "pid".to_string(), "cid".to_string()];

  let pmf = required_fields
    .clone()
    .into_iter()
    .filter(|rfield| !passport.clone().into_iter().any(|passport| passport.0 == *rfield))
    .collect::<Vec<String>>();

  let required_fields = pmf
    .clone()
    .into_iter()
    .any(|f| (pmf.len() == 1 && f == "cid")) || pmf.len() == 0;

  let required_values: Vec<bool> = passport
    .clone()
    .into_iter()
    .map(|field| {
      if field.0 == "byr" { return validate_boundary(field.1.parse::<i32>().unwrap(), 1920, 2002) && field.1.len() == 4; } 
      if field.0 == "iyr" {  return validate_boundary(field.1.parse::<i32>().unwrap(), 2010, 2020) && field.1.len() == 4; }
      if field.0 == "eyr" {  return validate_boundary(field.1.parse::<i32>().unwrap(), 2020, 2030)  && field.1.len() == 4; }
      if field.0 == "hgt" {  
        let regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
        let text = &field.1;
        let capture = regex.captures_iter(text).next();

        if capture.is_none() { return false }

        let unwrapped_capture = capture.unwrap();
        let value = unwrapped_capture[1].parse::<i32>().unwrap();
        let measure: String = unwrapped_capture[2].to_string();

        return measure == "cm" && validate_boundary(value, 150, 193) || measure == "in" && validate_boundary(value, 59, 76);
      }

      if field.0 == "hcl" { return Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(&field.1) }
      if field.0 == "ecl" { return Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap().is_match(&field.1) }
      if field.0 == "pid" { return Regex::new(r"^[0-9]{9}$").unwrap().is_match(&field.1) }
      if field.0 == "cid" { return true }

      return false;
    })
    .filter(|&x| x == false)
    .collect();

    required_fields && required_values.len() == 0
}

pub fn passport_validation() {
  let contents = fs::read_to_string("./input/four.txt").expect("Something went wrong reading the file");

  let passports: Vec<Vec<(String, String)>> = contents
    .split("\n\n")
    .map(|passport| passport
      .split(" ")
      .flat_map(|passport_data| passport_data
        .split("\n")
        .map(marshall_passport_key_value))
      .collect())
    .collect();

    let valid_passports: usize = passports
      .into_iter()
      .map(validate_passport)
      .filter(|&x| x == true)
      .collect::<Vec<bool>>()
      .len();

    println!("Day 4, Task 1: {:?}", valid_passports);
}