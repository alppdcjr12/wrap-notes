use std::fmt;
use chrono::{NaiveDate, Datelike};
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Client {
  pub id: u32,
  pub first_name: String,
  pub last_name: String,
  pub dob: NaiveDate,
  pub pronouns: u32,
  pub foreign_keys: HashMap<String, Vec<u32>>,
}

impl PartialEq for Client {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
      && self.first_name == other.first_name
      && self.last_name == other.last_name
      && self.dob == other.dob
  }
}

impl Client {
  pub fn new(
    id: u32,
    first_name: String,
    last_name: String,
    dob: NaiveDate,
    pronouns: u32,
    collaterals: Vec<u32>,
    ) -> Client {
    let foreign_keys: HashMap<String, Vec<u32>> = [
      (String::from("collateral_ids"), collaterals),
    ].iter().cloned().collect();
    Client {
      id,
      first_name,
      last_name,
      dob,
      pronouns,
      foreign_keys,
    }
  }
  pub fn full_name(&self) -> String {
    let mut name = String::new();
    name.push_str(&self.first_name);
    name.push_str(" ");
    name.push_str(&self.last_name);
    name
  }
  pub fn full_name_with_label(&self) -> String {
    format!("{} (youth)", self.full_name())
  }
  pub fn fmt_dob(&self) -> String {
    self.dob.format("%Y-%m-%d").to_string()
  }
  pub fn fmt_date_of_birth(&self) -> String {
    let month = match &self.dob.month() {
      1 => "January",
      2 => "February",
      3 => "March",
      4 => "April",
      5 => "May",
      6 => "June",
      7 => "July",
      8 => "August",
      9 => "September",
      10 => "October",
      11 => "November",
      12 => "December",
      _ => "UNKNOWN MONTH",
    };
    let suffix = match &self.dob.day() {
      1 | 21 | 31 => "st",
      2 | 22 => "nd",
      3 | 23 => "rd",
      _ => "th"
    };
    let dob: String = format!("{} {}{}, {}", month, &self.dob.day(), suffix, &self.dob.year());
    dob
  }
}

impl fmt::Display for Client {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let first_name = self.first_name.replace(" | ", " / ");
    let last_name = self.last_name.replace(" | ", " / ");
    write!(
      f,
      "{} | {} | {} | {}-{}-{} | {} | {}\n",
      &self.id,
      &first_name[..],
      &last_name[..],
      &self.dob.year(),
      &self.dob.month(),
      &self.dob.day(),
      &self.pronouns,
      &self
        .foreign_keys["collateral_ids"]
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("#"),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_clients() {
    let c1 = Client::new(1, String::from("Bob"), String::from("Smith"), NaiveDate::from_ymd(2000, 1, 1), 2, vec![]);
    let c2 = Client::new(2, String::from("Joe"), String::from("Shmoe"), NaiveDate::from_ymd(2000, 1, 2), 1, vec![]);
    let test_vec: Vec<u32> = vec![];
    assert_eq!(c1.id, 1);
    assert_eq!(c1.first_name, String::from("Bob"));
    assert_eq!(c1.last_name, String::from("Smith"));
    assert_eq!(c1.dob, NaiveDate::from_ymd(2000, 1, 1));
    assert_eq!(c1.pronouns, 2);
    assert_eq!(c1.foreign_keys["collateral_ids"], test_vec);
    assert_eq!(c2.id, 2);
    assert_eq!(c2.first_name, String::from("Joe"));
    assert_eq!(c2.last_name, String::from("Shmoe"));
    assert_eq!(c2.dob, NaiveDate::from_ymd(2000, 1, 2));
    assert_eq!(c2.pronouns, 1);
    assert_eq!(c2.foreign_keys["collateral_ids"], test_vec);
  }
}