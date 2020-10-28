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
use crate::collateral::*;
use crate::pronouns::*;
use crate::user::*;
use crate::EmployeeRole::{FP, ICC};
use crate::SupportType::{Natural, Formal};
use crate::utils::*;

const FAMILY_ROLES: [&'static str; 104] = [
  "family", "parent", "nuclear family", "nuclear family member", "family member", "immediate family", "spouse", "husband", "wife",
  "father", "mother", "step-father", "step father", "stepfather", "step-mother", "step mother", "stepmother", "step-mother", "legal guardian",
  "child", "son", "daughter", "step-son", "step son", "stepson", "step-daughter", "stepdaughter", "step daughter", "sibling", "brother", "sister",
  "extended family", "grandparent", "grandfather", "grandmother", "grandson", "granddaughter", "uncle", "aunt", "cousin", "nephew", "niece",
  "family-in-law", "family in law", "father-in-law", "father in law", "mother-in-law", "mother in law", "brother in law", "sister in law", "kin",
  "kinship caregiver", "family member", "partner", "adoptive mother", "adoptive father", "birth mother", "birth father", "guardian", "adopted child",
  "adopted son", "adopted daughter", "adoptive sister", "adoptive sibling", "step sibling", "step-sibling", "stepsibling", "adoptive brother",
  "adopted brother", "adopted sister", "foster sister", "foster brother", "foster mother", "foster mom", "foster dad", "foster father",
  "former foster mom", "former foster mother", "former foster father", "former foster dad", "former foster brother", "former foster sister",
  "grandma", "grandpa", "first cousin", "second cousin", "grandchild", "adoptive grandchild", "adopted grandchild", "adopted grandson",
  "adoptive grandson", "adoptive granddaughter", "adopted granddaughter", "adoptive uncle", "adoptive aunt", "half brother", "half sister",
  "biological mother", "biological father", "biological mom", "biological dad", "adoptive first couse", "adoptive second cousin", "adoptive cousin"
];

const FORMAL_ROLES: [&'static str; 40] = [
  "intensive care coordinator", "icc", "family partner", "fp", "in home therapist", "in-home therapist", "iht",
  "in home behavioral therapist", "in-home behavioral therapist", "ihbt", "therapeutic mentor", "tm", "ot", "occupational therapist",
  "psychiatrist", "outpatient therapist", "opt", "guidance counselor", "school social worker", "social worker", "dcf worker",
  "guardian ad litem", "asentria worker", "mentor", "therapist", "behavioral therapist", "parole officer", "primary care physician",
  "pcp", "therapeutic training and support", "therapeutic training & support", "tt&s", "tt and s", "dmh worker", "clinician",
  "teacher", "special education teacher", "school guidance counselor", "lifeset worker", "lifeset mentor"
];

const INDIRECT_ROLES: [&'static str; 23] = [
  "director", "clinical director", "principal", "assistant director", "assistant clinical director", "director of special education",
  "clinical supervisor", "director of social and emotional learning", "assistant director of special education",
  "crisis support worker", "crisis clinician", "crisis response clinician", "mci worker", "mci clinician", "mobile crisis intervention",
  "emergency services clinician", "emergency services", "crisis assessment clinician",
  "mobile crisis intervention clinician", "mobile crisis intervention worker", "crisis support clinician", "crisis response worker",
  "mobile crisis clinician"
];

pub struct NoteArchive {
  pub users: Vec<User>,
  pub clients: Vec<Client>,
  pub collaterals: Vec<Collateral>,
  pub pronouns: Vec<Pronouns>,
  pub current_user_id: Option<u32>,
  pub current_client_ids: Option<Vec<u32>>,
  pub current_client_id: Option<u32>,
  pub current_collateral_ids: Option<Vec<u32>>,
  pub current_collateral_id: Option<u32>,
  user_filepath: String,
  client_filepath: String,
  collateral_filepath: String,
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
    collateral_filepath: String,
    pronouns_filepath: String,
  ) -> NoteArchive {
    let mut a = NoteArchive {
      users: Self::read_users(&user_filepath),
      clients: Self::read_clients(&client_filepath),
      collaterals: Self::read_collaterals(&collateral_filepath),
      pronouns: vec![],
      current_user_id: None,
      current_client_ids: None,
      current_client_id: None,
      current_collateral_ids: None,
      current_collateral_id: None,
      user_filepath,
      client_filepath,
      collateral_filepath,
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
    let collateral_1 = Collateral::new(
      1,
      String::from("Jerry"),
      String::from("Smith"),
      String::from("TM"),
      Some(String::from("Kaleidoscope Family Solutions")),
      2,
      Formal,
      false
    );
    let collateral_2 = Collateral::new(
      2,
      String::from("Barry"),
      String::from("Plith"),
      String::from("OPT"),
      Some(String::from("Family Solutions, Inc.")),
      1,
      Formal,
      false
    );
    let collaterals = vec![collateral_1, collateral_2];
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
      String::from("test_collateral.txt"),
      String::from("test_pronouns.txt"),
    );

    notes.users = users;
    notes.clients = clients;
    notes.collaterals = collaterals;
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
    if fs::metadata("test_collateral.txt").is_ok() {
      fs::remove_file("test_collateral.txt").unwrap();
    }
    if fs::metadata("test_pronouns.txt").is_ok() {
      fs::remove_file("test_pronouns.txt").unwrap();
    }
  }
  pub fn write_to_files(&mut self) {
    self.write_users().unwrap();
    self.write_clients().unwrap();
    self.write_collaterals().unwrap();
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
      " COL / CO ", " View/edit collateral records "
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
        "collateral" | "co" | "COLLATERAL" | "CO" | "Collateral" | "Co" | "collat" | "COLLAT" | "Collat" | "COL" | "col" | "Col" => {
          self.choose_collaterals();
        },
        "user" | "u" | "USER" | "U" | "User" => {
          self.choose_user();
        },
        "PRNS" | "Prns" | "prns" | "P" | "p" | "pronouns" | "Pronouns" | "PRONOUNS" => {
          let chosen_pronoun = self.choose_pronouns_option();
          match chosen_pronoun {
            Some(prn) => {
              self.view_pronoun(prn);
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
    let pos = self.users.binary_search_by(|u| u.id.cmp(&user.id) ).unwrap_or_else(|e| e);
    self.users.insert(pos, user);
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
    users.sort_by(|a, b| a.id.cmp(&b.id));
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
      println!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
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
        _ => {
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
      None => panic!("The loaded client id does not match any saved clients."),
    }
  }
  fn display_clients(&self) {
    let mut heading = String::from(" ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", heading);
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
    println!("| {} | {} | {} | {} | {}", "Choose client by ID.", "NEW / N: new client", "ADD / A: Add from other user", "EDIT / E: edit records", "QUIT / Q: quit menu");
  }
  fn display_edit_clients(&self) {
    let mut heading = String::from(" Edit ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", heading);
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
    println!("| {} | {}", "Choose client by ID.", "QUIT / Q: quit menu");
  }
  fn display_add_client(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", " Clients ");
    println!("{:-^96}", "-");
    println!("{:-^10} | {:-^40} | {:-^40}", "ID", "NAME", "DOB");
    match &self.current_client_ids {
      Some(c_ids) => {
        for c in self.clients.iter().filter(|client| !self.current_client_ids.as_ref().unwrap().iter().any(|&id| id == client.id)) {
          println!(
            "{: ^10} | {: ^40} | {: <12} {: >26}",
            c.id,
            c.full_name(),
            c.fmt_dob(),
            c.fmt_date_of_birth()
          );
        }
      },
      None => {
        for c in self.clients.iter() {
          println!(
            "{: ^10} | {: ^40} | {: <12} {: >26}",
            c.id,
            c.full_name(),
            c.fmt_dob(),
            c.fmt_date_of_birth()
          );
        }
      },
    }
    println!("{:-^96}", "-");
    println!("| {} | {}", "Enter ID to add client.", "QUIT / Q: quit menu");
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
  fn add_client(&mut self) {
    loop {
      self.display_add_client();
      let input = loop {
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
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        "NEW" | "new" | "New" | "n" | "N" => {
          let new_id = self.create_client_get_id();
          self.update_current_clients(new_id);
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.clients.iter()
              .filter(|c| !self.current_client_ids.as_ref().unwrap().iter().any(|c_id| &c.id == c_id ))
              .map(|c| c.id )
              .any(|id| id == num) {
              println!("Please select from the available choices.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            } else {
              match self.load_client(num) {
                Ok(_) => {
                  self.current_user_mut().clients.push(num);
                  self.update_current_clients(num);
                  break;
                }
                Err(e) => {
                  println!("Unable to load client with id {}: {}", num, e);
                  continue;
                }
              }
            }
          },
          Err(e) => {
            println!("Failed to read input as a number.");
            continue;
          }
        }
      }
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
        "ADD" | "add" | "Add" | "a" | "A" => {
          self.add_client();
          continue;
        },
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          self.choose_edit_clients();
          continue;
        },
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_client_ids
              .as_ref()
              .unwrap()
              .iter()
              .any(|&id| id == num) {
                println!("Please choose from among the listed clients, or add a client from another user.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_client(num) {
              Ok(_) => self.choose_client(),
              Err(e) => {
                println!("Unable to load client with id {}: {}", num, e);
                thread::sleep(time::Duration::from_secs(1));
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
  fn choose_edit_clients(&mut self) {
    loop {
      let input = loop {
        self.display_edit_clients();
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
        "QUIT" | "quit" | "Quit" | "Q" | "q" => {
          break;
        },
        _ => {
          match input.parse() {
            Ok(num) => {
              if !self.current_client_ids.as_ref().unwrap().iter().any(|c_id| c_id == &num) {
                println!("Please choose from among the listed IDs.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
              match self.load_client(num) {
                Ok(_) => self.choose_client(),
                Err(e) => {
                  println!("Failed to load client with ID {}: {}", num, e);
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              }
            },
            Err(e) => {
              println!("Failed to read input '{}' as a number: {}", input, e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        }
      }
    }
  }
  fn display_specify_clients(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", " Choose client for collateral ");
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
    println!("| {} | {} | {} | {}", "Choose client by ID.", "NEW / N: new client", "ADD / A: Add from other user", "QUIT / Q: quit menu");
  }
  fn specify_client(&mut self) -> u32 {
    let id: u32 = loop {
      let input = loop {
        self.display_specify_clients();
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
        "ADD" | "add" | "Add" | "a" | "A" => {
          self.add_client();
          continue;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_client_ids
              .as_ref()
              .unwrap()
              .iter()
              .any(|id| id == &num) {
                println!("Please choose from among the listed clients.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              } else {
                match self.load_client(num) {
                  Ok(_) => {
                    self.current_client_id = None;
                    break num
                  }
                  Err(e) => {
                    println!("Unable to load client with id {}: {}", num, e);
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                  }
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
    };
    id
  }
  fn choose_client(&mut self) {
    loop {
      self.display_client();
      println!("| {} | {} | {} | {}", "EDIT / E: edit client", "DELETE: delete client", "COLLATERAL / COL: view/edic client collaterals", "QUIT / Q: quit menu");
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
        "COLLATERAL" | "collateral" | "Collateral" | "COLLAT" | "collat" | "Collat" | "COL" | "col" | "Col" => {
          self.choose_client_collaterals();
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
    clients.sort_by(|a, b| a.id.cmp(&b.id));
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
    let pos = self.clients.binary_search_by(|c| c.id.cmp(&client.id) ).unwrap_or_else(|e| e);
    self.clients.insert(pos, client);
  }
  fn update_current_clients(&mut self, id: u32) {
    self.current_user_mut().clients.push(id);
    self.current_client_ids = Some(self.current_user_mut().clients.clone());
  }
  fn choose_edit_client(&mut self) {
    loop {
      self.display_client();
      println!("| {} | {} | {} | {}", "FIRST / F: edit first name", "LAST / L: edit surname", "PRNS / P: edit pronouns", "QUIT / Q: quit menu");
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
      println!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
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
          break;
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
  fn get_client_by_id(&self, id: u32) -> &Client {
    self.clients.iter().find(|c| c.id == id).unwrap()
  }
  fn get_client_by_id_mut(&mut self, id: u32) -> &mut Client {
    self.clients.iter_mut().find(|c| c.id == id).unwrap()
  }

  // collaterals
  fn current_collateral_mut(&mut self) -> &mut Collateral {
    let collateral_id = match self.current_collateral_id {
      Some(id) => id,
      None => panic!("There is no current collateral selected."),
    };
    let maybe_current: Option<&mut Collateral> = self.collaterals.iter_mut().find(|c| c.id == collateral_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded collateral ID does not match any saved collaterals."),
    }
  }
  fn current_collateral(&self) -> &Collateral {
    let collateral_id = match self.current_collateral_id {
      Some(id) => id,
      None => panic!("There is no current collateral selected."),
    };
    let maybe_current: Option<&Collateral> = self.collaterals.iter().find(|c| c.id == collateral_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded collateral id does not match any saved collaterals."),
    }
  }
  fn current_user_collaterals(&self) -> Vec<&Collateral> {
    self.collaterals
      .iter()
      .filter(|co|
        self.current_user().clients
          .iter()
          .any(|client_id|
            self.get_client_by_id(*client_id).collaterals
              .iter()
              .any(|collateral_id|
                co.id == *collateral_id) 
          )
        )
      .collect()
  }
  fn current_user_collateral_ids(&self) -> Vec<u32> {
    let mut co_ids: Vec<u32> = vec![];
    for client_id in &self.current_user().clients {
      let cols = &self.get_client_by_id(*client_id).collaterals;
      for c in cols {
        if !co_ids.iter().any(|cid| cid == c) {
          co_ids.push(*c);
        }
      }
    }
    co_ids
  }
  fn collateral_clients_string(&self, co_id: u32) -> String {
    let clients = self.get_clients_by_collateral_id(co_id);

    if clients.len() > 1 {
      let mut display_clients_string = clients
        .iter()
        .enumerate()
        .filter(|&(i, _)| i < clients.len()-1 )
        .map(|(_, c)| c.full_name() )
        .collect::<Vec<String>>()
        .join(", ");
  
      display_clients_string.push_str(" and ");
  
      display_clients_string.push_str(&clients[clients.len()-1].full_name()[..]);
  
      display_clients_string
    } else {
      clients[0].full_name()
    }

  }
  fn display_client_collaterals(&self) {
    let mut heading = self.current_client().first_name.clone();
    heading.push_str(" ");
    heading.push_str(&self.current_client().last_name);
    heading.push_str("'s Collaterals");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^113}", "-");
    println!("{:-^113}", heading);
    println!("{:-^113}", "-");
    println!("{:-^10} | {:-<100}", "ID", "Info");
    match self.current_collateral_ids {
      Some(_) => {
        for c in self.collaterals.iter().filter(|collateral| {
          self
            .current_collateral_ids
            .as_ref()
            .unwrap()
            .iter()
            .any(|&id| id == collateral.id)
        }) {
          println!(
            "{: ^10} | {: <100}",
            c.id,
            c.full_name_and_title(),
          );
        }
      }
      None => (),
    }
    println!("{:-^113}", "-");
    println!("| {} | {} | {} | {}", "Enter ID to choose collateral.", "NEW / N: new collateral", "EDIT / E: edit", "QUIT / Q: quit menu");
  }
  fn display_edit_client_collaterals(&self) {
    let mut heading = self.current_client().first_name.clone();
    heading.push_str(" Edit ");
    heading.push_str(&self.current_client().last_name);
    heading.push_str("'s Collateral records");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^113}", "-");
    println!("{:-^113}", heading);
    println!("{:-^113}", "-");
    println!("{:-^10} | {:-<100}", "ID", "Info");
    match self.current_collateral_ids {
      Some(_) => {
        for c in self.collaterals.iter().filter(|collateral| {
          self
            .current_collateral_ids
            .as_ref()
            .unwrap()
            .iter()
            .any(|&id| id == collateral.id)
        }) {
          println!(
            "{: ^10} | {: <100}",
            c.id,
            c.full_name_and_title(),
          );
        }
      }
      None => (),
    }
    println!("{:-^113}", "-");
    println!("| {} | {}", "Enter ID of collateral record to edit.", "QUIT / Q: quit menu");
  }
  fn display_user_collaterals(&self) {
    let current = self.current_user();
    let heading = format!(
      "{} {}, {} - All collateral records",
      current.first_name,
      current.last_name,  
      current.role,
    );

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^146}", "-");
    println!("{:-^146}", heading);
    println!("{:-^146}", "-");
    println!("{:-^10} | {:-<100} | {:-<30}", " ID ", "Info ", " Clients ");

    for co in self.current_user_collaterals() {

      println!(
        "{: ^10} | {: <100} | {: <30}",
        co.id,
        co.full_name_and_title(),
        self.collateral_clients_string(co.id),
      );
    }
    println!("{:-^146}", "-");
    println!("| {} | {} | {} | {}", "Enter ID to choose collateral.", "EDIT / E: edit", "NEW / N: new collateral", "QUIT / Q: quit menu");
  }
  fn display_edit_user_collaterals(&self) {
    let current = self.current_user();
    let heading = format!(
      "{} {}, {} - All collateral records",
      current.first_name,  
      current.last_name,  
      current.role,
    );

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^146}", "-");
    println!("{:-^146}", heading);
    println!("{:-^146}", "-");
    println!("{:-^10} | {:-<100} | {:-<30}", " ID ", "Info ", " Clients ");

    for co in self.current_user_collaterals() {

      println!(
        "{: ^10} | {: <100} | {: <30}",
        co.id,
        co.full_name_and_title(),
        self.collateral_clients_string(co.id),
      );
    }
    println!("{:-^146}", "-");
    println!("| {} | {}", "Enter ID of collateral to edit.", "QUIT / Q: quit menu");
  }
  fn display_collateral(&self) {
    let current = self.current_collateral();

    let pronouns_id = current.pronouns;
    let pronouns_option = self.get_pronouns_by_id(pronouns_id);
    let display_pronouns = match pronouns_option {
      Some(p) => p.short_string(),
      None => String::from("-----"),
    };

    let inst_option = &current.institution;
    let display_inst = match inst_option {
      Some(i) => i.to_string(),
      None => String::from("n/a"),
    };

    let display_type = match current.support_type {
      Natural => "Y",
      Formal => "N",
    };
    let display_indirect = match current.indirect_support {
      true => "N",
      false => "Y",
    };

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^178}", "-");
    println!("{:-^178}", " View collateral record ");
    println!("{:-^178}", "-");
    println!(
      "{:-^162} | {:-^13}",
      "-", "Support type",
    );
    println!(
      "{:-^20} | {:-^20} | {:-^30} | {:-^30} | {:-^50} | {:-^5} | {:-^5}",
      " First name ", " Last name ", " Pronouns ", " Role/Title ", " Institution ", " Nat ", " Dir "
    );
    println!(
      "{: ^20} | {: ^20} | {: ^30} | {: ^30} | {: ^50} | {:-^5} | {:-^5}",
      current.first_name,
      current.last_name,
      display_pronouns,
      current.title,
      display_inst,
      display_type,
      display_indirect,
    );
    println!("{:-^178}", "-");
    println!("| {} | {} | {}", "EDIT / E: edit collateral", "DELETE: delete collateral", "QUIT / Q: quit menu");
    println!("{:-^178}", "-");
  }
  fn display_edit_collateral(&self) {
    let current = self.current_collateral();

    let pronouns_id = current.pronouns;
    let pronouns_option = self.get_pronouns_by_id(pronouns_id);
    let display_pronouns = match pronouns_option {
      Some(p) => p.short_string(),
      None => String::from("-----"),
    };

    let inst_option = &current.institution;
    let display_inst = match inst_option {
      Some(i) => i.to_string(),
      None => String::from("n/a"),
    };

    let (display_type, opposite_type_command) = match current.support_type {
      Natural => ("Y", "FORMAL: Change to formal support",),
      Formal => ("N", "NATURAL: Change to natural support"),
    };
    let (display_indirect, opposite_indirect_command) = match current.indirect_support {
      true => ("N", "DIRECT: Change to direct support (e.g., 'for youth')"),
      false => ("Y", "INDIRECT: Change to indirect support (e.g., not 'for youth')"),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^178}", "-");
    println!("{:-^178}", " Edit collateral record ");
    println!("{:-^178}", "-");
    println!(
      "{:-^162} | {:-^13}",
      "-", "Support type",
    );
    println!(
      "{:-^20} | {:-^20} | {:-^30} | {:-^30} | {:-^50} | {:-^5} | {:-^5}",
      " First name ", " Last name ", " Pronouns ", " Role/Title ", " Institution ", " Nat ", " Dir "
    );
    println!(
      "{: ^20} | {: ^20} | {: ^30} | {: ^30} | {: ^50} | {:-^5} | {:-^5}",
      current.first_name,
      current.last_name,
      display_pronouns,
      current.title,
      display_inst,
      display_type,
      display_indirect,
    );
    println!("{:-^178}", "-");
    println!(
      "| {} | {} | {} | {} | {} | {}",
      "FIRST / F: edit first name",
      "LAST / L: edit surname",
      "TITLE / T: edit title/role",
      "INST / I: edit institution",
      "PRNS / P: edit pronouns",
      "QUIT / Q: quit menu");
    println!(
      "| {} | {}",
      opposite_type_command,
      opposite_indirect_command,
    );
    println!("{:-^178}", "-");
  }
  fn load_collateral(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&Collateral> = self.collaterals.iter().find(|c| c.id == id);
    match current {
      Some(c) => {
        self.current_collateral_id = Some(c.id);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to find collateral.",
      )),
    }
  }
  fn choose_client_collaterals(&mut self) {
    loop {
      let input = loop {
        self.display_client_collaterals();
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
          let new_id = self.create_collateral_get_id();
          self.update_current_collaterals(new_id);
          continue;
        },
        "EDIT" | "edit" | "Edit" | "E" | "e" => {
          self.choose_edit_client_collaterals();
          continue;
        },
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_collateral_ids.as_ref().unwrap().iter().any(|n| n == &num) {
              println!("Please select one of the listed IDs.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
            match self.load_collateral(num) {
              Ok(_) => self.choose_collateral(),
              Err(e) => {
                println!("Unable to load collateral with id {}: {}", num, e);
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
  fn choose_edit_client_collaterals(&mut self) {
    loop {
      let input = loop {
        self.display_edit_client_collaterals();
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
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_collateral_ids.as_ref().unwrap().iter().any(|n| n == &num) {
              println!("Please select one of the listed IDs.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
            match self.load_collateral(num) {
              Ok(_) => self.choose_edit_collateral(),
              Err(e) => {
                println!("Unable to load collateral with id {}: {}", num, e);
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
  fn choose_collaterals(&mut self) {
    loop {
      let input = loop {
        self.display_user_collaterals();
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
          let new_id = self.create_collateral_get_id();
          continue;
        },
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          self.choose_edit_collaterals();
          continue;
        },
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_collateral_ids().iter().any(|n| n == &num) {
            println!("Please select one of the listed IDs.");
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
            match self.load_collateral(num) {
              Ok(_) => self.choose_collateral(),
              Err(e) => {
                println!("Unable to load collateral with id {}: {}", num, e);
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
  fn choose_edit_collaterals(&mut self) {
    loop {
      let input = loop {
        self.display_edit_user_collaterals();
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
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_collateral_ids().iter().any(|n| n == &num) {
            println!("Please select one of the listed IDs.");
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
            match self.load_collateral(num) {
              Ok(_) => self.choose_edit_collateral(),
              Err(e) => {
                println!("Unable to load collateral with id {}: {}", num, e);
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
  fn choose_collateral(&mut self) {
    loop {
      self.display_collateral();
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
          self.choose_delete_collateral();
        }
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          self.choose_edit_collateral();
        }
        _ => {
          println!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
        }
      }
    }
  }
  fn create_collateral_get_id(&mut self) -> u32 {
    let collateral = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        println!("Enter collateral's first name.");
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
        println!("Enter collateral's last name.");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println!("Invalid last name: {}", e);
            continue;
          }
        };
      };
      let title = loop {
        let mut title_choice = String::new();
        println!("Enter collateral's role/title.");
        let title_attempt = io::stdin().read_line(&mut title_choice);
        match title_attempt {
          Ok(_) => break String::from(title_choice.trim()).to_lowercase(),
          Err(e) => {
            println!("Invalid title: {}", e);
            continue;
          }
        };
      };

      let pronouns = self.choose_pronouns();

      let (support_type, indirect_support, institution) = if FAMILY_ROLES.iter().any(|role| role == &title) {
        (Natural, false, None)
      } else if FORMAL_ROLES.iter().any(|role| role == &title ) {
        let institution = loop {
          let mut institution_choice = String::new();
          println!("Enter collateral's institution.");
          let institution_attempt = io::stdin().read_line(&mut institution_choice);
          match institution_attempt {
            Ok(_) => break String::from(institution_choice.trim()),
            Err(e) => {
              println!("Invalid institution: {}", e);
              continue;
            }
          }
        };
        (Formal, false, Some(institution))
      } else if INDIRECT_ROLES.iter().any(|role| role == &title ) {
        let institution = loop {
          let mut institution_choice = String::new();
          println!("Enter collateral's institution.");
          let institution_attempt = io::stdin().read_line(&mut institution_choice);
          match institution_attempt {
            Ok(_) => break String::from(institution_choice.trim()),
            Err(e) => {
              println!("Invalid institution: {}", e);
              continue;
            }
          }
        };
        (Formal, true, Some(institution))
      } else {
        loop {
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          let mut support_type_choice = String::new();
          println!("Natural or Formal support?");
          println!("NATURAL / N | FORMAL / F");
          let support_type_attempt = io::stdin().read_line(&mut support_type_choice);
          let s = match support_type_attempt {
            Ok(_) => match support_type_choice.trim() {
              "Natural" | "natural" | "NATURAL" | "NAT" | "Nat" | "nat" | "N" | "n" => Natural,
              "Formal" | "formal" | "FORMAL" | "FORM" | "Form" | "form" | "F" | "f" => Formal,
              _ => {
                println!("Please choose NATURAL or FORMAL.");
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
            Err(e) => {
              println!("Failed to read input.");
              continue;
            }
          };
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          let mut indirect_choice = String::new();
          println!("Does this collateral work directly with the client?");
          println!("YES / Y | NO / N");
          let indirect_attempt = io::stdin().read_line(&mut indirect_choice);
          let i = match indirect_attempt {
            Ok(_) => match indirect_choice.trim() {
              "YES" | "yes" | "Y" | "y" => false,
              "NO" | "no" | "N" | "n" => true,
              _ => {
                println!("Please choose YES or NO.");
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
            Err(e) => {
              println!("Failed to read input.");
              continue;
            }
          };
          let institution = loop {
            if s == Formal {
              let mut institution_choice = String::new();
              println!("Enter collateral's institution.");
              let institution_attempt = io::stdin().read_line(&mut institution_choice);
              match institution_attempt {
                Ok(_) => break Some(String::from(institution_choice.trim())),
                Err(e) => {
                  println!("Invalid institution: {}", e);
                  continue;
                }
              }
            } else {
              break None
            }
          };
          break (s, i, institution)
        }
      };

      let collateral_attempt = self.generate_unique_new_collateral(
        first_name,
        last_name,
        title,
        institution,
        pronouns,
        support_type,
        indirect_support,
      );
      match collateral_attempt {
        Ok(collateral) => break collateral,
        Err(e) => {
          println!("Collateral could not be generated: {}.", e);
          continue;
        }
      }
    };
    let id = collateral.id;
    match self.current_client_id {
      Some(_) => self.current_client_mut().collaterals.push(id),
      None => {
        let c_id = self.specify_client();
        self.get_client_by_id_mut(c_id).collaterals.push(id);
      }
    }
    self.save_collateral(collateral);
    id
  }
  pub fn generate_unique_new_collateral(
    &mut self,
    first_name: String,
    last_name: String,
    title: String,
    institution: Option<String>,
    pronouns: u32,
    support_type: SupportType,
    indirect_support: bool,
  ) -> Result<Collateral, String> {
    let id: u32 = self.collaterals.len() as u32 + 1;

    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .collaterals
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.title[..], &c.institution))
      .collect();

    let result = if names_and_roles
      .iter()
      .any(|(f, l, t, i)| f == &first_name && l == &last_name && t == &title && i == &&institution)
    {
      match institution {
        Some(i) => {
          Err(format!(
            "There is already a {} at {} named '{} {}.'",
            title, i, first_name, last_name
          ))
        },
        None => {
          Err(format!(
            "There is already a {} named '{} {}.'",
            title, first_name, last_name
          ))
        }
      }
    } else {
      Ok(Collateral::new(
        id,
        first_name,
        last_name,
        title,
        institution,
        pronouns,
        support_type,
        indirect_support
      ))
    };

    result
  }
  pub fn read_collaterals(filepath: &str) -> Vec<Collateral> {
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

    let mut collaterals: Vec<Collateral> = vec![];

    for line in lines {
      let values: Vec<String> = line
        .unwrap()
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let id: u32 = values[0].parse().unwrap();
      let first_name = String::from(&values[1]);
      let last_name = String::from(&values[2]);
      let title = String::from(&values[3]);
      let institution = match &values[4][..] {
        "--NONE--" => None,
        _ => Some(String::from(&values[4])),
      };
      let pronouns: u32 = values[5].parse().unwrap();
      let support_type = match &values[6][..] {
        "Natural" => Natural,
        "Formal" => Formal,
        _ => panic!("Invalud support type stored in file."),
      };
      let indirect_support = match &values[7][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalud indirect support boolean value stored in file."),
      };

      let c = Collateral::new(id, first_name, last_name, title, institution, pronouns, support_type, indirect_support);
      collaterals.push(c);
    }
    collaterals
  }
  pub fn write_collaterals(&self) -> std::io::Result<()> {
    let mut lines = String::from("##### collaterals #####\n");
    for c in &self.collaterals {
      lines.push_str(&c.to_string()[..]);
    }
    lines.push_str("##### collaterals #####");
    let mut file = File::create(self.collateral_filepath.clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  fn get_collateral_user_id(&self, id:u32) -> u32 {
    self.users
      .iter()
      .find(|&u|
        u.clients.iter().any(|&c|
          self.get_client_by_id(c).collaterals.iter().any(|&co_id|
            co_id == id
          )
        )
      )
      .unwrap()
      .id
  }
  fn get_first_client_with_collat_id(&self, id: u32) -> Option<&Client> {
    self.clients.iter().find(|&c| c.collaterals.iter().any(|c_id| c_id == &id ))
  }
  pub fn save_collateral(&mut self, collateral: Collateral) {
    self.collaterals.push(collateral);

    // sort by institution

    self.collaterals.sort_by(|a, b|
      match (a.institution.as_ref(), b.institution.as_ref()) {
        (Some(a_i), Some(b_i)) => a_i.cmp(&b_i),
        _ => a.institution.cmp(&b.institution),
      }
    );
    
    // sort by Natural/Formal
    
    self.collaterals.sort_by(|a, b| a.support_type.cmp(&b.support_type) );

    // sort by client ID (first located)

    let client_ids: Vec<u32> = self.collaterals.iter().map(|c| self.get_first_client_with_collat_id(c.id).unwrap().id ).collect();
    let mut collaterals_and_client_ids = client_ids
      .iter()
      .enumerate()
      .map(|(index, client_id)| (self.collaterals[index].clone(), client_id) )
      .collect::<Vec<(Collateral, &u32)>>();

    collaterals_and_client_ids.sort_by(|(i_a, u_a), (i_b, u_b)| u_a.cmp(&u_b) );

    let collat_refs_by_client = collaterals_and_client_ids
      .iter()
      .map(|(collat, _)| collat )
      .collect::<Vec<&Collateral>>();

    self.collaterals = collat_refs_by_client.iter().map(|col| *col ).cloned().collect();

    // sort by user ID (first located user ID)

    let user_ids: Vec<u32> = self.collaterals.iter().map(|c| self.get_collateral_user_id(c.id)).collect();
    let mut collaterals_and_user_ids = user_ids
      .iter()
      .enumerate()
      .map(|(index, user_id)| (self.collaterals[index].clone(), user_id) )
      .collect::<Vec<(Collateral, &u32)>>();

    collaterals_and_user_ids.sort_by(|(i_a, u_a), (i_b, u_b)| u_a.cmp(&u_b) );

    let collat_refs_by_user = collaterals_and_user_ids
      .iter()
      .map(|(collat, _)| collat )
      .collect::<Vec<&Collateral>>();

    self.collaterals = collat_refs_by_user.iter().map(|col| *col ).cloned().collect();

  }
  fn update_current_collaterals(&mut self, id: u32) {
    self.current_client_mut().collaterals.push(id);
    self.current_collateral_ids = Some(self.current_client_mut().collaterals.clone());
  }
  fn choose_edit_collateral(&mut self) {
    loop {
      self.display_edit_collateral();
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
            Ok(_) => match self.change_collateral_first_name(name_choice.trim()) {
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
        },
        "LAST" | "Last" | "last" | "lst" | "l" | "L" | "last name" | "Last name" | "LAST NAME"
        | "Last Name" => {
          println!("Enter new last name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_collateral_last_name(name_choice.trim()) {
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
        },
        "TITLE" | "title" | "Title" | "t" | "T" => {
          println!("Enter new title:");
          let mut title_choice = String::new();
          let title_attempt = io::stdin().read_line(&mut title_choice);
          match title_attempt {
            Ok(_) => match self.change_collateral_title(title_choice.trim()) {
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
        },
        "INSITUTION" | "Institution" | "institution" | "inst" | "INST" | "Inst" | "I" | "i" => {
          if self.current_collateral().support_type == Natural {
            println!("Unable to add institution for a natural support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          println!("Enter new institution or NONE to remove:");
          let mut inst_choice = String::new();
          let inst_attempt = io::stdin().read_line(&mut inst_choice);
          match inst_attempt {
            Ok(_) => match (self.current_collateral().institution.as_ref(), &inst_choice.trim()[..]) {
              (None, "NONE") => {
                println!("Collateral currently has no institution.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              },
              (Some(i), "NONE") => {
                match self.change_collateral_institution(None) {
                  Ok(_) => (),
                  Err(e) => {
                    println!("Error: {}", e);
                    thread::sleep(time::Duration::from_secs(1));
                  }
                }
              },
              (Some(i), inst_choice_slice) => {
                if &i[..] == inst_choice_slice {
                  println!("Collateral institution already matches.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                } else {
                  let new_inst = String::from(inst_choice.trim());
                  match self.change_collateral_institution(Some(new_inst)) {
                    Ok(_) => (),
                    Err(e) => {
                      println!("Error: {}", e);
                      thread::sleep(time::Duration::from_secs(1));
                    }
                  }
                }
              },
              (None, &_) => {
                let new_inst = String::from(inst_choice.trim());
                match self.change_collateral_institution(Some(new_inst)) {
                  Ok(_) => (),
                  Err(e) => {
                    println!("Error: {}", e);
                    thread::sleep(time::Duration::from_secs(1));
                  }
                }
              }
            }
            Err(e) => {
              println!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        },
        "PRNS" | "Prns" | "prns" | "P" | "p" | "pronouns" | "Pronouns" | "PRONOUNS" => {
          self.current_collateral_mut().pronouns = self.choose_pronouns();
        },
        "FORMAL" | "formal" | "Formal" => {
          if self.current_collateral().support_type == Formal {
            println!("Collateral is already a formal support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            self.current_collateral_mut().support_type = Formal;
          }
        },
        "NATURAL" | "natural" | "Natural" => {
          if self.current_collateral().support_type == Natural {
            println!("Collateral is already a natural support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            if self.current_collateral().institution != None {
              println!("Setting collateral as a natural support will remove the name of any associated institution.");
              println!("Proceed? (Y/N)");
              let mut proceed_choice = String::new();
              let proceed_attempt = io::stdin().read_line(&mut proceed_choice);
              match proceed_attempt {
                Ok(_) => match &proceed_choice.trim()[..] {
                  "Y" | "y" | "YES" | "Yes" | "yes" => (),
                  _ => {
                    println!("Canceled.");
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                  }
                }
                Err(e) => {
                  println!("Failed to read input.");
                  thread::sleep(time::Duration::from_secs(1));
                  continue;
                }
              }
            }

            self.current_collateral_mut().support_type = Natural;
            self.current_collateral_mut().indirect_support = false;
            self.current_collateral_mut().institution = None;
          }
        },
        "INDIRECT" | "indirect" | "Indirect" => {
          if self.current_collateral().indirect_support == true {
            println!("Collateral is already an indirect support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            self.current_collateral_mut().indirect_support = true;
          }
        },
        "DIRECT" | "direct" | "Direct" => {
          if self.current_collateral().indirect_support == false {
            println!("Collateral is already a direct support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            self.current_collateral_mut().indirect_support = false;
          }
        },
        _ => {
          println!("Invalid entry.");
          thread::sleep(time::Duration::from_secs(1));
        }
      }
    }
  }
  fn change_collateral_first_name(&mut self, new_name: &str) -> Result<(), String> {
    let current = self.current_collateral();
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .collaterals
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.title[..], &c.institution))
      .collect();

    let (cf, cl, ct, ci): (&str, &str, &str, &Option<String>) = (
      new_name,
      &current.last_name,
      &current.title,
      &current.institution,
    );

    let result = if names_and_roles
      .iter()
      .any(|(f, l, t, i)| f == &cf && l == &cl && t == &ct && i == &ci)
    {
      match &current.institution {
        Some(i) => {
          Err(format!(
            "There is already a {} at {} named '{} {}.'",
            ct, i, cf, cl
          ))
        },
        None => {
          Err(format!(
            "There is already a {} named '{} {}.'",
            ct, cf, cl
          ))

        }
      }
    } else {
      self.current_collateral_mut().first_name = String::from(new_name);
      Ok(())
    };
    result
  }
  fn change_collateral_last_name(&mut self, new_name: &str) -> Result<(), String> {
    let current = &self.current_collateral();
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .collaterals
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.title[..], &c.institution))
      .collect();

    let (cf, cl, ct, ci): (&str, &str, &str, &Option<String>) = (
      &current.first_name,
      new_name,
      &current.title,
      &current.institution,
    );

    let result = if names_and_roles
      .iter()
      .any(|(f, l, t, i)| f == &cf && l == &cl && t == &ct && i == &ci)
    {
      match &current.institution {
        Some(i) => {
          Err(format!(
            "There is already a {} at {} named '{} {}.'",
            ct, i, cf, cl
          ))
        },
        None => {
          Err(format!(
            "There is already a {} named '{} {}.'",
            ct, cf, cl
          ))
        }
      }
    } else {
      self.current_collateral_mut().last_name = String::from(new_name);
      Ok(())
    };
    result
  }
  fn change_collateral_title(&mut self, new_title: &str) -> Result<(), String> {
    let current = &self.current_collateral();
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .collaterals
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.title[..], &c.institution))
      .collect();

    let (cf, cl, ct, ci): (&str, &str, &str, &Option<String>) = (
      &current.first_name,
      &current.last_name,
      new_title,
      &current.institution,
    );

    let result = if names_and_roles
      .iter()
      .any(|(f, l, t, i)| f == &cf && l == &cl && t == &ct && i == &ci)
    {
      match &current.institution {
        Some(i) => {
          Err(format!(
            "There is already a {} at {} named '{} {}.'",
            ct, i, cf, cl
          ))
        },
        None => {
          Err(format!(
            "There is already a {} named '{} {}.'",
            ct, cf, cl
          ))
        }
      }
    } else {
      self.current_collateral_mut().title = String::from(new_title);
      Ok(())
    };
    result
  }
  fn change_collateral_institution(&mut self, new_inst: Option<String>) -> Result<(), String> {
    let current = self.current_collateral();
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .collaterals
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.title[..], &c.institution))
      .collect();

    let (cf, cl, ct, ci): (&str, &str, &str, &Option<String>) = (
      &current.first_name,
      &current.last_name,
      &current.title,
      &new_inst,
    );

    let result = if names_and_roles
      .iter()
      .any(|(f, l, t, i)| f == &cf && l == &cl && t == &ct && i == &ci)
    {
      match new_inst {
        Some(i) => {
          Err(format!(
            "There is already a {} at {} named '{} {}.'",
            ct, i, cf, cl
          ))
        },
        None => {
          Err(format!(
            "There is already a {} named '{} {}.'",
            ct, cf, cl
          ))
        },
      }
    } else {
      self.current_collateral_mut().institution = new_inst;
      Ok(())
    };
    result
  }
  fn choose_delete_collateral(&mut self) {
    loop {
      self.display_delete_collateral();
      println!("Are you sure you want to delete this collateral?");
      println!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
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
          self.delete_current_collateral();
          break;
        }
        _ => {
          break;
        }
      }
    }
  }
  fn get_clients_by_collateral_id(&self, id: u32) -> Vec<&Client> {
    let c_ids: Vec<&Client> = self.clients
      .iter()
      .filter(|c|
        c.collaterals
          .iter()
          .any(|co_id|
            co_id == &id))
      .collect();
    
    c_ids
  }
  fn display_delete_collateral(&self) {
    let current = self.current_collateral();

    let clients = self.get_clients_by_collateral_id(current.id);
    let client_names: Vec<String> = clients.iter().map(|c| format!("{} {}", c.first_name, c.last_name)).collect();
    let all_client_names = client_names.join(", ");

    let inst_option = &current.institution;
    let display_inst = match inst_option {
      Some(i) => i.to_string(),
      None => String::from("n/a"),
    };

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^162}", "-");
    println!("{:-^162}", " DELETE COLLATERAL ");
    println!("{:-^162}", "-");
    println!(
      "{:-^30} | | {:-^30} | {:-^30} | {:-^40}",
      "Name", "Role/Title", "Institution", "Client(s)"
    );
    println!(
      "{: ^30} | {: ^30} | {: ^30} | {: ^40}",
      current.full_name(),
      current.title,
      display_inst,
      all_client_names,
    );
    println!("{:-^162}", "-");
  }
  fn delete_current_collateral(&mut self) {
    let id = self.current_collateral_id.unwrap();
    self.collaterals.retain(|c| c.id != id);
    self.reindex_collaterals();
    self.current_collateral_id = None;
  }
  fn reindex_collaterals(&mut self) {
    let mut i: u32 = 1;
    for mut co in &mut self.collaterals {
      for cl in &mut self.clients {
        for co_id in &mut cl.collaterals {
          if co_id == &co.id {
            *co_id = i;
          }
        }
      }
      co.id = i;
      i += 1;
    }
  }
  fn get_collateral_by_id(&self, id: u32) -> Option<&Collateral> {
    self.collaterals.iter().find(|p| p.id == id)
  }
  pub fn get_collateral_by_id_mut(&mut self, id: u32) -> Option<&mut Collateral> {
    self.collaterals.iter_mut().find(|p| p.id == id)
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

    let new_pronouns = Pronouns::new(
      id,
      subject.to_lowercase(),
      object.to_lowercase(),
      possessive_determiner.to_lowercase(),
      possessive.to_lowercase(),
    );

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
        println!("'Q'/'QUIT' to quit menu.");
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
                  break;
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
          println!("'Q'/'QUIT' to quit menu.");
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
        String::from("some_random_blank_collateral_file_name.txt"),
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
    fs::remove_file("some_random_blank_collateral_file_name.txt").unwrap();
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
        String::from("test_load_collateral.txt"),
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
    fs::remove_file("test_load_collateral.txt").unwrap();
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
  fn updates_current_pronouns() {
    let mut notes = NoteArchive::new_test();

    notes.load_user(1).unwrap();

    notes.update_current_pronouns(2);
    assert_eq!(notes.current_user().pronouns, 2);

    notes.update_current_pronouns(1);
    assert_eq!(notes.current_user().pronouns, 1);
  }
}
