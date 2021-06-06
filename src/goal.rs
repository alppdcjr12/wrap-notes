use std::fmt;

#[derive(Debug, Clone)]
pub struct Goal {
  pub user_id: u32,
  pub goal: String,
}

impl PartialEq for Goal {
  fn eq(&self, other: &Self) -> bool {
    self.goal == other.goal
      && self.user_id == other.user_id
  }
}

impl Goal {
  pub fn new(
    user_id: u32,
    goal: String,
    ) -> Goal {
    Goal {
      user_id,
      goal,
    }
  }
  pub fn short_string(&self) -> String {
    String::from(format!("({}/{}/{}/{})", self.subject, self.object, self.possessive_determiner, self.possessive))
  }
  pub fn pub_display_pronoun(&self) {
    println!("{:-^100}", "-");
    println!("{:-^100}", self.short_string());
    println!("{:-^100}", "-");
    println!("{:-^20} | {:-^20} | {:-^20} | {:-^20} | {:-^20}", "ID", "SUBJECT", "OBJECT", "POSSESSIVE DETERMINER", "POSSESSIVE");
    println!("{: ^20} | {: ^20} | {: ^20} | {: ^20} | {: ^20}", self.id, self.subject, self.object, self.possessive_determiner, self.possessive);
    println!("{:-^100}", "-");
  }
  pub fn update_subject(&mut self, new_subject: String) {
    self.subject = new_subject;
  }
  pub fn update_object(&mut self, new_object: String) {
    self.object = new_object;
  }
  pub fn update_possessive_determiner(&mut self, new_possessive_determiner: String) {
    self.possessive_determiner = new_possessive_determiner;
  }
  pub fn update_possessive(&mut self, new_possessive: String) {
    self.possessive = new_possessive;
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