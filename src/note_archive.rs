use std::fmt;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::io;
use chrono::{NaiveDate, TimeZone, Utc, Local};

use crate::user::*;
use crate::client::*;
use crate::EmployeeRole::{FP, ICC};

pub struct NoteArchive {
  pub users: Vec<User>,
  pub current_user_id: Option<u32>,
  pub clients: Vec<Client>,
  pub current_client_ids: Option<Vec<u32>>,
}

impl NoteArchive {
  pub fn run(&mut self) {
    self.write_users(
      vec![
        User::new(
          1,
          String::from("Pete"),
          String::from("Peterson"),
          ICC,
          vec![],
        ),
        User::new(
          2,
          String::from("Charles"),
          String::from("Charleson"),
          FP,
          vec![],
        )
    ], "users.txt").unwrap();
    let user_id = self.choose_user("users.txt");
    self.load_users("users.txt");
    self.load_user(user_id, "users.txt").unwrap();

  }
  pub fn new() -> NoteArchive {
    NoteArchive {
      users: vec![],
      clients: vec![],
      current_user_id: None,
      current_client_ids: None,
    }
  }
  fn pub_display_users(&self) {
    println!("{:-^50}", "-");
    println!("{:-^50}", " Users ");
    println!("{:-^50}", "-");
    println!("{:-^6} | {:-^8} | {:-^30}", "ID", "ROLE", "NAME");
    for u in &self.users {
      println!("{: ^6} | {: ^8} | {: ^30}", u.id, u.role.to_string(), u.full_name());
    }
    println!("{:-^50}", "-");
  }
  fn load_users(&mut self, filepath: &str) {
    self.users = Self::read_users(filepath);
  }
  fn load_user(&mut self, id: u32, filepath: &str) -> std::io::Result<()> {
    self.load_users(filepath);
    let current: Option<&User> = self.users.iter().find(|u| u.id == id);
    match current {
      Some(u) => {
        self.current_client_ids = Some(u.clients.clone());
        self.current_user_id = Some(u.id);
        Ok(())
      },
      None => {
        Err(Error::new(ErrorKind::Other, "Failed to read user {} from filepath."))
      }
    }
  }
  fn choose_user(&mut self, filepath: &str) -> u32 {
    self.load_users(filepath);
    self.pub_display_users();
    
    let verified_id = loop {

      let chosen_id = loop {
        let input = loop {
          let mut choice = String::new();
          println!("Enter user ID (or 'NEW' to create new).");
          let read_attempt = io::stdin().read_line(&mut choice);
          match read_attempt {
            Ok(_) => break choice,
            Err(e) => {
              println!("Could not read input; try again ({}).", e);
              continue;
            },
          }
        };
        let input = input.trim();
        if input == "NEW" || input == "new" || input == "New" {
          let num = self.create_user_get_id(filepath);
          break num
        } else {
          match input.trim().parse() {
            Ok(num) => break num,
            Err(e) => {
              println!("input is {:?}", input);
              println!("Could not read input as a number; try again ({}).", e);
              continue;
            },
          }
        }
      };
      match self.load_user(chosen_id, filepath) {
        Ok(_) => break chosen_id,
        Err(e) => {
          println!("Unable to load user with id {}: {}", chosen_id, e);
          continue;
        }
      }
    };
    verified_id
  }

  // users

  pub fn create_user_get_id(&mut self, filepath: &str) -> u32 {
    
    let user = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        println!("Enter your first name.");
        let first_name_attempt = io::stdin().read_line(&mut first_name_choice);
        match first_name_attempt {
          Ok(_) => break String::from(first_name_choice.trim()),
          Err(e) => {
            println!("Invalid first name: {}", e);
            continue;
          },
        };
      };
      let last_name = loop {
        let mut last_name_choice = String::new();
        println!("Enter your last name.");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println!("Invalid last name: {}", e);
            continue;
          },
        };
      };
      let role: EmployeeRole = loop {
        let mut role_choice = String::new();
        println!("Enter your role ('ICC' or 'FP').");
        let role_attempt = io::stdin().read_line(&mut role_choice);
        match role_attempt {
          Ok(_) => match role_choice.trim() {
            "ICC" | "icc" | "Icc" => break ICC,
            "FP" | "fp" | "Fp" => break FP,
            _ => {
              println!("Please choose role 'FP' or 'ICC.'");
              continue;
            }
          } 
          Err(e) => {
            println!("Unreadable entry: {}", e);
            continue;
          },
        };
      };
      let user_attempt = self.generate_unique_new_user(first_name, last_name, role, filepath);
      match user_attempt {
        Ok(user) => break user,
        Err(e) => {
          println!("User could not be generated.");
          continue;
        }
      }
    };
    let id = user.id;
    self.save_user(user, filepath);
    id
  }
  pub fn generate_unique_new_user(
    &mut self,
    first_name: String,
    last_name: String,
    role: EmployeeRole,
    filepath: &str,
  ) -> Result<User, String> {
    let saved_users = NoteArchive::read_users(&filepath);
    let id: u32 = saved_users.len() as u32 + 1;

    let names_and_roles: Vec<(&str, &str, &EmployeeRole)> =
      saved_users.iter().map(|u| (&u.first_name[..], &u.last_name[..], &u.role)).collect();

    let result = if names_and_roles.iter().any(|(f, l, r)| f == &first_name && l == &last_name && *r == &role) {
      Err(format!(
        "There is already a {} with the name '{} {}'.",
        role, first_name, last_name
      ))
    } else {
      Ok(User::new(id, first_name, last_name, role, vec![]))
    };

    result
  }
  pub fn save_user(&mut self, user: User, filepath: &str) {
    let mut users: Vec<User> = NoteArchive::read_users(filepath);
    users.push(user);
    self.write_users(users, filepath).unwrap();
  }
  pub fn write_users(&mut self, users: Vec<User>, filepath: &str) -> std::io::Result<()> {
    let mut lines = String::from("##### users #####\n");
    for u in users {
      lines.push_str(&u.to_string());
    }
    lines.push_str("##### users #####");
    let mut file = File::create(filepath).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn read_users(filepath: &str) -> Vec<User> {

    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader
      .lines()
      .collect();

    if lines.len() > 0 {
      lines.remove(0).unwrap();
      }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1).unwrap();
    }

    let mut users: Vec<User> = vec![];

    for line in lines {
      let values: Vec<String> = line.unwrap().split(" | ").map(|val| val.to_string()).collect();

      let id: u32 = values[0].parse().unwrap();
      let first_name = String::from(&values[1]);
      let last_name = String::from(&values[2]);
      let role = match &values[3][..] {
        "FP" => Ok(FP),
        "ICC" => Ok(ICC),
        _ => Err("Invalid role."),
      }
      .unwrap();

      let clients: Vec<u32> = match &values[4][..] {
        "" => vec![],
        _ => values[4].split("#").map(|val| val.parse().unwrap()).collect(),
      };

      let u = User::new(id, first_name, last_name, role, clients);
      users.push(u);
    }
    users
  }
  pub fn generate_unique_new_client(
    &mut self,
    first_name: String,
    last_name: String,
    dob: NaiveDate,
    filepath: &str,
  ) -> Result<Client, String> {
    let saved_clients = NoteArchive::read_clients(&filepath);
    let id: u32 = saved_clients.len() as u32 + 1;

    let names_and_dobs: Vec<(&str, &str, &NaiveDate)> =
      saved_clients.iter().map(|c| (
        &c.first_name[..],
        &c.last_name[..],
        &c.dob,
      )).collect();

    let result = if names_and_dobs.iter()
      .any(|(f, l, d)| f == &first_name && l == &last_name && *d == &dob) {
      Err(format!(
        "There is already a '{} {}' with DOB '{}'.",
        first_name, last_name, dob
      ))
    } else {
      Ok(Client::new(id, first_name, last_name, dob, vec![]))
    };

    result
  }
  pub fn read_clients(filepath: &str) -> Vec<Client> {

    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader
      .lines()
      .collect();

    if lines.len() > 0 {
      lines.remove(0).unwrap();
      }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1).unwrap();
    }

    let mut clients: Vec<Client> = vec![];

    for line in lines {
      let values: Vec<String> = line.unwrap()
        .split(" | ").map(|val| val.to_string()).collect();

      let id: u32 = values[0].parse().unwrap();
      let first_name = String::from(&values[1]);
      let last_name = String::from(&values[2]);

      let date: Vec<i32> = match &values[3][..] {
        "" => vec![],
        _ => values[3].split("-").map(|val| val.parse().unwrap()).collect(),
      };

      let (year, month, day): (i32, u32, u32) = (date[0], date[1] as u32, date[2] as u32);

      let dob = NaiveDate::from_ymd(year, month, day);

      let collaterals: Vec<u32> = match &values[4][..] {
        "" => vec![],
        _ => values[4].split("#").map(|val| val.parse().unwrap()).collect(),
      };

      let c = Client::new(id, first_name, last_name, dob, collaterals);
      clients.push(c);
    }
    clients
  }
  pub fn write_clients(&mut self, clients: Vec<Client>, filepath: &str) -> std::io::Result<()> {
    let mut lines = String::from("##### clients #####\n");
    for c in clients {
      lines.push_str(&c.to_string());
    }
    lines.push_str("##### clients #####");
    let mut file = File::create(filepath).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn save_client(&mut self, client: Client, filepath: &str) {
    let mut clients: Vec<Client> = NoteArchive::read_clients(filepath);
    clients.push(client);
    self.write_clients(clients, filepath).unwrap();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_open_blank_users() {
    {
      let users = NoteArchive::read_users("some_random_user_file_name.txt");
      assert_eq!(users, vec![]);
    }
    fs::remove_file("some_random_user_file_name.txt").unwrap();
  }
  #[test]
  fn can_open_blank_clients() {
    {
      let users = NoteArchive::read_clients("some_random_client_file_name.txt");
      assert_eq!(users, vec![]);
    }
    fs::remove_file("some_random_client_file_name.txt").unwrap();
  }
  #[test]
  fn can_load_users() {
    {
      let mut a1 = NoteArchive::new();
      let test_user_1 = User::new(
        1,
        String::from("Bob"),
        String::from("Smith"),
        ICC,
        vec![1, 2, 3]
      );
      let test_user_2 = User::new(
        2,
        String::from("Gerald"),
        String::from("Ford"),
        FP,
        vec![1, 2, 3]
      );
      a1.write_users(vec![test_user_1, test_user_2], "test_load_user.txt").unwrap();
      a1.load_user(2, "test_load_user.txt").unwrap();
      assert_eq!(a1.current_user_id, Some(2));
    }
    fs::remove_file("test_load_user.txt").unwrap();
  }
  #[test]
  fn can_load_clients() {
    {
      let mut a1 = NoteArchive::new();
      let test_user_1 = User::new(
        1,
        String::from("Gary"),
        String::from("Shmonson"),
        ICC,
        vec![1, 2, 3]
      );
      let test_user_2 = User::new(
        2,
        String::from("Gerald"),
        String::from("Ford"),
        FP,
        vec![7, 8, 9]
      );
      a1.write_users(
        vec![test_user_1, test_user_2],
        "test_load_clients_from_user.txt").unwrap();
        a1.load_user(2, "test_load_clients_from_user.txt").unwrap();
        assert_eq!(a1.current_client_ids, Some(vec![7, 8, 9])
      );
    }
    fs::remove_file("test_load_clients_from_user.txt").unwrap();
  }

}