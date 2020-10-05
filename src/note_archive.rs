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
use crate::pronouns::*;

pub struct NoteArchive {
  pub users: Vec<User>,
  pub clients: Vec<Client>,
  pub pronouns: Vec<Pronouns>,
  pub current_user_id: Option<u32>,
  pub current_client_ids: Option<Vec<u32>>,
  pub current_client_id: Option<u32>,
  pub current_collateral_ids: Option<Vec<u32>>,
  pub current_collateral_id: Option<u32>,
}

pub const USR_FL: &str = "users.txt";
pub const CLT_FL: &str = "clients.txt";
pub const PRN_FL: &str = "pronouns.txt";

impl NoteArchive {
  pub fn run(&mut self) {
    self.load_from_files(USR_FL, CLT_FL, PRN_FL);
    let user_id = self.choose_user(USR_FL, PRN_FL);
    self.load_user(user_id, USR_FL).unwrap();

    loop {
      let client_id = self.choose_client(CLT_FL, USR_FL);
      let client = self.load_client(client_id, CLT_FL).unwrap();
    }

  }
  pub fn new() -> NoteArchive {
    NoteArchive {
      users: vec![],
      clients: vec![],
      pronouns: vec![],
      current_user_id: None,
      current_client_ids: None,
      current_client_id: None,
      current_collateral_ids: None,
      current_collateral_id: None,
    }
  }

  // users

  fn current_user(&mut self) -> &mut User {
    let user_id = match self.current_user_id {
      Some(id) => id,
      None => panic!("There is no user loaded."),
    };
    let maybe_current: Option<&mut User> = self.users.iter_mut().find(|u| u.id == user_id );
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded user id does not match any saved users."),
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
  fn load_from_files(
    &mut self,
    user_filepath: &str,
    client_filepath: &str,
    pronouns_filepath: &str
  ) {
    self.users = Self::read_users(user_filepath);
    self.clients = Self::read_clients(client_filepath);
    self.pronouns = Self::read_pronouns(pronouns_filepath);
  }
  fn load_user(&mut self, id: u32, filepath: &str) -> std::io::Result<()> {
    let current: Option<&User> = self.users.iter().find(|u| u.id == id);
    match current {
      Some(u) => {
        self.current_client_ids = Some(u.clients.clone());
        self.current_user_id = Some(u.id);
        Ok(())
      },
      None => {
        Err(Error::new(ErrorKind::Other, "Failed to read user from filepath."))
      }
    }
  }
  fn choose_user(&mut self, user_filepath: &str, pronouns_filepath: &str) -> u32 {
    self.pub_display_users();
    
    let verified_id = loop {
      let chosen_id = loop {
        let input = loop {
          let mut choice = String::new();
          println!("Enter user ID (or 'NEW' to create a new user).");
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
          let num = self.create_user_get_id(user_filepath, pronouns_filepath);
          break num
        } else {
          match input.trim().parse() {
            Ok(num) => break num,
            Err(e) => {
              println!("Could not read input as a number; try again ({}).", e);
              continue;
            },
          }
        }
      };
      match self.load_user(chosen_id, user_filepath) {
        Ok(_) => break chosen_id,
        Err(e) => {
          println!("Unable to load user with id {}: {}", chosen_id, e);
          continue;
        }
      }
    };
    verified_id
  }
  fn create_user_get_id(&mut self, user_filepath: &str, pronouns_filepath: &str) -> u32 {
    
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
      let pronouns = self.get_pronouns(pronouns_filepath);
      let user_attempt = self.generate_unique_new_user(first_name, last_name, role, pronouns, user_filepath);
      match user_attempt {
        Ok(user) => break user,
        Err(e) => {
          println!("User could not be generated: {}.", e);
          continue;
        }
      }
    };
    let id = user.id;
    self.save_user(user, user_filepath);
    id
  }
  pub fn generate_unique_new_user(
    &mut self,
    first_name: String,
    last_name: String,
    role: EmployeeRole,
    pronouns: u32,
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
      Ok(User::new(id, first_name, last_name, role, pronouns, vec![]))
    };

    result
  }
  pub fn save_user(&mut self, user: User, filepath: &str) {
    self.users.push(user);
    self.write_users(self.users.clone(), filepath).unwrap();
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
      
      let pronouns: u32 = values[4].parse().unwrap();

      let clients: Vec<u32> = match &values[5][..] {
        "" => vec![],
        _ => values[5].split("#").map(|val| val.parse().unwrap()).collect(),
      };

      let u = User::new(id, first_name, last_name, role, pronouns, clients);
      users.push(u);
    }
    users
  }

  // clients
  fn pub_display_clients(&self) {
    println!("{:-^85}", "-");
    println!("{:-^85}", " Clients ");
    println!("{:-^85}", "-");
    println!("{:-^4} | {:-^35} | {:-^40}", "ID", "NAME", "DOB");
    match self.current_client_ids {
      Some(_) => {
        for c in self.clients.iter().filter(|client| self.current_client_ids.as_ref().unwrap().iter().any(|&id| id == client.id ) ) {
          println!("{: ^4} | {: ^35} | {: <12} {: >26}", c.id, c.full_name(), c.fmt_dob(), c.fmt_date_of_birth());
        }
      },
      None => (),
    }
    println!("{:-^85}", "-");
  }
  fn load_client(&mut self, id: u32, filepath: &str) -> std::io::Result<()> {
    let current: Option<&Client> = self.clients.iter().find(|c| c.id == id);
    match current {
      Some(c) => {
        self.current_collateral_ids = Some(c.collaterals.clone());
        self.current_client_id = Some(c.id);
        Ok(())
      },
      None => {
        Err(Error::new(ErrorKind::Other, "Failed to read client from filepath."))
      }
    }
  }
  fn choose_client(&mut self, client_filepath: &str, user_filepath: &str) -> u32 {
    self.pub_display_clients();
    
    let verified_id = loop {
      let chosen_id = loop {
        let input = loop {
          let mut choice = String::new();
          println!("Enter client ID (or 'NEW' to create a new client).");
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
          let new_id = self.create_client_get_id(client_filepath);
          self.update_current_clients(new_id);
          match self.write_users(self.users.clone(), user_filepath) {
            Ok(_) => break new_id,
            Err(e) => {
              panic!("Failed to save new user to file: {}", e);
            }
          }
        } else {
          match input.trim().parse() {
            Ok(num) => break num,
            Err(e) => {
              println!("Could not read input as a number; try again ({}).", e);
              continue;
            },
          }
        }
      };
      match self.load_client(chosen_id, client_filepath) {
        Ok(_) => break chosen_id,
        Err(e) => {
          println!("Unable to load client with id {}: {}", chosen_id, e);
          continue;
        }
      }
    };
    verified_id
  }
  fn create_client_get_id(&mut self, filepath: &str) -> u32 {
    
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
          },
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
          },
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
            },
          };
          let birth_year_input = match birth_year_attempt {
            Ok(val) => val,
            Err(e) => {
              println!("Invalid birth year: {}", e);
              continue;
            },
          };
          if birth_year_input > 9999 || birth_year_input < 1000 {
            println!("Please enter a valid year.");
            continue;
          }
          break birth_year_input
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
            },
          };
          let birth_month_input = match birth_month_attempt {
            Ok(val) => val,
            Err(e) => {
              println!("Invalid birth month: {}", e);
              continue;
            },
          };
          if birth_month_input > 12 || birth_month_input < 1 {
            println!("Please enter a valid month using decimal numbers 1-12.");
            continue;
          }
          break birth_month_input
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
            },
          };
          let birth_day_input = match birth_day_attempt {
            Ok(val) => val,
            Err(e) => {
              println!("Invalid birth day: {}", e);
              continue;
            },
          };
          if birth_day_input > 31 || birth_day_input < 1 {
            println!("Please enter a valid day using decimal numbers 1-12.");
            continue;
          }
          break birth_day_input
        };

        match NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day) {
          Some(date) => break date,
          None => {
            println!("{}-{}-{} does not appear to be a valid date. Please try again.", birth_year, birth_month, birth_day);
            continue;
          }
        };

      };

      let client_attempt = self.generate_unique_new_client(first_name, last_name, dob, filepath);
      match client_attempt {
        Ok(client) => break client,
        Err(e) => {
          println!("Client could not be generated: {}.", e);
          continue;
        }
      }
    };
    let id = client.id;
    self.save_client(client, filepath);
    id
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
      lines.push_str(&c.to_string()[..]);
    }
    lines.push_str("##### clients #####");
    let mut file = File::create(filepath).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn save_client(&mut self, client: Client, filepath: &str) {
    self.clients.push(client);
    self.write_clients(self.clients.clone(), filepath).unwrap();
  }
  fn update_current_clients(&mut self, id: u32) {
    self.current_user().clients.push(id);
    self.current_client_ids = Some(self.current_user().clients.clone());
  }

  // pronouns

  pub fn read_pronouns(filepath: &str) -> Vec<Pronouns> {

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

    let mut pronouns: Vec<Pronouns> = vec![];

    for line in lines {
      let values: Vec<String> = line.unwrap().split(" | ").map(|val| val.to_string()).collect();

      let id: u32 = values[0].parse().unwrap();
      let subject = String::from(&values[1]);
      let object = String::from(&values[2]);
      let possessive_determiner = String::from(&values[3]);
      let possessive = String::from(&values[4]);

      let p = Pronouns::new(id, subject, object, possessive_determiner, possessive);
      pronouns.push(p);
    }
    pronouns
  }
  fn get_pronouns(&mut self, filepath: &str) -> u32 {
    self.pub_display_pronouns();
    
    let chosen_id = loop {
      let input = loop {
        let mut choice = String::new();
        println!("Enter pronouns ID (or 'NEW' to create new pronouns).");
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
        let pronouns = self.create_get_pronouns(filepath);
        let new_id = pronouns.id;
        self.save_pronouns(pronouns, filepath);
        break new_id
      }
      let id = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(e) => {
          println!("Could not read input as a number; try again ({}).", e);
          continue;
        },
      };
      match self.load_pronouns(id) {
        Ok(_) => break id,
        Err(e) => {
          println!("Unable to load client with id {}: {}", input, e);
          continue;
        },
      }
    };
    chosen_id
  }
  fn pub_display_pronouns(&self) {
    println!("{:-^30}", "-");
    println!("{:-^30}", " Pronouns ");
    println!("{:-^30}", "-");
    println!("{:-^4} | {:-^20}", "ID", "PRONOUNS");
    for p in &self.pronouns {
      println!("{: ^4} | {: ^20}", p.id, p.short_string());
    }
    println!("{:-^30}", "-");
  }
  fn create_get_pronouns(&mut self, filepath: &str) -> Pronouns {
    
    let pronouns = loop {
      let subject = loop {
        let mut subject_choice = String::new();
        println!("Enter your subject pronoun ('he' in 'he/him/his/his', 'she' in 'she/her/her/hers', or 'they' in 'they/them/their/theirs').");
        println!("Example: [pronoun] attended a Care Plan Meeting.");
        let subject_attempt = io::stdin().read_line(&mut subject_choice);
        match subject_attempt {
          Ok(_) => break String::from(subject_choice.trim()),
          Err(e) => {
            println!("Failed to read line: {}", e);
            continue;
          },
        };
      };
      let object = loop {
        let mut object_choice = String::new();
        println!("Enter your object pronoun ('him' in 'he/him/his/his', 'her' in 'she/her/her/hers', or 'them' in 'they/them/their/theirs').");
        println!("Example: Guidance counselor called ICC and left a message for [pronoun].");
        let object_attempt = io::stdin().read_line(&mut object_choice);
        match object_attempt {
          Ok(_) => break String::from(object_choice.trim()),
          Err(e) => {
            println!("Failed to read line: {}", e);
            continue;
          },
        };
      };
      let possessive_determiner = loop {
        let mut possessive_determiner_choice = String::new();
        println!("Enter your possessive determiner ('his' in 'he/him/his/his', 'her' in 'she/her/her/hers', or 'their' in 'they/them/their/theirs').");
        println!("Example: ICC used [pronoun] personal vehicle to transport youth home.");
        let possessive_determiner_attempt = io::stdin().read_line(&mut possessive_determiner_choice);
        match possessive_determiner_attempt {
          Ok(_) => break String::from(possessive_determiner_choice.trim()),
          Err(e) => {
            println!("Failed to read line: {}", e);
            continue;
          },
        };
      };
      let possessive = loop {
        let mut possessive_choice = String::new();
        println!("Enter your possessive pronoun ('his' in 'he/him/his/his', 'hers' in 'she/her/her/hers', or 'theirs' in 'they/them/their/theirs').");
        println!("Example: OPT for youth provider her contact information, and ICC provider [pronoun].");
        let possessive_attempt = io::stdin().read_line(&mut possessive_choice);
        match possessive_attempt {
          Ok(_) => break String::from(possessive_choice.trim()),
          Err(e) => {
            println!("Failed to read line: {}", e);
            continue;
          },
        };
      };
      let pronouns_attempt = self.generate_unique_new_pronouns(subject, object, possessive_determiner, possessive, filepath);
      match pronouns_attempt {
        Ok(pronouns) => break pronouns,
        Err(e) => {
          println!("Pronouns could not be generated: {}.", e);
          continue;
        }
      }
    };
    let new_pronouns = pronouns.clone();
    self.save_pronouns(pronouns, filepath);
    new_pronouns
  }
  pub fn generate_unique_new_pronouns(
    &mut self,
    subject: String,
    object: String,
    possessive_determiner: String,
    possessive: String,
    filepath: &str,
  ) -> Result<Pronouns, String> {
    let saved_pronouns = NoteArchive::read_pronouns(&filepath);
    let id: u32 = saved_pronouns.len() as u32 + 1;

    let new_pronouns = Pronouns::new(id, subject, object, possessive_determiner, possessive);

    let result = if self.pronouns.iter().any(|p| p == &new_pronouns ) {
      Err(format!(
        "Pronouns already stored ({}).",
        new_pronouns.short_string(),
      ))
    } else {
      Ok(new_pronouns)
    };

    result
  }
  pub fn write_pronouns(&mut self, pronouns: Vec<Pronouns>, filepath: &str) -> std::io::Result<()> {
    let mut lines = String::from("##### pronouns #####\n");
    for p in pronouns {
      lines.push_str(&p.to_string()[..]);
    }
    lines.push_str("##### pronouns #####");
    let mut file = File::create(filepath).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn save_pronouns(&mut self, pronouns: Pronouns, filepath: &str) {
    self.pronouns.push(pronouns);
    self.write_pronouns(self.pronouns.clone(), filepath).unwrap();
  }
  fn load_pronouns(&mut self, id: u32) -> Result<u32, String> {
    let pronouns: Option<&Pronouns> = self.pronouns.iter().find(|c| c.id == id);
    match pronouns {
      Some(p) => Ok(p.id),
      None => Err(format!("Invalid ID: {}.", id)),
    }
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
  fn can_open_blank_pronouns() {
    {
      let pronouns = NoteArchive::read_pronouns("some_random_pronoun_file_name.txt");
      assert_eq!(pronouns, vec![]);
    }
    fs::remove_file("some_random_pronoun_file_name.txt").unwrap();
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
        1,
        vec![1, 2, 3]
      );
      let test_user_2 = User::new(
        2,
        String::from("Gerald"),
        String::from("Ford"),
        FP,
        1,
        vec![1, 2, 3]
      );
      a1.write_users(vec![test_user_1, test_user_2], "test_load_user.txt").unwrap();

      a1.load_from_files(
        "test_load_user.txt",
        "placehold_load_user_client_filepath.txt",
        "placeholder_load_user_pronouns_filepath.txt",
      );

      a1.load_user(2, "test_load_user.txt").unwrap();
      assert_eq!(a1.current_user_id, Some(2));
    }
    fs::remove_file("test_load_user.txt").unwrap();
    fs::remove_file("placehold_load_user_client_filepath.txt").unwrap();
    fs::remove_file("placeholder_load_user_pronouns_filepath.txt").unwrap();
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
        1,
        vec![1, 2, 3]
      );
      let test_user_2 = User::new(
        2,
        String::from("Gerald"),
        String::from("Ford"),
        FP,
        2,
        vec![7, 8, 9]
      );
      a1.write_users(vec![test_user_1, test_user_2], "test_load_clients_from_user.txt").unwrap();

      a1.load_from_files(
        "test_load_clients_from_user.txt",
        "placehold_load_client_client_filepath.txt",
        "placehold_load_client_pronouns_filepath.txt",
      );

      a1.load_user(2, "test_load_clients_from_user.txt").unwrap();
      assert_eq!(a1.current_client_ids, Some(vec![7, 8, 9]));
    }
    fs::remove_file("test_load_clients_from_user.txt").unwrap();
    fs::remove_file("placehold_load_client_client_filepath.txt").unwrap();
    fs::remove_file("placehold_load_client_pronouns_filepath.txt").unwrap();
  }
  #[test]
  fn can_load_pronouns() {
    {
      let mut a1 = NoteArchive::new();
      let test_pronouns_1 = Pronouns::new(
        1,
        String::from("he"),
        String::from("him"),
        String::from("his"),
        String::from("his"),
      );
      let test_pronouns_2 = Pronouns::new(
        2,
        String::from("she"),
        String::from("her"),
        String::from("her"),
        String::from("hers"),
      );

      let t1 = test_pronouns_1.clone();
      let t2 = test_pronouns_2.clone();

      a1.write_pronouns(vec![test_pronouns_1, test_pronouns_2], "test_load_pronouns.txt").unwrap();

      a1.load_from_files(
        "placeholder_load_pronouns_user_filepath.txt",
        "placehold_load_pronouns_client_filepath.txt",
        "test_load_pronouns.txt",
      );

      assert_eq!(a1.pronouns, vec![t1, t2]);
    }
    fs::remove_file("placeholder_load_pronouns_user_filepath.txt").unwrap();
    fs::remove_file("placehold_load_pronouns_client_filepath.txt").unwrap();
    fs::remove_file("test_load_pronouns.txt").unwrap();
  }

}