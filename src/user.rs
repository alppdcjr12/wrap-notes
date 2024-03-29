use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct User {
  pub id: u32,
  pub first_name: String,
  pub last_name: String,
  pub role: EmployeeRole,
  pub pronouns: u32,
  pub foreign_keys: HashMap<String, Vec<u32>>,
}

impl PartialEq for User {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
      && self.first_name == other.first_name
      && self.last_name == other.last_name
      && self.role == other.role
  }
}

#[derive(Debug, Clone)]
pub enum EmployeeRole {
  Fp,
  Icc,
}

use crate::user::EmployeeRole::{Fp, Icc};

impl fmt::Display for EmployeeRole {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      Fp => "FP",
      Icc => "ICC",
    };
    write!(f, "{}", display)
  }
}

impl PartialEq for EmployeeRole {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&Fp, &Fp) => true,
      (&Icc, &Icc) => true,
      _ => false,
    }
  }
}

impl User {
  pub fn new(
    id: u32,
    first_name: String,
    last_name: String,
    role: EmployeeRole,
    pronouns: u32,
    clients: Vec<u32>,
    collaterals: Vec<u32>,
  ) -> User {
    let foreign_keys: HashMap<String, Vec<u32>> = [
      (String::from("client_ids"), clients),
      (String::from("collateral_ids"), collaterals),
    ].iter().cloned().collect();
    User {
      id,
      first_name,
      last_name,
      role,
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
  pub fn name_and_title(&self) -> String {
    let mut name = String::new();
    name.push_str(&self.first_name);
    name.push_str(" ");
    name.push_str(&self.last_name);
    name.push_str(", ");
    name.push_str(&self.role.to_string());
    name
  }
}

impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} | {} | {} | {} | {} | {} | {}\n",
      &self.id,
      &self.first_name[..],
      &self.last_name[..],
      &self.role,
      &self.pronouns,
      &self
        .foreign_keys["client_ids"]
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("#"),
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
  fn new_users() {
    let u1 = User::new(1, String::from("Carol"), String::from("Carolson"), Icc, 1, vec![], vec![]);
    let u2 = User::new(2, String::from("Kerri"), String::from("Kerrison"), Fp, 2, vec![], vec![]);
    let test_vec: Vec<u32> = vec![];
    assert_eq!(u1.id, 1);
    assert_eq!(u1.first_name, String::from("Carol"));
    assert_eq!(u1.last_name, String::from("Carolson"));
    assert_eq!(u1.foreign_keys["client_ids"], test_vec);
    assert_eq!(u1.foreign_keys["collateral_ids"], test_vec);
    assert_eq!(u2.id, 2);
    assert_eq!(u2.first_name, String::from("Kerri"));
    assert_eq!(u2.last_name, String::from("Kerrison"));
    assert_eq!(u2.foreign_keys["client_ids"], test_vec);
    assert_eq!(u2.foreign_keys["collateral_ids"], test_vec);
  }
}