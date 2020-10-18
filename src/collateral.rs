use std::fmt;

#[derive(Debug, Clone)]
pub struct Collateral {
  pub id: u32,
  pub first_name: String,
  pub last_name: String,
  pub title: String,
  pub institution: String,
  pub pronouns: u32,
}

impl PartialEq for Collateral {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
      && self.first_name == other.first_name
      && self.last_name == other.last_name
      && self.title == other.title
      && self.institution == other.institution
  }
}

impl Collateral {
  pub fn new(
    id: u32,
    first_name: String,
    last_name: String,
    title: String,
    institution: String,
    pronouns: u32,
    ) -> Collateral {
    Collateral {
      id,
      first_name,
      last_name,
      title,
      institution,
      pronouns,
    }
  }
  pub fn full_name(&self) -> String {
    let mut name = String::new();
    name.push_str(&self.first_name);
    name.push_str(" ");
    name.push_str(&self.last_name);
    name
  }
  pub fn full_name_and_title(&self) -> String {
    format!("{} ({} at {})", &self.full_name(), &self.title, &self.institution)
  }
  pub fn full_name_and_relationship_to_youth(&self) -> String {
    format!("{} ({} for youth at {})", &self.full_name(), &self.title, &self.institution)
  }
}

impl fmt::Display for Collateral {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} | {} | {} | {} | {} | {}\n",
      &self.id,
      &self.first_name[..],
      &self.last_name[..],
      &self.title[..],
      &self.institution[..],
      &self.pronouns,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_collaterals() {
    let c1 = Collateral::new(
      1,
      String::from("Bob"),
      String::from("Smith"),
      String::from("OPT"),
      String::from("Riverside Community Care"),
      2
    );
    assert_eq!(c1.id, 1);
    assert_eq!(c1.first_name, String::from("Bob"));
    assert_eq!(c1.last_name, String::from("Smith"));
    assert_eq!(c1.title, String::from("OPT"));
    assert_eq!(c1.institution, String::from("Riverside Community Care"));
    assert_eq!(c1.pronouns, 2);
  }
}