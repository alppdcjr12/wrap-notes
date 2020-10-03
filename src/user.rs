use std::fmt;

#[derive(Debug)]
pub struct User {
  pub id: u32,
  pub name: String,
  pub role: EmployeeRole,
  pub clients: Vec<u32>,
}

impl PartialEq for User {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
      && self.name == other.name
      && self.clients == other.clients
      && self.role == other.role
  }
}

#[derive(Debug)]
pub enum EmployeeRole {
  FP,
  ICC,
}

use crate::EmployeeRole::{FP, ICC};

impl fmt::Display for EmployeeRole {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      FP => "FP",
      ICC => "ICC",
    };
    write!(f, "{}", display)
  }
}

impl PartialEq for EmployeeRole {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&FP, &FP) => true,
      (&ICC, &ICC) => true,
      _ => false,
    }
  }
}

impl User {
  pub fn new(
    id: u32,
    name: String,
    role: EmployeeRole,
    clients: Vec<u32>) -> User {
    User {
      id,
      name,
      role,
      clients,
    }
  }
}

impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} | {} | {} | {}\n",
      &self.id,
      &self.name[..],
      &self.role,
      &self
        .clients
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
    let u1 = User::new(1, String::from("Carol"), ICC, vec![]);
    let u2 = User::new(2, String::from("Kerri"), FP, vec![]);
    let test_vec: Vec<u32> = vec![];
    assert_eq!(u1.id, 1);
    assert_eq!(u1.name, String::from("Carol"));
    assert_eq!(u1.clients, test_vec);
    assert_eq!(u2.id, 2);
    assert_eq!(u2.name, String::from("Kerri"));
    assert_eq!(u2.clients, test_vec);
  }
}