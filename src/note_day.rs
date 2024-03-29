use std::fmt;
use chrono::{NaiveDate, Datelike, Weekday};
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct NoteDay {
  pub id: u32,
  pub foreign_key: HashMap<String, u32>,
  pub foreign_keys: HashMap<String, Vec<u32>>,
  pub date: NaiveDate,
}

impl PartialEq for NoteDay {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl NoteDay {
  pub fn new(
    id: u32,
    date: NaiveDate,
    user_id: u32,
    client_id: u32,
    notes: Vec<u32>,
    ) -> NoteDay {
    let foreign_key: HashMap<String, u32> = [
      (String::from("user_id"), user_id),
      (String::from("client_id"), client_id),
    ].iter().cloned().collect();
    let foreign_keys: HashMap<String, Vec<u32>> = [
      (String::from("note_ids"), notes),
    ].iter().cloned().collect();
    NoteDay {
      id,
      date,
      foreign_key,
      foreign_keys,
    }
  }
  pub fn fmt_date(&self) -> String {
    self.date.format("%Y-%m-%d").to_string()
  }
  pub fn heading_date(&self) -> String {
    let wd = match self.date.weekday() {
      Weekday::Mon => "Monday",
      Weekday::Tue => "Tuesday",
      Weekday::Wed => "Wednesday",
      Weekday::Thu => "Thursday",
      Weekday::Fri => "Friday",
      Weekday::Sat => "Saturday",
      Weekday::Sun => "Sunday",
    };
    format!("{} {}/{}", wd, self.date.month(), self.date.day())
  }
  pub fn fmt_date_long(&self) -> String {
    let wd = match self.date.weekday() {
      Weekday::Mon => "Monday",
      Weekday::Tue => "Tuesday",
      Weekday::Wed => "Wednesday",
      Weekday::Thu => "Thursday",
      Weekday::Fri => "Friday",
      Weekday::Sat => "Saturday",
      Weekday::Sun => "Sunday",
    };
    let month = match &self.date.month() {
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
    let suffix = match &self.date.day() {
      1 | 21 | 31 => "st",
      2 | 22 => "nd",
      3 | 23 => "rd",
      _ => "th"
    };
    let date: String = format!("{}, {} {}{}, {}", wd, month, &self.date.day(), suffix, &self.date.year());
    date
  }
  pub fn fmt_date_short(&self) -> String {
    self.date.format("%m-%d-%y").to_string()
  }
  pub fn fmt_day(&self) -> String {
    let wd = match self.date.weekday() {
      Weekday::Mon => "Monday",
      Weekday::Tue => "Tuesday",
      Weekday::Wed => "Wednesday",
      Weekday::Thu => "Thursday",
      Weekday::Fri => "Friday",
      Weekday::Sat => "Saturday",
      Weekday::Sun => "Sunday",
    };
    format!("{}", wd)
  }
}

impl fmt::Display for NoteDay {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} | {}-{}-{} | {} | {} | {}\n",
      &self.id,
      &self.date.year(),
      &self.date.month(),
      &self.date.day(),
      &self.foreign_key["user_id"],
      &self.foreign_key["client_id"],
      &self
        .foreign_keys["note_ids"]
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("#"),
    )
  }
}