use std::fmt;
use chrono::{NaiveDate, Datelike, TimeZone, Utc, Local};


#[derive(Debug, Clone)]
pub struct Client {
  pub id: u32,
  pub first_name: String,
  pub last_name: String,
  pub dob: NaiveDate,
  pub collaterals: Vec<u32>,
}

impl PartialEq for Client {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
      && self.first_name == other.first_name
      && self.last_name == other.last_name
      && self.dob == other.dob
      && self.collaterals == other.collaterals
  }
}

impl Client {
  pub fn new(
    id: u32,
    first_name: String,
    last_name: String,
    dob: NaiveDate,
    collaterals: Vec<u32>,
    ) -> Client {
    Client {
      id,
      first_name,
      last_name,
      dob,
      collaterals,
    }
  }
  pub fn full_name(&self) -> String {
    let mut name = String::new();
    name.push_str(&self.first_name);
    name.push_str(" ");
    name.push_str(&self.last_name);
    name
  }
  pub fn fmt_dob(&self) -> String {
    let dob: String = format!("{}-{}-{}", &self.dob.year(), &self.dob.month(), &self.dob.day());
    dob
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
    write!(
      f,
      "{} | {} | {} | {}-{}-{} | {}\n",
      &self.id,
      &self.first_name[..],
      &self.last_name[..],
      &self.dob.year(),
      &self.dob.month(),
      &self.dob.day(),
      &self
        .collaterals
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
    let c1 = Client::new(1, String::from("Bob"), String::from("Smith"), NaiveDate::from_ymd(2000, 1, 1), vec![]);
    let c2 = Client::new(2, String::from("Joe"), String::from("Shmoe"), NaiveDate::from_ymd(2000, 1, 2), vec![]);
    let test_vec: Vec<u32> = vec![];
    assert_eq!(c1.id, 1);
    assert_eq!(c1.first_name, String::from("Bob"));
    assert_eq!(c1.last_name, String::from("Smith"));
    assert_eq!(c1.dob, NaiveDate::from_ymd(2000, 1, 1));
    assert_eq!(c1.collaterals, test_vec);
    assert_eq!(c2.id, 2);
    assert_eq!(c2.first_name, String::from("Joe"));
    assert_eq!(c2.last_name, String::from("Shmoe"));
    assert_eq!(c2.dob, NaiveDate::from_ymd(2000, 1, 2));
    assert_eq!(c2.collaterals, test_vec);
  }
}