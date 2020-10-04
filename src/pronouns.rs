use std::fmt;

#[derive(Debug, Clone)]
pub struct Pronouns {
  pub id: u32,
  pub subject: String,
  pub object: String,
  pub possessive_determiner: String,
  pub possessive: String,
}

impl PartialEq for Pronouns {
  fn eq(&self, other: &Self) -> bool {
    self.subject == other.subject
      && self.object == other.object
      && self.possessive_determiner == other.possessive_determiner
      && self.possessive == other.possessive
  }
}

impl Pronouns {
  pub fn new(
    id: u32,
    subject: String,
    object: String,
    possessive_determiner: String,
    possessive: String,
    ) -> Pronouns {
    Pronouns {
      id,
      subject,
      object,
      possessive_determiner,
      possessive,
    }
  }
  pub fn short_string(&self) -> String {
    String::from(format!("({}/{}/{}/{})", self.subject, self.object, self.possessive_determiner, self.possessive))
  }
}

impl fmt::Display for Pronouns {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} | {} | {} | {} | {}\n",
      &self.id,
      &self.subject,
      &self.object,
      &self.possessive_determiner,
      &self.possessive,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_pronouns() {
    let p1 = Pronouns::new(1, String::from("he"), String::from("him"), String::from("his"), String::from("his"));
    let p2 = Pronouns::new(2, String::from("she"), String::from("her"), String::from("her"), String::from("hers"));
    let p3 = Pronouns::new(3, String::from("they"), String::from("them"), String::from("their"), String::from("theirs"));
    assert_eq!(p1.id, 1);
    assert_eq!(p1.subject, String::from("he"));
    assert_eq!(p1.object, String::from("him"));
    assert_eq!(p1.possessive_determiner, String::from("his"));
    assert_eq!(p1.possessive, String::from("his"));
    assert_eq!(p2.id, 2);
    assert_eq!(p2.subject, String::from("she"));
    assert_eq!(p2.object, String::from("her"));
    assert_eq!(p2.possessive_determiner, String::from("her"));
    assert_eq!(p2.possessive, String::from("hers"));
    assert_eq!(p3.id, 3);
    assert_eq!(p3.subject, String::from("they"));
    assert_eq!(p3.object, String::from("them"));
    assert_eq!(p3.possessive_determiner, String::from("their"));
    assert_eq!(p3.possessive, String::from("theirs"));
  }
}