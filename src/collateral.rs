use std::fmt;

#[derive(Debug, Clone)]
pub struct Collateral {
  pub id: u32,
  pub first_name: String,
  pub last_name: String,
  pub title: String,
  pub institution: Option<String>,
  pub pronouns: u32,
  pub support_type: SupportType,
  pub indirect_support: bool,
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

#[derive(Debug, Clone, PartialOrd, Eq, Ord)]
pub enum SupportType {
  Natural,
  Formal,
}

use crate::collateral::SupportType::{Natural, Formal};

impl PartialEq for SupportType {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&Natural, &Natural) => true,
      (&Formal, &Formal) => true,
      _ => false,
    }
  }
}

impl Collateral {
  pub fn new(
    id: u32,
    first_name: String,
    last_name: String,
    title: String,
    institution: Option<String>,
    pronouns: u32,
    support_type: SupportType,
    indirect_support: bool,
    ) -> Collateral {
    Collateral {
      id,
      first_name,
      last_name,
      title,
      institution,
      pronouns,
      support_type,
      indirect_support,
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
    format!(
      "{} ({})",
      &self.full_name(),
      &self.title(),
    )
  }
  pub fn full_name_and_title(&self) -> String {
    match self.support_type {
      Natural => {
        if self.indirect_support {
          return format!("{} ({})", &self.full_name(), &self.title);
        } else {
          return format!("{} ({} of youth)", &self.full_name(), &self.title);
        }
      },
      Formal => {
        match &self.institution {
          Some(i) => {
            if self.indirect_support {
              return format!("{} ({} at {})", &self.full_name(), &self.title, i);
            } else {
              return format!("{} ({} for youth at {})", &self.full_name(), &self.title, i);
            }
          },
          None => {
            if self.indirect_support {
              return format!("{} ({})", &self.full_name(), &self.title);
            } else {
              return format!("{} ({} for youth)", &self.full_name(), &self.title);
            }
          }
        }
      },
    }
  }
  pub fn title(&self) -> String {
    match self.support_type {
      Natural => {
        return format!("{}", &self.title);
      },
      Formal => {
        match &self.institution {
          Some(i) => {
            return format!("{} at {}", &self.title, i);
          },
          None => {
            return format!("{}", &self.title);
          }
        }
      },
    }
  }
}

impl fmt::Display for Collateral {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let inst_str = match &self.institution {
      Some(i) => &i[..],
      None => "--NONE--",
    };
    let sup_type = match self.support_type {
      Natural => "Natural",
      Formal => "Formal",
    };
    let indirect = match self.indirect_support {
      true => "true",
      false => "false",
    };
    let first_name = self.first_name.replace(" | ", " / ");
    let last_name = self.last_name.replace(" | ", " / ");
    write!(
      f,
      "{} | {} | {} | {} | {} | {} | {} | {}\n",
      &self.id,
      &first_name[..],
      &last_name[..],
      &self.title[..],
      &inst_str,
      &self.pronouns,
      sup_type,
      indirect,
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
      Some(String::from("Riverside Community Care")),
      2,
      Natural,
      false,
    );
    assert_eq!(c1.id, 1);
    assert_eq!(c1.first_name, String::from("Bob"));
    assert_eq!(c1.last_name, String::from("Smith"));
    assert_eq!(c1.title, String::from("OPT"));
    assert_eq!(c1.institution, Some(String::from("Riverside Community Care")));
    assert_eq!(c1.pronouns, 2);
  }
}