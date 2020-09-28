#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub struct NoteArchive {
  users: Vec<User>,
  current_user: Option<User>,
  clients: Vec<String>,
}

#[derive(Debug)]
pub struct User {
  id: u32,
  name: String,
  role: EmployeeRole,
  clients: Vec<u32>,
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
enum EmployeeRole {
  FP,
  ICC,
}

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

use crate::EmployeeRole::{FP, ICC};

impl User {
  fn new(id: u32, name: String, role: EmployeeRole, clients: Vec<u32>) -> User {
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

impl NoteArchive {
  fn new() -> NoteArchive {
    NoteArchive {
      users: vec![],
      clients: vec![],
      current_user: None,
    }
  }
  fn generate_new_user(
    &mut self,
    name: String,
    role: EmployeeRole,
    filepath: String,
  ) -> Result<User, String> {
    let saved_users = NoteArchive::read_users(&filepath);
    let id: u32 = saved_users.len() as u32 + 1;

    let ids_and_names: Vec<(&str, &EmployeeRole)> =
      saved_users.iter().map(|u| (&u.name[..], &u.role)).collect();

    let result = if ids_and_names.iter().any(|(n, r)| n == &name && *r == &role) {
      Err(format!(
        "There is already a {} with the name '{}'.",
        role, name
      ))
    } else {
      Ok(User::new(id, name, role, vec![]))
    };

    result
  }
  fn save_user(&mut self, user: User, filepath: &str) {
    let mut users: Vec<User> = NoteArchive::read_users(filepath);
    users.push(user);
    self.write_users(users, filepath).unwrap();
  }
  fn add_client(&mut self, client: String) {
    self.clients.push(client);
  }
  fn write_users(&mut self, users: Vec<User>, filepath: &str) -> std::io::Result<()> {
    let mut lines = String::from("##### users #####\n");
    for u in users {
      let emp_role = match u.role {
        FP => String::from("FP"),
        ICC => String::from("ICC"),
      };
      lines.push_str(&u.to_string());
    }
    lines.push_str("##### users #####");
    let mut file = File::create(filepath).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  fn read_users(filepath: &str) -> Vec<User> {
    let file = File::open(filepath);
    let file = match file {
      Ok(f) => f,
      Err(_) => File::create(filepath).unwrap(),
    };
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader.lines().map(|item| item.unwrap()).collect::<Vec<_>>();

    if lines.len() > 0 {
      lines.remove(0);
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1);
    }

    let mut users: Vec<User> = vec![];

    for line in lines {
      let values: Vec<String> = line.split(" | ").map(|val| val.to_string()).collect();

      let id: u32 = values[0].parse().unwrap();
      let name = String::from(&values[1]);
      let role = match &values[2][..] {
        "FP" => Ok(FP),
        "ICC" => Ok(ICC),
        _ => Err("Invalid role."),
      }
      .unwrap();

      let clients: Vec<u32> = match &values[3][..] {
        "" => vec![],
        _ => values[3]
          .split("#")
          .map(|val| val.parse().unwrap())
          .collect(),
      };

      let u = User::new(id, name, role, clients);
      users.push(u);
    }
    users
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
  #[test]
  fn adding_clients_to_archive() {
    let mut a1 = NoteArchive::new();
    let u1 = User::new(1, String::from("Carol"), ICC, vec![]);
    a1.add_client(String::from("Client1"));
    a1.add_client(String::from("Client2"));
    a1.add_client(String::from("Client3"));
    assert_eq!(
      a1.clients,
      [
        String::from("Client1"),
        String::from("Client2"),
        String::from("Client3"),
      ]
    );
  }
  #[test]
  fn can_write_users() {
    let u1 = User::new(1, String::from("Pete"), ICC, vec![]);
    let u2 = User::new(2, String::from("Johana"), FP, vec![]);

    let client_string_1 = u1
      .clients
      .iter()
      .map(|i| i.to_string())
      .collect::<Vec<String>>()
      .join("#");

    let client_string_2 = u2
      .clients
      .iter()
      .map(|i| i.to_string())
      .collect::<Vec<String>>()
      .join("#");

    NoteArchive::new()
      .write_users(vec![u1, u2], "test_write_users.txt")
      .unwrap();
    let file = File::open("test_write_users.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|item| item.unwrap()).collect::<Vec<_>>();

    // remove unneeded file
    fs::remove_file("test_write_users.txt").unwrap();

    assert_eq!(
      lines,
      vec![
        String::from("##### users #####"),
        format!("{} | {} | {} | {}", 1, "Pete", "ICC", client_string_1),
        format!("{} | {} | {} | {}", 2, "Johana", "FP", client_string_2),
        String::from("##### users #####"),
      ]
    );
  }

  #[test]
  fn can_read_users() {
    let mut lines = String::from("##### users #####\n");
    lines.push_str("1 | Pete | ICC | 1#2#3#4\n");
    lines.push_str("2 | Vivian | FP | 5#6#7#8\n");
    lines.push_str("3 | Sabrina | FP | 9#10#11#12\n");
    lines.push_str("4 | Dave | ICC | 13#24#35#46\n");
    lines.push_str("##### users #####");

    let mut file = File::create("test_read_users.txt").unwrap();
    file.write_all(lines.as_bytes()).unwrap();

    assert_eq!(
      NoteArchive::read_users("test_read_users.txt"),
      vec![
        User::new(1, String::from("Pete"), ICC, vec![1, 2, 3, 4],),
        User::new(2, String::from("Vivian"), FP, vec![5, 6, 7, 8],),
        User::new(3, String::from("Sabrina"), FP, vec![9, 10, 11, 12],),
        User::new(4, String::from("Dave"), ICC, vec![13, 24, 35, 46],),
      ]
    );
    // remove unneeded file
    fs::remove_file("test_read_users.txt").unwrap();
  }
  #[test]
  fn can_open_nonexistent() {
    {
      let users = NoteArchive::read_users("some_random_file_name.txt");
      assert_eq!(users, vec![]);
    }
    fs::remove_file("some_random_file_name.txt").unwrap();
  }
  #[test]
  fn creates_unique_new_user() {
    let user_1 = User::new(1, String::from("Pete"), ICC, vec![1, 2, 3, 4]);
    let user_2 = User::new(2, String::from("Sandy"), FP, vec![5, 6, 7, 8]);
    let users = vec![user_1, user_2];
    let mut notes = NoteArchive::new();
    notes.write_users(users, "test_unique_users.txt").unwrap();

    let new_user_attempt = notes.generate_new_user(
      String::from("Carl"),
      ICC,
      String::from("test_unique_users.txt"),
    );

    let new_user = match new_user_attempt {
      Ok(user) => user,
      Err(_) => panic!("Failed to generate user."),
    };

    assert_eq!(new_user, User::new(3, String::from("Carl"), ICC, vec![]));

    fs::remove_file("test_unique_users.txt").unwrap();
  }
  #[test]
  fn saves_user_to_file() {
    {
      let mut lines = String::from("##### users #####\n");
      lines.push_str("1 | Pete | ICC | 1#2#3#4\n");
      lines.push_str("2 | Sandy | FP | 5#6#7#8\n");
      lines.push_str("##### users #####");

      let mut file = File::create("test_save_user.txt").unwrap();
      file.write_all(lines.as_bytes()).unwrap();

      let mut notes = NoteArchive::new();
      let user = User::new(3, String::from("Carl"), ICC, vec![]);
      notes.save_user(user, "test_save_user.txt");

      assert_eq!(
        NoteArchive::read_users("test_save_user.txt"),
        vec![
          User::new(1, String::from("Pete"), ICC, vec![1, 2, 3, 4],),
          User::new(2, String::from("Sandy"), FP, vec![5, 6, 7, 8],),
          User::new(3, String::from("Carl"), ICC, vec![])
        ]
      );
    }
    fs::remove_file("test_save_user.txt").unwrap();
  }
}
