use chrono::{Local, NaiveDate, TimeZone, Utc};
use std::fmt;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::{thread, time};

use crate::client::*;
use crate::pronouns::*;
use crate::user::*;
use crate::EmployeeRole::{FP, ICC};

pub struct NoteArchive {
  pub users: Vec<User>,
  pub clients: Vec<Client>,
  pub pronouns: Vec<Pronouns>,
  pub current_user_id: Option<u32>,
  pub current_client_ids: Option<Vec<u32>>,
  pub current_client_id: Option<u32>,
  pub current_collateral_ids: Option<Vec<u32>>,
  pub current_collateral_id: Option<u32>,
  user_filepath: String,
  client_filepath: String,
  pronouns_filepath: String,
}

impl NoteArchive {
  pub fn run(&mut self) {
    NoteArchive::remove_test_files();
    thread::sleep(time::Duration::from_secs(3));
    self.choose_user();
    self.write_to_files();

    self.logged_in_action();
  }
  pub fn new(
    user_filepath: String,
    client_filepath: String,
    pronouns_filepath: String,
  ) -> NoteArchive {
    let mut a = NoteArchive {
      users: Self::read_users(&user_filepath),
      clients: Self::read_clients(&client_filepath),
      pronouns: vec![],
      current_user_id: None,
      current_client_ids: None,
      current_client_id: None,
      current_collateral_ids: None,
      current_collateral_id: None,
      user_filepath,
      client_filepath,
      pronouns_filepath,
    };
    a.read_pronouns();
    a
  }
  pub fn new_test() -> NoteArchive {
    let user_1 = User::new(
      1,
      String::from("Pete"),
      String::from("Peteson"),
      ICC,
      1,
      vec![1, 2, 3, 4],
    );
    let user_2 = User::new(
      2,
      String::from("Sandy"),
      String::from("Sandyson"),
      FP,
      1,
      vec![5, 6, 7, 8],
    );
    let users = vec![user_1, user_2];
    let client_1 = Client::new(
      1,
      String::from("Pete"),
      String::from("McLastName"),
      NaiveDate::from_ymd(2006, 1, 2),
      1,
      vec![1, 2, 3, 4],
    );
    let client_2 = Client::new(
      2,
      String::from("Sandy"),
      String::from("O'Lastnymn"),
      NaiveDate::from_ymd(2007, 2, 3),
      1,
      vec![5, 6, 7, 8],
    );
    let clients = vec![client_1, client_2];
    let p1 = Pronouns::new(
      1,
      String::from("he"),
      String::from("him"),
      String::from("his"),
      String::from("his"),
    );
    let p2 = Pronouns::new(
      2,
      String::from("she"),
      String::from("her"),
      String::from("her"),
      String::from("hers"),
    );
    let pronouns = vec![p1, p2];
    let mut notes = NoteArchive::new(
      String::from("test_user.txt"),
      String::from("test_client.txt"),
      String::from("test_pronouns.txt"),
    );

    notes.users = users;
    notes.clients = clients;
    notes.pronouns = pronouns;
    notes.write_to_files();

    notes
  }
  pub fn remove_test_files() {
    if fs::metadata("test_user.txt").is_ok() {
      fs::remove_file("test_user.txt").unwrap();
    }
    if fs::metadata("test_client.txt").is_ok() {
      fs::remove_file("test_client.txt").unwrap();
    }
    if fs::metadata("test_pronouns.txt").is_ok() {
      fs::remove_file("test_pronouns.txt").unwrap();
    }
  }
  pub fn write_to_files(&mut self) {
    self.write_users().unwrap();
    self.write_clients().unwrap();
    self.write_pronouns().unwrap();
  }
  pub fn display_actions(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^58}", "-");
    let heading_with_spaces = format!(" {} ", self.current_user().name_and_title()); 
    println!("{:-^58}", heading_with_spaces);
    println!("{:-^58}", " Mission control ");
    println!("{:-^58}", "-");
    println!("{:-^15} | {:-^40}", " Command ", " Function ");
    println!("{:-^58}", "-");

    // once for each command

    println!(
      "{: >15} | {: <40}",
      " CLIENT / C ", " View/edit client records "
    );
    println!(
      "{: >15} | {: <40}",
      " EDIT / E ", " Edit current user info "
    );
    println!("{: >15} | {: <40}", " PRNS / P ", " View/edit pronoun records ");
    println!("{: >15} | {: <40}", " USER / U ", " Switch user ");
    println!("{: >15} | {: <40}", " DELETE / D ", " Delete current user ");
    println!("{: >15} | {: <40}", " QUIT / Q ", " End program ");

    println!("{:-^58}", "-");
  }
  pub fn logged_in_action(&mut self) {
    loop {
      self.display_actions();

      let mut choice = String::new();
      let choice_attempt = io::stdin().read_line(&mut choice);
      match choice_attempt {
        Ok(_) => (),
        Err(e) => {
          println!("Failed to read input. Please try again.");
        }
      }
      choice = choice.trim().to_string();
      match &choice[..] {
        "quit" | "q" | "QUIT" | "Q" | "Quit" => {
          break ();
        },
        "edit" | "e" | "EDIT" | "E" | "Edit" => {
          self.choose_edit_user();
        },
        "delete" | "d" | "DELETE" | "D" | "Delete" => {
          self.choose_delete_user();
          self.choose_user();
        },
        "client" | "c" | "CLIENT" | "C" | "Client" => {
          self.choose_clients();
        },
        "user" | "u" | "USER" | "U" | "User" => {
          self.choose_user();
        },
        "PRNS" | "Prns" | "prns" | "P" | "p" | "pronouns" | "Pronouns" | "PRONOUNS" => {
          let chosen_pronoun = self.choose_pronouns_option();
          match chosen_pronoun {
            Some(prn) => {
              self.view_pronoun(chosen_pronoun.unwrap());
            },
            None => (),
          }
        },
        _ => {
          println!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
        },
      }
      self.write_to_files();
    }
  }

  // users

  pub fn current_user_mut(&mut self) -> &mut User {
    let user_id = match self.current_user_id {
      Some(id) => id,
      None => panic!("There is no user loaded."),
    };
    let maybe_current: Option<&mut User> = self.users.iter_mut().find(|u| u.id == user_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded user ID does not match any saved users."),
    }
  }
  pub fn current_user(&self) -> &User {
    let user_id = match self.current_user_id {
      Some(id) => id,
      None => panic!("There is no user loaded."),
    };
    let maybe_current: Option<&User> = self.users.iter().find(|u| u.id == user_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded user id does not match any saved users."),
    }
  }
  fn display_users(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^66}", "-");
    println!("{:-^66}", " Users ");
    println!("{:-^66}", "-");
    println!("{:-^10} | {:-^10} | {:-^40}", "ID", "Role", "Name");
    for u in &self.users {
      println!(
        "{: ^10} | {: ^10} | {: ^40}",
        u.id,
        u.role.to_string(),
        u.full_name()
      );
    }
    println!("{:-^66}", "-");
  }
  fn display_edit_user(&self) {
    let pronouns_option = self.get_pronouns_by_id(self.current_user().pronouns);
    let display_pronouns = match pronouns_option {
      Some(p) => p.short_string(),
      None => String::from("-----"),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^90}", "-");
    println!("{:-^90}", " Edit user ");
    println!("{:-^90}", "-");
    println!(
      "{:-^10} | {:-^20} | {:-^20} | {:-^30}",
      "Role", "First name", "Last name", "Pronouns"
    );
    println!(
      "{: ^10} | {: ^20} | {: ^20} | {: ^30}",
      self.current_user().role.to_string(),
      self.current_user().first_name,
      self.current_user().last_name,
      display_pronouns,
    );
    println!("{:-^90}", "-");
    println!("Choose field to edit (FIRST, LAST, ROLE, PRNS).");
    println!("'Q'/'QUIT' to return to previous menu.");
  }
  pub fn load_user(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&User> = self.users.iter().find(|u| u.id == id);
    match current {
      Some(u) => {
        let prns_id = u.pronouns;
        self.current_client_ids = Some(u.clients.clone());
        self.current_user_id = Some(u.id);
        match self.load_pronouns(prns_id) {
          Ok(_) => (),
          Err(e) => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Error: {} Pronoun record not found. Please select pronouns again.", e);
            thread::sleep(time::Duration::from_secs(2));
            self.current_user_mut().pronouns = self.choose_pronouns();
          }
        }
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to read user from filepath.",
      )),
    }
  }
  fn choose_user(&mut self) -> u32 {
    self.display_users();
    let verified_id = loop {
      let chosen_id = loop {
        let input = loop {
          let mut choice = String::new();
          println!("| {} | {}", "Enter ID to choose user.", "NEW / N: new user");
          let read_attempt = io::stdin().read_line(&mut choice);
          match read_attempt {
            Ok(_) => break choice,
            Err(e) => {
              println!("Could not read input; try again ({}).", e);
            continue;
            }
          }
        };
        let input = input.trim();
        match input {
          "NEW" | "new" | "New" | "N" | "n" => {
            let num = self.create_user_get_id();
            break num;
          }
          _ => match input.parse() {
            Ok(num) => break num,
            Err(e) => {
              println!("Could not read input as a number; try again ({}).", e);
              continue;
            }
          },
        }
      };
      match self.load_user(chosen_id) {
        Ok(_) => break chosen_id,
        Err(e) => {
          println!("Unable to load user with id {}: {}", chosen_id, e);
          continue;
        }
      }
    };
    verified_id
  }
  fn create_user_get_id(&mut self) -> u32 {
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
          }
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
          }
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
          },
          Err(e) => {
            println!("Unreadable entry: {}", e);
            continue;
          }
        };
      };
      let pronouns = self.choose_pronouns();
      let user_attempt = self.generate_unique_new_user(first_name, last_name, role, pronouns);
      match user_attempt {
        Ok(user) => break user,
        Err(e) => {
          println!("User could not be generated: {}.", e);
          continue;
        }
      }
    };
    let id = user.id;
    self.save_user(user);
    id
  }
  pub fn generate_unique_new_user(
    &mut self,
    first_name: String,
    last_name: String,
    role: EmployeeRole,
    pronouns: u32,
  ) -> Result<User, String> {
    let id: u32 = self.users.len() as u32 + 1;

    let names_and_roles: Vec<(&str, &str, &EmployeeRole)> = self
      .users
      .iter()
      .map(|u| (&u.first_name[..], &u.last_name[..], &u.role))
      .collect();

    let result = if names_and_roles
      .iter()
      .any(|(f, l, r)| f == &first_name && l == &last_name && *r == &role)
    {
      Err(format!(
        "There is already a {} with the name '{} {}'.",
        role, first_name, last_name
      ))
    } else {
      Ok(User::new(id, first_name, last_name, role, pronouns, vec![]))
    };

    result
  }
  pub fn save_user(&mut self, user: User) {
    self.users.push(user);
  }
  pub fn write_users(&mut self) -> std::io::Result<()> {
    let mut lines = String::from("##### users #####\n");
    for u in &self.users {
      lines.push_str(&u.to_string());
    }
    lines.push_str("##### users #####");
    let mut file = File::create(self.user_filepath.clone()).unwrap();
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

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0).unwrap();
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1).unwrap();
    }

    let mut users: Vec<User> = vec![];

    for line in lines {
      let values: Vec<String> = line
        .unwrap()
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let id: u32 = values[0].parse().unwrap();
      let first_name = String::from(&values[1]);
      let last_name = String::from(&values[2]);
      let role = match &values[3][..] {
        "FP" => Ok(FP),
        "ICC" => Ok(ICC),
        _ => Err("Invalid role."),
      }
      .unwrap();
      let pronouns: u32 = values[4].parse().unwrap();

      let clients: Vec<u32> = match &values[5][..] {
        "" => vec![],
        _ => values[5]
        .split("#")
        .map(|val| val.parse().unwrap())
          .collect(),
      };

      
      let u = User::new(id, first_name, last_name, role, pronouns, clients);
      users.push(u);
    }
    users
  }
  fn change_user_first_name(&mut self, new_name: &str) -> Result<(), String> {
    let names_and_roles: Vec<(&str, &str, &EmployeeRole)> = self
      .users
      .iter()
      .map(|u| (&u.first_name[..], &u.last_name[..], &u.role))
      .collect();

    let (cf, cl, cr): (&str, &str, &EmployeeRole) = (
      new_name,
      &self.current_user().last_name,
      &self.current_user().role,
    );

    let result = if names_and_roles
      .iter()
      .any(|(f, l, r)| f == &cf && l == &cl && r == &cr)
    {
      Err(format!(
        "There is already a {} with the name '{} {}'.",
        cr, cf, cl
      ))
    } else {
      self.current_user_mut().first_name = String::from(new_name);
      Ok(())
    };
    result
  }
  fn change_user_last_name(&mut self, new_name: &str) -> Result<(), String> {
    let names_and_roles: Vec<(&str, &str, &EmployeeRole)> = self
      .users
      .iter()
      .map(|u| (&u.first_name[..], &u.last_name[..], &u.role))
      .collect();

    let (cf, cl, cr): (&str, &str, &EmployeeRole) = (
      &self.current_user().first_name,
      new_name,
      &self.current_user().role,
    );

    let result = if names_and_roles
      .iter()
      .any(|(f, l, r)| f == &cf && l == &cl && r == &cr)
    {
      Err(format!(
        "There is already a {} with the name '{} {}'.",
        cr, cf, cl
      ))
    } else {
      self.current_user_mut().last_name = String::from(new_name);
      Ok(())
    };
    result
  }
  fn change_role(&mut self, new_role: &EmployeeRole) -> Result<(), String> {
    let names_and_roles: Vec<(&str, &str, &EmployeeRole)> = self
      .users
      .iter()
      .map(|u| (&u.first_name[..], &u.last_name[..], &u.role))
      .collect();

    let (cf, cl, cr): (&str, &str, &EmployeeRole) = (
      &self.current_user().first_name,
      &self.current_user().last_name,
      new_role,
    );

    let result = if names_and_roles
      .iter()
      .any(|(f, l, r)| f == &cf && l == &cl && r == &cr)
    {
      Err(format!(
        "There is already a {} with the name '{} {}'.",
        cr, cf, cl
      ))
    } else {
      self.current_user_mut().role = new_role.clone();
      Ok(())
    };
    result
  }
  fn choose_edit_user(&mut self) {
    loop {
      self.display_edit_user();
      let mut field_to_edit = String::new();
      let input_attempt = io::stdin().read_line(&mut field_to_edit);
      match input_attempt {
        Ok(_) => (),
        Err(e) => {
          println!("Failed to read input. Please try again.");
          continue;
        }
      }
      field_to_edit = field_to_edit.trim().to_string();
      match &field_to_edit[..] {
        "quit" | "q" | "QUIT" | "Q" | "Quit" => {
          break ();
        }
        _ => (),
      }
      match &field_to_edit[..] {
        "FIRST" | "First" | "first" | "fst" | "f" | "F" | "1st" | "first name" | "First name"
        | "FIRST NAME" | "First Name" => {
          println!("Enter new first name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_user_first_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println!("Error: {}", e);
              }
            },
            Err(e) => {
              println!("Error: {}", e);
              continue;
            }
          }
        }
        "LAST" | "Last" | "last" | "lst" | "l" | "L" | "last name" | "Last name" | "LAST NAME"
        | "Last Name" => {
          println!("Enter new last name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_user_last_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println!("Error: {}", e);
              }
            },
            Err(e) => {
              println!("Error: {}", e);
              continue;
            }
          }
        }
        "ROLE" | "Role" | "role" | "r" | "R" => match self.current_user().role {
          ICC => {
            self.change_role(&FP).unwrap();
          }
          FP => {
            self.change_role(&ICC).unwrap();
          }
        },
        "PRNS" | "Prns" | "prns" | "P" | "p" | "pronouns" | "Pronouns" | "PRONOUNS" => {
          self.current_user_mut().pronouns = self.choose_pronouns();
        }
        _ => println!("Invalid entry."),
      }
    }
  }
  fn choose_delete_user(&mut self) {
    loop {
      self.display_delete_user();
      println!("Are you sure you want to delete this user?");
      println!("| {} | {}", "YES / Y: confirm", "QUIT / Q: cancel");
      let mut choice = String::new();
      let input_attempt = io::stdin().read_line(&mut choice);
      match input_attempt {
        Ok(_) => choice = choice.trim().to_string(),
        Err(e) => {
          println!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      }
      match &choice[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          self.delete_current_user();
          break;
        },
        "QUIT" | "quit" | "Q" | "q" => break,
        _ => {
          println!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
          break;
        },
      }
    }
  }
  fn display_delete_user(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^79}", "-");
    println!("{:-^79}", " Delete user ");
    println!("{:-^79}", "-");
    println!(
      "{:-^10} | {:-^20} | {:-^20} | {:-^20}",
      "Role", "First name", "Last name", "Client records",
    );
    println!(
      "{: ^10} | {: ^20} | {: ^20} | {: ^20}",
      self.current_user().role.to_string(),
      self.current_user().first_name,
      self.current_user().last_name,
      &self.current_client_ids.as_ref().unwrap().len(),
    );
    println!("{:-^79}", "-");
  }
  fn delete_current_user(&mut self) {
    let id = self.current_user_id.unwrap();
    self.users.retain(|u| u.id != id);
    self.reindex_users();
    self.current_user_id = None;
    self.current_client_ids = None;
    self.current_client_id = None;
    self.current_collateral_ids = None;
    self.current_collateral_id = None;

  }
  fn reindex_users(&mut self) {
    let mut i: u32 = 1;
    for mut u in &mut self.users {
      u.id = i;
      i += 1;
    }
  }

  // clients
  fn current_client_mut(&mut self) -> &mut Client {
    let client_id = match self.current_client_id {
      Some(id) => id,
      None => panic!("There is no current client selected."),
    };
    let maybe_current: Option<&mut Client> = self.clients.iter_mut().find(|c| c.id == client_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded client ID does not match any saved clients."),
    }
  }
  fn current_client(&self) -> &Client {
    let client_id = match self.current_client_id {
      Some(id) => id,
      None => panic!("There is no current client selected."),
    };
    let maybe_current: Option<&Client> = self.clients.iter().find(|c| c.id == client_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded user id does not match any saved users."),
    }
  }
  fn display_clients(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", " Clients ");
    println!("{:-^96}", "-");
    println!("{:-^10} | {:-^40} | {:-^40}", "ID", "NAME", "DOB");
    match self.current_client_ids {
      Some(_) => {
        for c in self.clients.iter().filter(|client| {
          self
            .current_client_ids
            .as_ref()
            .unwrap()
            .iter()
            .any(|&id| id == client.id)
        }) {
          println!(
            "{: ^10} | {: ^40} | {: <12} {: >26}",
            c.id,
            c.full_name(),
            c.fmt_dob(),
            c.fmt_date_of_birth()
          );
        }
      }
      None => (),
    }
    println!("{:-^96}", "-");
    println!("| {} | {} | {}", "Enter ID to choose client.", "NEW / N: new client", "QUIT / Q: quit menu");
  }
  fn display_client(&self) {
      let pronouns_option = self.get_pronouns_by_id(self.current_client().pronouns);
    let display_pronouns = match pronouns_option {
      Some(p) => p.short_string(),
      None => String::from("-----"),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^119}", "-");
    println!("{:-^119}", " View client record ");
    println!("{:-^119}", "-");
    println!(
      "{:-^20} | {:-^20} | {:-^30} | {:-^40}",
      "First name", "Last name", "Pronouns", "DOB"
    );
    println!(
      "{: ^20} | {: ^20} | {: ^30} | {: <12} {: >26}",
      self.current_client().first_name,
      self.current_client().last_name,
      display_pronouns,
      self.current_client().fmt_dob(),
      self.current_client().fmt_date_of_birth(),
    );
    println!("{:-^119}", "-");
    println!("| {} | {} | {}", "EDIT / E: edit client", "DELETE: delete client", "QUIT / Q: quit menu");
    println!("{:-^119}", "-");
  }
  fn load_client(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&Client> = self.clients.iter().find(|c| c.id == id);
    match current {
      Some(c) => {
        self.current_collateral_ids = Some(c.collaterals.clone());
        self.current_client_id = Some(c.id);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to read client from filepath.",
      )),
    }
  }
  fn choose_clients(&mut self) {
    loop {
      let input = loop {
        self.display_clients();
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice,
          Err(e) => {
            println!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "NEW" | "new" | "New" | "n" | "N" => {
          let new_id = self.create_client_get_id();
          self.update_current_clients(new_id);
          continue;
        },
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            match self.load_client(num) {
              Ok(_) => self.choose_client(),
              Err(e) => {
                println!("Unable to load client with id {}: {}", num, e);
                continue;
              }
            }
          },
          Err(e) => {
            println!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn choose_client(&mut self) {
    loop {
      self.display_client();
      let mut choice = String::new();
      let read_attempt = io::stdin().read_line(&mut choice);
      let input = match read_attempt {
        Ok(_) => choice,
        Err(e) => {
          println!("Could not read input; try again ({}).", e);
          continue;
        }
      };
      let input = input.trim();
      match input {
        "QUIT" | "quit" | "Quit" | "Q" | "q" => {
          break;
        }
        "DELETE" | "delete" | "Delete" | "d" | "D" => {
          self.choose_delete_client();
          break;
        }
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          self.choose_edit_client();
        }
        _ => println!("Invalid command."),
      }
    }
  }
  fn create_client_get_id(&mut self) -> u32 {
    let client = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        println!("Enter client's first name.");
        let first_name_attempt = io::stdin().read_line(&mut first_name_choice);
        match first_name_attempt {
          Ok(_) => break String::from(first_name_choice.trim()),
          Err(e) => {
            println!("Invalid first name: {}", e);
            continue;
          }
        };
      };
      let last_name = loop {
        let mut last_name_choice = String::new();
        println!("Enter client's last name.");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println!("Invalid last name: {}", e);
            continue;
          }
        };
      };
      let dob: NaiveDate = loop {
        let birth_year = loop {
          let mut birth_year_choice = String::new();
          println!("Enter client's birth year.");
          let birth_year_attempt = io::stdin().read_line(&mut birth_year_choice);
          let birth_year_attempt = match birth_year_attempt {
            Ok(_) => birth_year_choice.trim().parse(),
            Err(e) => {
              println!("Invalid birth year: {}", e);
              continue;
            }
          };
          let birth_year_input = match birth_year_attempt {
            Ok(val) => val,
            Err(e) => {
              println!("Invalid birth year: {}", e);
              continue;
            }
          };
          if birth_year_input > 9999 || birth_year_input < 1000 {
            println!("Please enter a valid year.");
            continue;
          }
          break birth_year_input;
        };
        let birth_month = loop {
          let mut birth_month_choice = String::new();
          println!("Enter client's birth month as a decimal number (1-12).");
          let birth_month_attempt = io::stdin().read_line(&mut birth_month_choice);
          let birth_month_attempt = match birth_month_attempt {
            Ok(_) => birth_month_choice.trim().parse(),
            Err(e) => {
              println!("Invalid birth month: {}", e);
              continue;
            }
          };
          let birth_month_input = match birth_month_attempt {
            Ok(val) => val,
            Err(e) => {
              println!("Invalid birth month: {}", e);
              continue;
            }
          };
          if birth_month_input > 12 || birth_month_input < 1 {
            println!("Please enter a valid month using decimal numbers 1-12.");
            continue;
          }
          break birth_month_input;
        };
        let birth_day = loop {
          let mut birth_day_choice = String::new();
          println!("Enter client's birth day as a decimal number (1-31).");
          let birth_day_attempt = io::stdin().read_line(&mut birth_day_choice);
          let birth_day_attempt = match birth_day_attempt {
            Ok(_) => birth_day_choice.trim().parse(),
            Err(e) => {
              println!("Invalid birth day: {}", e);
              continue;
            }
          };
          let birth_day_input = match birth_day_attempt {
            Ok(val) => val,
            Err(e) => {
              println!("Invalid birth day: {}", e);
              continue;
            }
          };
          if birth_day_input > 31 || birth_day_input < 1 {
            println!("Please enter a valid day using decimal numbers 1-12.");
            continue;
          }
          break birth_day_input;
        };

        match NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day) {
          Some(date) => break date,
          None => {
            println!(
              "{}-{}-{} does not appear to be a valid date. Please try again.",
              birth_year, birth_month, birth_day
            );
            continue;
          }
        };
      };

      let pronouns = self.choose_pronouns();

      let client_attempt = self.generate_unique_new_client(first_name, last_name, dob, pronouns);
      match client_attempt {
        Ok(client) => break client,
        Err(e) => {
          println!("Client could not be generated: {}.", e);
          continue;
        }
      }
    };
    let id = client.id;
    self.save_client(client);
    id
  }
  pub fn generate_unique_new_client(
    &mut self,
    first_name: String,
    last_name: String,
    dob: NaiveDate,
    pronouns: u32,
  ) -> Result<Client, String> {
    let id: u32 = self.clients.len() as u32 + 1;

    let names_and_dobs: Vec<(&str, &str, &NaiveDate)> = self
      .clients
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.dob))
      .collect();

    let result = if names_and_dobs
      .iter()
      .any(|(f, l, d)| f == &first_name && l == &last_name && *d == &dob)
    {
      Err(format!(
        "There is already a '{} {}' with DOB '{}'.",
        first_name, last_name, dob
      ))
    } else {
      Ok(Client::new(
        id,
        first_name,
        last_name,
        dob,
        pronouns,
        vec![],
      ))
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

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0).unwrap();
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1).unwrap();
    }

    let mut clients: Vec<Client> = vec![];

    for line in lines {
      let values: Vec<String> = line
        .unwrap()
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let id: u32 = values[0].parse().unwrap();
      let first_name = String::from(&values[1]);
      let last_name = String::from(&values[2]);

      let date: Vec<i32> = match &values[3][..] {
        "" => vec![],
        _ => values[3]
          .split("-")
          .map(|val| val.parse().unwrap())
          .collect(),
      };
      let (year, month, day): (i32, u32, u32) = (date[0], date[1] as u32, date[2] as u32);
      let dob = NaiveDate::from_ymd(year, month, day);
      let pronouns: u32 = values[4].parse().unwrap();
      let collaterals: Vec<u32> = match &values[5][..] {
        "" => vec![],
        _ => values[5]
          .split("#")
          .map(|val| val.parse().unwrap())
          .collect(),
      };

      let c = Client::new(id, first_name, last_name, dob, pronouns, collaterals);
      clients.push(c);
    }
    clients
  }
  pub fn write_clients(&self) -> std::io::Result<()> {
    let mut lines = String::from("##### clients #####\n");
    for c in &self.clients {
      lines.push_str(&c.to_string()[..]);
    }
    lines.push_str("##### clients #####");
    let mut file = File::create(self.client_filepath.clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn save_client(&mut self, client: Client) {
    self.clients.push(client);
  }
  fn update_current_clients(&mut self, id: u32) {
    self.current_user_mut().clients.push(id);
    self.current_client_ids = Some(self.current_user_mut().clients.clone());
  }
  fn choose_edit_client(&mut self) {
    loop {
      self.display_client();
      println!("| {} | {} | {} | {}", "FIRST / F: edit first name", "LAST / L: edit surname", "PRNS / P: edit pronouns", "QUIT / Q: cancel");
      let mut field_to_edit = String::new();
      let input_attempt = io::stdin().read_line(&mut field_to_edit);
      match input_attempt {
        Ok(_) => (),
        Err(e) => {
          println!("Failed to read input. Please try again.");
          continue;
        }
      }
      field_to_edit = field_to_edit.trim().to_string();
      match &field_to_edit[..] {
        "quit" | "q" | "QUIT" | "Q" | "Quit" => {
          break ();
        }
        _ => (),
      }
      match &field_to_edit[..] {
        "FIRST" | "First" | "first" | "fst" | "f" | "F" | "1st" | "first name" | "First name"
        | "FIRST NAME" | "First Name" => {
          println!("Enter new first name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_client_first_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println!("Error: {}", e);
              }
            },
            Err(e) => {
              println!("Error: {}", e);
              continue;
            }
          }
        }
        "LAST" | "Last" | "last" | "lst" | "l" | "L" | "last name" | "Last name" | "LAST NAME"
        | "Last Name" => {
          println!("Enter new last name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_client_last_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println!("Error: {}", e);
                thread::sleep(time::Duration::from_secs(1));
              }
            },
            Err(e) => {
              println!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        }
        "PRNS" | "Prns" | "prns" | "P" | "p" | "pronouns" | "Pronouns" | "PRONOUNS" => {
          self.current_client_mut().pronouns = self.choose_pronouns();
        }
        _ => {
          println!("Invalid entry.");
          thread::sleep(time::Duration::from_secs(1));
        }
      }
    }
  }
  fn change_client_first_name(&mut self, new_name: &str) -> Result<(), String> {
    let names_and_dobs: Vec<(&str, &str, &NaiveDate)> = self
      .clients
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.dob))
      .collect();

    let (cf, cl, cd): (&str, &str, &NaiveDate) = (
      new_name,
      &self.current_client().last_name,
      &self.current_client().dob,
    );

    let result = if names_and_dobs
      .iter()
      .any(|(f, l, d)| f == &cf && l == &cl && d == &cd)
    {
      Err(format!(
        "There is already a '{} {}' with DOB '{}'.",
        cf, cl, cd
      ))
    } else {
      self.current_client_mut().first_name = String::from(new_name);
      Ok(())
    };
    result
  }
  fn change_client_last_name(&mut self, new_name: &str) -> Result<(), String> {
    let names_and_dobs: Vec<(&str, &str, &NaiveDate)> = self
      .clients
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.dob))
      .collect();

    let (cf, cl, cd): (&str, &str, &NaiveDate) = (
      &self.current_client().first_name,
      new_name,
      &self.current_client().dob,
    );

    let result = if names_and_dobs
      .iter()
      .any(|(f, l, d)| f == &cf && l == &cl && d == &cd)
    {
      Err(format!(
        "There is already a '{} {}' with DOB '{}'.",
        cf, cl, cd
      ))
    } else {
      self.current_client_mut().last_name = String::from(new_name);
      Ok(())
    };
    result
  }
  fn choose_delete_client(&mut self) {
    loop {
      self.display_delete_client();
      println!("Are you sure you want to delete this client?");
      println!("| {} | {}", "YES / Y: confirm", "Any key to cancel");
      let mut confirm = String::new();
      let input_attempt = io::stdin().read_line(&mut confirm);
      let command = match input_attempt {
        Ok(_) => confirm.trim().to_string(),
        Err(e) => {
          println!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &command[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          self.delete_current_client();
          break;
        }
        _ => {
          continue;
        }
      }
    }
  }
  fn display_delete_client(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^114}", "-");
    println!("{:-^114}", " DELETE CLIENT ");
    println!("{:-^114}", "-");
    println!(
      "{:-^20} | {:-^20} | {:-^40} | {:-^25}",
      "First name", "Last name", "DOB", "Collateral records",
    );
    println!(
      "{: ^20} | {: ^20} | {: <12} {: >26} | {: ^25}",
      self.current_client().first_name,
      self.current_client().last_name,
      self.current_client().fmt_dob(),
      self.current_client().fmt_date_of_birth(),
      &self.current_collateral_ids.as_ref().unwrap().len(),
    );
    println!("{:-^114}", "-");
  }
  fn delete_current_client(&mut self) {
    let id = self.current_client_id.unwrap();
    self.clients.retain(|c| c.id != id);
    self.reindex_clients();
    self.current_client_id = None;
    self.current_collateral_ids = None;
    self.current_collateral_id = None;
  }
  fn reindex_clients(&mut self) {
    let mut i: u32 = 1;
    for mut c in &mut self.clients {
      for u in &mut self.users {
        for c_id in &mut u.clients {
          if c_id == &c.id {
            *c_id = i;
          }
        }
      }
      c.id = i;
      i += 1;
    }
  }

  // pronouns

  pub fn read_pronouns(&mut self) {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(self.pronouns_filepath.clone())
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0).unwrap();
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1).unwrap();
    }

    let mut pronouns: Vec<Pronouns> = vec![
      Pronouns::new(
        1,
        String::from("he"),
        String::from("him"),
        String::from("his"),
        String::from("his"),
      ),
      Pronouns::new(
        2,
        String::from("she"),
        String::from("her"),
        String::from("her"),
        String::from("hers"),
      ),
      Pronouns::new(
        3,
        String::from("they"),
        String::from("them"),
        String::from("their"),
        String::from("theirs"),
      ),
    ];

    for line in lines {
      let values: Vec<String> = line
        .unwrap()
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let saved_id: u32 = values[0].parse().unwrap();

      // if any pronouns have a matching ID
      // due to someone editing the default values,
      // change ID to last item in vector + 1, continuing count

      let subject = String::from(&values[1]);
      let object = String::from(&values[2]);
      let possessive_determiner = String::from(&values[3]);
      let possessive = String::from(&values[4]);

      let next_id = pronouns[pronouns.len() - 1].id + 1;
      
      let s2 = subject.clone();
      let o2 = object.clone();
      let pd2 = possessive_determiner.clone();
      let p2 = possessive.clone();

      let p = Pronouns::new(next_id, subject, object, possessive_determiner, possessive);
      
      if !pronouns.iter().any(|prn| prn == &p) {
        if pronouns.iter().any(|p| p.id == saved_id) {
          self.reassign_pronouns_id(saved_id, next_id);
          pronouns.push(p);
        } else {
          let p = Pronouns::new(saved_id, s2, o2, pd2, p2);
          pronouns.push(p);
        }
      }
    }
    self.pronouns = pronouns;
  }
  pub fn reassign_pronouns_id(&mut self, old_id: u32, new_id: u32) {
    let mut i = 0;
    while i < self.users.len() - 1 {
      let mut u = &mut self.users[i];
      if u.pronouns == old_id {
        u.pronouns = new_id
      }
      i += 1;
    }
  }
  fn choose_pronouns(&mut self) -> u32 {
    let chosen_id = loop {
      self.display_pronouns();
      let input = loop {
        let mut choice = String::new();
        println!(
          "| {} | {} | {}",
          "Enter ID to choose pronouns.",
          "EDIT / E: edit pronouns",
          "DELETE: delete pronouns",
        );
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice,
          Err(e) => {
            println!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();

      match input {
        "NEW" | "new" | "New" | "N" | "n" => {
          let pronouns_option = self.create_get_pronouns();
          match pronouns_option {
            Some(p) => {
              let new_id = p.id;
              break new_id;
            },
            None => continue,
          }
        },
        "EDIT" | "edit" | "Edit" | "E" | "e" => {
          self.choose_edit_pronouns();
          continue;
        },
        "DELETE" | "delete" | "Delete" | "D" | "d" => {
          self.choose_delete_pronouns();
          continue;
        },
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println!("Could not read input as a number; try again ({}).", e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          };
          match self.load_pronouns(id) {
            Ok(_) => break id,
            Err(e) => {
              println!("Unable to load pronouns with id {}: {}", input, e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        },
      }
    };
    chosen_id
  }
  fn choose_pronouns_option(&mut self) -> Option<u32> {
    let id_option = loop {
      self.display_pronouns();
      let input = loop {
        let mut choice = String::new();
        println!("| {} | {} | {} | {}", "NEW / N: new", "EDIT / E: edit (for all)", "DELETE / D: delete (for all)", "QUIT / Q: quit menu");
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice,
          Err(e) => {
            println!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();

      match input {
        "NEW" | "new" | "New" | "N" | "n" => {
          let pronouns_option = self.create_get_pronouns();
          match pronouns_option {
            Some(p) => {
              let new_id = p.id;
              break Some(new_id);
            },
            None => continue,
          }
        },
        "EDIT" | "edit" | "Edit" | "E" | "e" => {
          self.choose_edit_pronouns();
          continue;
        },
        "DELETE" | "delete" | "D" | "d" => {
          self.choose_delete_pronouns();
          continue;
        },
        "QUIT" | "quit" | "Quit" | "Q" | "q" => {
          break None;
        },
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println!("Could not read input as a number; try again ({}).", e);
              continue;
            }
          };
          match self.load_pronouns(id) {
            Ok(_) => break Some(id),
            Err(e) => {
              println!("Unable to load pronouns with id {}: {}", input, e);
              continue;
            }
          }
        },
      }
    };
    id_option
  }
  fn display_pronouns(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^44}", "-");
    println!("{:-^44}", " Pronouns ");
    println!("{:-^44}", "-");
    println!("{:-^10} | {:-^31}", "ID", "Pronouns");
    for p in &self.pronouns {
      println!("{: ^10} | {: ^31}", p.id, p.short_string());
    }
    println!("{:-^44}", "-");
  }
  fn display_view_pronoun(&self, prns_id: u32) {
    let prns = self.get_pronouns_by_id(prns_id).unwrap();
    let mut title = String::from(" ");
    title.push_str(&prns.short_string()[..]);
    title.push_str(" ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^69}", "-");
    println!("{:-^69}", " Edit pronouns ");
    println!("{:-^69}", title);
    println!("{:-^69}", "-");
    println!("{:-^15} | {:-^15} | {:-^15} | {:-^15}", "Subject", "Object", "Pos. Det.", "Pos.");
    println!("{: ^15} | {: ^15} | {: ^15} | {: ^15}", prns.subject, prns.object, prns.possessive_determiner, prns.possessive);
    println!("{:-^69}", "-");
  }
  fn display_pronoun_examples(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^109}", "-");
    println!("{: ^25} | {: ^25} | {: ^25} | {: ^25}", "Subject pronoun", "Object pronoun", "Possessive determiner", "Possessive pronoun");
    println!("{:-^109}", "-");
    println!("{: ^25} | {: ^25} | {: ^25} | {: ^25}", "he", "him", "his", "his");
    println!("{: ^25} | {: ^25} | {: ^25} | {: ^25}", "she", "her", "her", "hers");
    println!("{: ^25} | {: ^25} | {: ^25} | {: ^25}", "they", "them", "their", "theirs");
    println!("{:-^109}", "-");
  }
  fn create_get_pronouns(&mut self) -> Option<Pronouns> {
    let pronouns_option = 'pronouns: loop {
      self.display_pronoun_examples();
      let subject = loop {
        let mut subject_choice = String::new();
        println!("Enter your subject pronoun (e.g., he, she, they). Example: [pronoun] attended a Care Plan Meeting.");
        let subject_attempt = io::stdin().read_line(&mut subject_choice);
        match subject_attempt {
          Ok(_) => match subject_choice.trim() {
            "quit" | "QUIT" | "q" | "Q" => break 'pronouns None,
            _ => break String::from(subject_choice.trim()),
          }
          Err(e) => {
            println!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let object = loop {
        let mut object_choice = String::new();
        println!(
          "Enter your object pronoun (e.g., him, her, them). Example: Guidance counselor called ICC and left a message for [pronoun]."
        );
        let object_attempt = io::stdin().read_line(&mut object_choice);
        match object_attempt {
          Ok(_) => match object_choice.trim() {
            "quit" | "QUIT" | "q" | "Q" => break 'pronouns None,
            _ => break String::from(object_choice.trim()),
          }
          Err(e) => {
            println!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let possessive_determiner = loop {
        let mut possessive_determiner_choice = String::new();
        println!(
          "Enter your possessive determiner (e.g., his, her, their). Example: ICC used [pronoun] personal vehicle to transport youth home."
        );
        let possessive_determiner_attempt =
          io::stdin().read_line(&mut possessive_determiner_choice);
        match possessive_determiner_attempt {
          Ok(_) => match possessive_determiner_choice.trim() {
            "quit" | "QUIT" | "q" | "Q" => break 'pronouns None,
            _ => break String::from(possessive_determiner_choice.trim()),
          }
          Err(e) => {
            println!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let possessive = loop {
        let mut possessive_choice = String::new();
        println!(
          "Enter your possessive pronoun (e.g., his, hers, theirs). Example: OPT for youth provided her contact information, and ICC provider [pronoun]."
        );
        let possessive_attempt = io::stdin().read_line(&mut possessive_choice);
        match possessive_attempt {
          Ok(_) => match possessive_choice.trim() {
            "quit" | "QUIT" | "q" | "Q" => break 'pronouns None,
            _ => break String::from(possessive_choice.trim()),
          },
          Err(e) => {
            println!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let pronouns_attempt =
        self.generate_unique_new_pronouns(subject, object, possessive_determiner, possessive);
      match pronouns_attempt {
        Ok(pronouns) => break Some(pronouns),
        Err(e) => {
          println!("Pronouns could not be generated: {}.", e);
          thread::sleep(time::Duration::from_secs(1));
          break None;
        }
      }
    };
    match pronouns_option {
      Some(p) => {
        let new_pronouns = p.clone();
        self.save_pronouns(p);
        self.display_pronouns();
        println!("Pronouns records updated.");
        thread::sleep(time::Duration::from_secs(1));
        Some(new_pronouns)
      }
      None => None,
    }
  }
  pub fn generate_unique_new_pronouns(
    &mut self,
    subject: String,
    object: String,
    possessive_determiner: String,
    possessive: String,
  ) -> Result<Pronouns, String> {
    let id: u32 = self.pronouns.len() as u32 + 1;

    let new_pronouns = Pronouns::new(id, subject, object, possessive_determiner, possessive);

    let result = if self.pronouns.iter().any(|p| p == &new_pronouns) {
      Err(format!(
        "Pronouns already stored ({}).",
        new_pronouns.short_string(),
      ))
    } else {
      Ok(new_pronouns)
    };

    result
  }
  pub fn write_pronouns(&mut self) -> std::io::Result<()> {
    self.delete_duplicate_pronouns();
    let mut lines = String::from("##### pronouns #####\n");
    for p in &self.pronouns {
      lines.push_str(&p.to_string()[..]);
    }
    lines.push_str("##### pronouns #####");
    let mut file = File::create(self.pronouns_filepath.clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn save_pronouns(&mut self, pronouns: Pronouns) {
    self.pronouns.push(pronouns);
  }
  fn load_pronouns(&mut self, id: u32) -> Result<u32, String> {
    let pronouns: Option<&Pronouns> = self.pronouns.iter().find(|p| p.id == id);
    match pronouns {
      Some(p) => Ok(p.id),
      None => Err(format!("Invalid ID: {}.", id)),
    }
  }
  pub fn get_pronouns_by_id(&self, id: u32) -> Option<&Pronouns> {
    self.pronouns.iter().find(|p| p.id == id)
  }
  pub fn get_pronouns_by_id_mut(&mut self, id: u32) -> Option<&mut Pronouns> {
    self.pronouns.iter_mut().find(|p| p.id == id)
  }
  pub fn update_current_pronouns(&mut self, pronouns_id: u32) {
    self.current_user_mut().pronouns = pronouns_id;
  }
  pub fn get_duplicate_pronoun_ids(&self) -> Vec<u32> {
    let mut dups: Vec<u32> = vec![];
    for pronouns in &self.pronouns {
      if self
        .pronouns
        .iter()
        .any(|p| p == pronouns && p.id != pronouns.id)
      {
        if dups.iter().any(|d| d == &pronouns.id) {
          ()
        } else {
          dups.push(pronouns.id);
        }
      }
    }
    dups
  }
  pub fn delete_duplicate_pronouns(&mut self) {
    let mut unique_pronouns: Vec<Pronouns> = vec![];
    let dup_ids = self.get_duplicate_pronoun_ids();
    let mut unique_ids: Vec<u32> = vec![];
    for p in &self.pronouns {
      if dup_ids.iter().any(|id| id == &p.id) && unique_ids.iter().any(|id| id == &p.id) {
        ()
      } else {
        unique_ids.push(p.id);
        unique_pronouns.push(p.clone());
      }
    }
    self.pronouns = unique_pronouns;
  }
  pub fn update_pronouns_record(
    &mut self,
    pronoun_id: u32,
    pronoun_to_edit: String,
    new_pronoun: String,
  ) {
    match &pronoun_to_edit[..] {
      "subj" | "SUBJ" | "subject" | "SUBJECT" | "Subject" => {
        self
          .get_pronouns_by_id_mut(pronoun_id)
          .unwrap()
          .update_subject(new_pronoun);
      },
      "obj" | "OBJ" | "object" | "OBJECT" | "Object" => {
        self
          .get_pronouns_by_id_mut(pronoun_id)
          .unwrap()
          .update_object(new_pronoun);
      },
      "posdet"
      | "POSDET"
      | "possessive determiner"
      | "POSSESSIVE DETERMINER"
      | "Possessive Determiner"
      | "PosDet"
      | "Possessive determiner" => {
        self
          .get_pronouns_by_id_mut(pronoun_id)
          .unwrap()
          .update_possessive_determiner(new_pronoun);
      },
      "possessive" | "POSSESSIVE" | "Possessive" | "pos" | "POS" | "possess" | "Possess"
      | "POSSESS" => {
        self
          .get_pronouns_by_id_mut(pronoun_id)
          .unwrap()
          .update_possessive(new_pronoun);
      },
      _ => {
        panic!("Invalid string passed to 'fn update_pronouns_records'");
      },
    }
    if self.get_duplicate_pronoun_ids().len() > 0 {
      println!("Warning: Duplicate pronouns will be deleted on program load.");
    }
  }
  fn choose_edit_pronouns(&mut self) {
    'choose_edit_pronouns: loop {
      let final_pronouns_id;
      let final_pronoun_to_edit: String;
      let final_new_pronoun: String;
      {
        self.display_pronouns();
        let pronouns = loop {
          let input = loop {
            let mut choice = String::new();
            println!("Enter ID to edit.");
            let read_attempt = io::stdin().read_line(&mut choice);
            match read_attempt {
              Ok(_) => break choice,
              Err(e) => {
                println!("Could not read input; try again ({}).", e);
                continue;
              }
            }
          };
          match &input.trim().to_string()[..] {
            "QUIT" | "quit" | "Q" | "q" => break 'choose_edit_pronouns,
            _ => {
              let id = match input.trim().parse::<u32>() {
                Ok(num) => num,
                Err(e) => {
                  println!("Could not read input as a number; try again ({}).", e);
                  continue;
                }
              };
              match self.get_pronouns_by_id(id) {
                Some(p) => break p,
                None => {
                  println!("Unable to load pronouns with ID {}.", id);
                  continue;
                }
              }
            }
          }
        };
        final_pronouns_id = pronouns.id;
        println!("Choose the pronoun to edit (SUBJ, OBJ, POSDET, POS).");
        println!("'Q'/'QUIT' to cancel.");
        let mut pronoun_to_edit = String::new();
        let input_attempt = io::stdin().read_line(&mut pronoun_to_edit);
        match input_attempt {
          Ok(_) => (),
          Err(e) => {
            println!("Failed to read input. Please try again.");
            continue;
          }
        }
        pronoun_to_edit = pronoun_to_edit.trim().to_string();
        match &pronoun_to_edit[..] {
          "quit" | "q" | "QUIT" | "Q" | "Quit" => {
            continue;
          }
          _ => (),
        }
        final_pronoun_to_edit = pronoun_to_edit.clone();
        let new_pronoun = match &pronoun_to_edit[..] {
          "subj" | "SUBJ" | "subject" | "SUBJECT" | "Subject" => {
            println!("Enter your subject pronoun (e.g., he, she, they). Example: [pronoun] attended a Care Plan Meeting.");
            let mut subject_choice = String::new();
            let subject_attempt = io::stdin().read_line(&mut subject_choice);
            let p = match subject_attempt {
              Ok(_) => String::from(subject_choice.trim()),
              Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
              }
            };
            p
          },
          "obj" | "OBJ" | "object" | "OBJECT" | "Object" => {
            println!(
              "Enter your object pronoun (e.g., him, her, them). Example: Guidance counselor called ICC and left a message for [pronoun]."
            );
            let mut object_choice = String::new();
            let object_attempt = io::stdin().read_line(&mut object_choice);
            let p = match object_attempt {
              Ok(_) => String::from(object_choice.trim()),
              Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
              }
            };
            p
          },
          "posdet"
          | "POSDET"
          | "possessive determiner"
          | "POSSESSIVE DETERMINER"
          | "Possessive Determiner"
          | "PosDet"
          | "Possessive determiner" => {
            println!(
              "Enter your possessive determiner (e.g., his, her, their). Example: ICC used [pronoun] personal vehicle to transport youth home."
            );
            let mut posdet_choice = String::new();
            let posdet_attempt = io::stdin().read_line(&mut posdet_choice);
            let p = match posdet_attempt {
              Ok(_) => String::from(posdet_choice.trim()),
              Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
              }
            };
            p
          },
          "possessive" | "POSSESSIVE" | "Possessive" | "pos" | "POS" | "possess" | "Possess"
          | "POSSESS" => {
            println!(
              "Enter your possessive pronoun (e.g., his, hers, theirs). Example: OPT for youth provided her contact information, and ICC provider [pronoun]."
            );
            let mut possessive_choice = String::new();
            let possessive_attempt = io::stdin().read_line(&mut possessive_choice);
            let p = match possessive_attempt {
              Ok(_) => String::from(possessive_choice.trim()),
              Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
              }
            };
            p
          },
          _ => {
            println!("Invalid entry.");
            continue;
          }
        };
        final_new_pronoun = new_pronoun.clone();
      }
      self.update_pronouns_record(final_pronouns_id, final_pronoun_to_edit, final_new_pronoun);
    }
  }
  fn choose_delete_pronouns(&mut self) {
    loop {
      self.display_pronouns();
      let input = loop {
        let mut choice = String::new();
        println!("| {} | {}", "Enter ID to delete.", "QUIT / Q: cancel");
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice,
          Err(e) => {
            println!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();
    
      match input {
        "QUIT" | "quit" | "Quit" | "Q" | "q" => {
          break;
        },
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println!("Could not read input as a number; try again ({}).", e);
              continue;
            }
          };
          match self.load_pronouns(id) {
            Ok(_) => {
              self.display_view_pronoun(id);
              println!("Are you sure you want to delete this set of pronouns?");
              println!("'YES'/'Y' to confirm.");
              let mut confirm = String::new();
              let input_attempt = io::stdin().read_line(&mut confirm);
              let command = match input_attempt {
                Ok(_) => confirm.trim().to_string(),
                Err(e) => {
                  println!("Failed to read input: {}", e);
                  thread::sleep(time::Duration::from_secs(1));
                  continue;
                }
              };
              match &command[..] {
                "YES" | "yes" | "Yes" | "Y" | "y" => {
                  self.delete_pronouns(id);
                  continue;
                },
                _ => {
                  continue;
                },
              }
            },
            Err(e) => {
              println!("Unable to load pronouns with id {}: {}", input, e);
              continue;
            },
          }
        },
      }
    }
  }
  fn view_pronoun(&mut self, prns_id: u32) {
    loop {
      self.display_view_pronoun(prns_id);
      println!("| {} | {} | {}", "EDIT / E: edit (for all)", "DELETE / D: delete (for all)", "QUIT / Q: quit menu");
      let mut decision = String::new();
      let input_attempt = io::stdin().read_line(&mut decision);
      match input_attempt {
        Ok(_) => (),
        Err(e) => {
          println!("Failed to read input. Please try again.");
          continue;
        }
      }
      decision = decision.trim().to_string();
      match &decision[..] {
        "quit" | "q" | "QUIT" | "Q" | "Quit" => break,
        "delete" | "DELETE" | "Delete" => {
          self.delete_pronouns(prns_id);
          self.display_pronouns();
          thread::sleep(time::Duration::from_secs(1));
          continue;
        },
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          println!("Choose the pronoun to edit (SUBJ, OBJ, POSDET, POS).");
          println!("'Q'/'QUIT' to cancel.");
          let mut pronoun_to_edit = String::new();
          let field_input = io::stdin().read_line(&mut pronoun_to_edit);
          match field_input {
            Ok(_) => (),
            Err(e) => {
              println!("Failed to read input. Please try again.");
              continue;
            }
          }
          pronoun_to_edit = pronoun_to_edit.trim().to_string();
          match &pronoun_to_edit[..] {
            "quit" | "q" | "QUIT" | "Q" | "Quit" => {
              continue;
            },
            "subj" | "SUBJ" | "subject" | "SUBJECT" | "Subject" => {
              println!("Enter your subject pronoun (e.g., he, she, they). Example: [pronoun] attended a Care Plan Meeting.");
            },
            "obj" | "OBJ" | "object" | "OBJECT" | "Object" => {
              println!(
                "Enter your object pronoun (e.g., him, her, them). Example: Guidance counselor called ICC and left a message for [pronoun]."
              );
            },
            "posdet"
            | "POSDET"
            | "possessive determiner"
            | "POSSESSIVE DETERMINER"
            | "Possessive Determiner"
            | "PosDet"
            | "Possessive determiner" => {
              println!(
                "Enter your possessive determiner (e.g., his, her, their). Example: ICC used [pronoun] personal vehicle to transport youth home."
              );
            },
            "possessive" | "POSSESSIVE" | "Possessive" | "pos" | "POS" | "possess" | "Possess"
            | "POSSESS" => {
              println!(
                "Enter your possessive pronoun (e.g., his, hers, theirs). Example: OPT for youth provided her contact information, and ICC provider [pronoun]."
              );
            },
            _ => {
              println!("Invalid command.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
          let mut choice = String::new();
          let attempt = io::stdin().read_line(&mut choice);
          let new_prn = match attempt {
            Ok(_) => String::from(choice.trim()),
            Err(e) => {
              println!("Failed to read line: {}", e);
              continue;
            }
          };
          self.update_pronouns_record(prns_id, pronoun_to_edit, new_prn);
          self.display_pronouns();
          thread::sleep(time::Duration::from_secs(1));
        },
        _ => {
          println!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
          continue;
        },
      }
    }
  }
  fn delete_pronouns(&mut self, prns_id: u32) {
    self.pronouns.retain(|p| p.id != prns_id);
    match self.current_user_id {
      Some(id) => {
        if self.current_user().pronouns == prns_id {
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          println!("Please select new pronouns before continuing.");
          thread::sleep(time::Duration::from_secs(1));
          self.current_user_mut().pronouns = self.choose_pronouns();
          self.write_to_files();
        }
      },
      None => (),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_open_blank_files() {
    {
      let a = NoteArchive::new(
        String::from("some_random_blank_user_file_name.txt"),
        String::from("some_random_blank_client_file_name.txt"),
        String::from("some_random_blank_pronouns_file_name.txt"),
      );
      assert_eq!(a.users, vec![]);
      assert_eq!(a.clients, vec![]);
      assert_eq!(
        a.pronouns,
        vec![
          Pronouns::new(
            1,
            String::from("he"),
            String::from("him"),
            String::from("his"),
            String::from("his")
          ),
          Pronouns::new(
            2,
            String::from("she"),
            String::from("her"),
            String::from("her"),
            String::from("hers")
          ),
          Pronouns::new(
            3,
            String::from("they"),
            String::from("them"),
            String::from("their"),
            String::from("theirs")
          ),
        ]
      );
    }
    fs::remove_file("some_random_blank_user_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_client_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_pronouns_file_name.txt").unwrap();
  }
  #[test]
  fn can_load_from_files() {
    {
      let test_user = User::new(
        1,
        String::from("Bob"),
        String::from("Smith"),
        ICC,
        1,
        vec![1, 2, 3],
      );
      let test_client = Client::new(
        1,
        String::from("Harry"),
        String::from("et Tubman"),
        NaiveDate::from_ymd(2000, 1, 1),
        1,
        vec![1, 2, 3],
      );
      let test_pronouns = Pronouns::new(
        1,
        String::from("he"),
        String::from("him"),
        String::from("his"),
        String::from("his"),
      );
      let mut a1 = NoteArchive::new(
        String::from("test_load_user.txt"),
        String::from("test_load_client.txt"),
        String::from("test_load_pronouns.txt"),
      );

      a1.users = vec![test_user];
      a1.clients = vec![test_client];
      a1.pronouns = vec![test_pronouns];

      a1.write_to_files();

      a1.load_user(1).unwrap();
      assert_eq!(a1.current_user_id, Some(1));
    }
    fs::remove_file("test_load_user.txt").unwrap();
    fs::remove_file("test_load_client.txt").unwrap();
    fs::remove_file("test_load_pronouns.txt").unwrap();
  }
  #[test]
  fn creates_unique_new_instances() {
    let mut notes = NoteArchive::new_test();

    let new_user_attempt =
      notes.generate_unique_new_user(String::from("Carl"), String::from("Carlson"), ICC, 1);
    let new_client_attempt = notes.generate_unique_new_client(
      String::from("Carl"),
      String::from("Carlson"),
      NaiveDate::from_ymd(2008, 3, 4),
      1,
    );

    let new_pronouns_attempt = notes.generate_unique_new_pronouns(
      String::from("they"),
      String::from("them"),
      String::from("their"),
      String::from("theirs"),
    );

    let new_user = match new_user_attempt {
      Ok(user) => user,
      Err(_) => panic!("Failed to generate user."),
    };
    let new_client = match new_client_attempt {
      Ok(user) => user,
      Err(_) => panic!("Failed to generate client."),
    };
    let new_pronouns = match new_pronouns_attempt {
      Ok(pronouns) => pronouns,
      Err(_) => panic!("Failed to generate pronouns."),
    };

    assert_eq!(
      new_user,
      User::new(
        3,
        String::from("Carl"),
        String::from("Carlson"),
        ICC,
        1,
        vec![]
      )
    );
    assert_eq!(
      new_client,
      Client::new(
        3,
        String::from("Carl"),
        String::from("Carlson"),
        NaiveDate::from_ymd(2008, 3, 4),
        1,
        vec![]
      )
    );
    assert_eq!(
      new_pronouns,
      Pronouns::new(
        3,
        String::from("they"),
        String::from("them"),
        String::from("their"),
        String::from("theirs")
      )
    );
  }

  // pronouns

  #[test]
  fn gets_current_pronouns() {
    let mut notes = NoteArchive::new_test();

    notes.load_user(1).unwrap();

    notes.update_current_pronouns(1);

    let current_pronouns_id = notes.current_user().pronouns;

    assert_eq!(notes.get_pronouns_by_id(current_pronouns_id).unwrap().id, 1);
  }

  #[test]
  fn update_current_pronouns() {
    let mut notes = NoteArchive::new_test();

    notes.load_user(1).unwrap();

    notes.update_current_pronouns(2);
    assert_eq!(notes.current_user().pronouns, 2);

    notes.update_current_pronouns(1);
    assert_eq!(notes.current_user().pronouns, 1);
  }
}
