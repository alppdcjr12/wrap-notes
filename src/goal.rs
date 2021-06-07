use std::fmt;

#[derive(Debug, Clone)]
pub struct Goal {
  pub id: u32,
  pub client_id: u32,
  pub goal: String,
}

impl PartialEq for Goal {
  fn eq(&self, other: &Self) -> bool {
    self.goal == other.goal
      && self.client_id == other.client_id
  }
}

impl Goal {
  pub fn new(
    id: u32,
    client_id: u32,
    goal: String,
    ) -> Goal {
    Goal {
      id,
      client_id,
      goal,
    }
  }
}

impl fmt::Display for Goal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} | {} | {}\n",
      &self.id,
      &self.client_id,
      &self.goal,
    )
  }
}