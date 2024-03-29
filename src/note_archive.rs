use lazy_static::lazy_static;
use regex::Regex;

use ansi_term::Colour::{Black, Red, Green, Yellow, Blue, RGB};
use ansi_term::Style;

use chrono::{Local, NaiveDate, Datelike};
use std::{fs, io, thread, time};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::collections::{HashMap, BTreeMap};

use crate::user::*;
use crate::client::*;
use crate::goal::*;
use crate::collateral::*;
use crate::pronouns::*;
use crate::note_day::*;
use crate::note::*;
use crate::blank_enums::*;
use EmployeeRole::{Fp, Icc};
use SupportType::{Natural, Formal};
use StructureType::{
  CarePlan,
  Intake,
  Assessment,
  Sncd,
  HomeVisit,
  AgendaPrep,
  Debrief,
  PhoneCall,
  Scheduling,
  SentEmail,
  Referral,
  ParentSupport,
  SentCancellation,
  ParentAppearance,
  ParentSkills,
  FailedContactAttempt,
  CategorizedEmails,
  DocumentationStructure,
  AuthorizationRequested,
  AuthorizationIssued,
  CollateralOutreach,
  UpdateFromCollateral,
  InvitedToMeeting,
  SentDocument,
  UpdatedDocument,
  DiscussCommunication,
  ReceivedVerbalConsent,
  ReceivedWrittenConsent,
  BrainstormContribution,
  CustomStructure,
};
use NoteCategory::{ICCNote, FPNote};
use ICCNoteCategory::{FaceToFaceContactWithClient, TelephoneContactWithClient, CareCoordination, Documentation, CarePlanningTeam, TransportClient, MemberOutreachNoShow};
use FPNoteCategory::{DescriptionOfIntervention, ResponseToIntervention, Functioning, PlanAdditionalInformation};
use FPIntervention::{
  FaceToFaceContact,
  CollateralContact,
  CrisisSupport,
  TelephoneSupport,
  DirectTimeWithProviders,
  EducatingCoachingModelingAndGuiding,
  EngageParentCaregiverInAddressingGoals,
  TeachAdvocacyGuideLinkageToResources,
  TeachNetworkingInCommunityAndWithProviders,
  ProviderOutreachToPerson,
  MemberTransportationByStaff,
  NoShowLateCancellation,
  FPInterventionDocumentation,
  Other,
};

use Blank::{
  CurrentUser,
  PartnerICCOrFP,
  CurrentClientName,
  ClientGoal,
  Collaterals,
  AllCollaterals,
  PrimaryContact,
  Guardian,
  CarePlanTeam,
  Pronoun1ForBlank,
  Pronoun2ForBlank,
  Pronoun3ForBlank,
  Pronoun4ForBlank,
  Pronoun1ForUser,
  Pronoun2ForUser,
  Pronoun3ForUser,
  Pronoun4ForUser,
  Pronoun1ForClient,
  Pronoun2ForClient,
  Pronoun3ForClient,
  Pronoun4ForClient,
  TodayDate,
  NoteDayDate,
  InternalDocument,
  ExternalDocument,
  InternalMeeting,
  ExternalMeeting,
  Appearance,
  SupportedParent,
  ParentingSkill,
  CarePlanningTopic,
  YouthTopic,
  ContactMethod,
  ContactPurpose,
  FulfilledContactPurpose,
  Service,
  MeetingMethod,
  SignatureMethod,
  CustomBlank
};

use crate::utils::*;
use crate::constants::*;

pub struct NoteArchive {
  pub users: Vec<User>,
  pub clients: Vec<Client>,
  pub goals: Vec<Goal>,
  pub collaterals: Vec<Collateral>,
  pub general_collaterals: Vec<Collateral>,
  pub pronouns: Vec<Pronouns>,
  pub note_days: Vec<NoteDay>,
  pub note_templates: Vec<NoteTemplate>,
  pub notes: Vec<Note>,
  pub foreign_key: HashMap<String, u32>,
  pub foreign_keys: HashMap<String, Vec<u32>>,
  pub encrypted: bool,
  pub filepaths: HashMap<String, String>,
}

// general functions

use std::format_args;

// macros for printing formatted stuff - print_on_bg duplicated in note.rs

// print on default background
pub fn print_on_bg(s: String) {
  print!("{}", Style::new().on(BG).paint(s));
}
macro_rules! print_on_bg {
    ($($arg:tt)*) => (print_on_bg(format!("{}", format_args!($($arg)*))));
}
macro_rules! println_on_bg {
    () => (print_on_bg("\n"));
    ($fmt:expr) => (print_on_bg(concat!($fmt, "\n").to_string()));
    ($fmt:expr, $($arg:tt)*) => (print_on_bg!(concat!($fmt, "\n"), $($arg)*));
}
// print errors
pub fn print_err(s: String) {
  print!("{}", Black.on(Red).paint(s));
}
macro_rules! print_err {
    ($($arg:tt)*) => (print_err(format!("{}", format_args!($($arg)*))));
}
macro_rules! println_err {
    () => (print_err("\n"));
    ($fmt:expr) => (print_err(concat!($fmt, "\n").to_string()));
    ($fmt:expr, $($arg:tt)*) => (print_err!(concat!($fmt, "\n"), $($arg)*));
}
// print instructions
pub fn print_inst(s: String) {
  print!("{}", Black.on(Blue).paint(s));
}
macro_rules! print_inst {
    ($($arg:tt)*) => (print_inst(format!("{}", format_args!($($arg)*))));
}
macro_rules! println_inst {
    () => (print_inst("\n"));
    ($fmt:expr) => (print_inst(concat!($fmt, "\n").to_string()));
    ($fmt:expr, $($arg:tt)*) => (print_inst!(concat!($fmt, "\n"), $($arg)*));
}
// print yellow
pub fn print_yel(s: String) {
  print!("{}", Yellow.on(Black).paint(s));
}
macro_rules! print_yel {
    ($($arg:tt)*) => (print_yel(format!("{}", format_args!($($arg)*))));
}
macro_rules! println_yel {
    () => (print_yel("\n"));
    ($fmt:expr) => (print_yel(concat!($fmt, "\n").to_string()));
    ($fmt:expr, $($arg:tt)*) => (print_yel!(concat!($fmt, "\n"), $($arg)*));
}
// print on green for success
pub fn print_suc(s: String) {
  print!("{}", Black.on(Green).paint(s));
}
macro_rules! print_suc {
    ($($arg:tt)*) => (print_suc(format!("{}", format_args!($($arg)*))));
}
macro_rules! println_suc {
    () => (print_suc("\n"));
    ($fmt:expr) => (print_suc(concat!($fmt, "\n").to_string()));
    ($fmt:expr, $($arg:tt)*) => (print_suc!(concat!($fmt, "\n"), $($arg)*));
}
// print highlighted content
pub fn print_highlighted_content(s: String) {
  print!("{}", Black.on(RGB(25, 225, 225)).paint(s));
}
macro_rules! print_highlighted_content {
    ($($arg:tt)*) => (print_highlighted_content(format!("{}", format_args!($($arg)*))));
}
fn display_blanks_empty() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  println_on_bg!("{:-^83}", "-");
  println_on_bg!("{:-^83}", " Blanks ");
  println_on_bg!("{:-^83}", "-");
  println_on_bg!("{:-^10} | {:-^70}", " ID ", " Type ");
  println_on_bg!("{:-^83}", "-");
  for (i, b) in Blank::iterator().enumerate() {
    let display = format!("{: ^10} | {: <70}", &format!(" {} ", i+1), b.display_to_user_empty());
    println_on_bg!("{}", display);
  }
  println_on_bg!("{: <83}", "Choose blank type by ID.");
}
fn choose_blanks() -> usize {
  loop {
    display_blanks_empty();
    let mut input = String::new();
    let input_attempt = io::stdin().read_line(&mut input);
    match input_attempt {
      Ok(_) => (),
      Err(e) => {
        println_err!("Failed to read input: {}.", e);
        thread::sleep(time::Duration::from_secs(2));
        continue;
      }
    }
    match input.trim().parse::<usize>() {
      Ok(num) => {
        if num > 0 && num <= Blank::vector_of_variants().len() {
          break num-1;
        } else {
          println_err!("Invalid ID.");
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      },
      Err(e) => {
        println_err!("Failed to read input as a number: {}.", e);
        thread::sleep(time::Duration::from_secs(2));
        continue;
      }
    }
  }
}
fn choose_blanks_option() -> Option<usize> {
  loop {
    display_blanks_empty();
    println_inst!("Enter 'QUIT / Q' at any time to cancel.");
    let mut input = String::new();
    let input_attempt = io::stdin().read_line(&mut input);
    match input_attempt {
      Ok(_) => (),
      Err(e) => {
        println_err!("Failed to read input: {}.", e);
        thread::sleep(time::Duration::from_secs(2));
        continue;
      }
    }
    match &input.to_ascii_lowercase().trim()[..] {
      "quit" | "q" => return None,
      _ => (),
    }
    match input.trim().parse::<usize>() {
      Ok(num) => {
        if num > 0 && num <= Blank::vector_of_variants().len() {
          break Some(num-1);
        } else {
          println_err!("Invalid ID.");
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      },
      Err(e) => {
        println_err!("Failed to read input as a number: {}.", e);
        thread::sleep(time::Duration::from_secs(2));
        continue;
      }
    }
  }
}
pub fn get_spacing_buffers(last_content_char: Option<char>, next_content_char: Option<char>, s: String) -> (String, String) {
  let content_1 = match s.chars().next() {
    Some(c) => c,
    None => 'X', // arbitrary non space value
  };
  let content_2 = match s.chars().last() {
    Some(c) => c,
    None => 'X', // arbitrary non space value
  };
  if !s.chars().any(|c| !c.is_whitespace() ) {
    return (String::from(" "), String::new());
  }
  match (last_content_char, next_content_char) {
    (Some(lc), Some(nc)) => {
      if lc.is_whitespace() && (nc.is_whitespace() || nc == '.') {
        (String::new(), String::new())
      } else if content_1 == ' ' && content_2 == ' ' {
        (String::new(), String::new())
      } else if lc.is_whitespace() || content_1 == ' ' {
        (String::new(), String::from(" "))
      } else if nc.is_whitespace() || nc == '.' || content_2 == ' ' {
        (String::from(" "), String::new())
      } else {
        (String::from(" "), String::from(" "))
      }
    },
    (Some(lc), None) => {
      if lc == ' ' {
        (String::new(), String::from(" "))
      } else {
        (String::from(" "), String::from(" "))
      }
    },
    (None, Some(nc)) => {
      if nc == ' ' {
        (String::from(" "), String::new())
      } else {
        (String::from(" "), String::from(" "))
      }
    },
    (None, None) => {
      // in this case, there is no prior char, which means we wouldn't want a random space at the start of the text.
      // however, a space on the end would be permissible because the input is often trimmed and there are checks for double spacing.
      (String::new(), String::from(" "))
    }
  }
}

impl NoteArchive {
  pub fn run(&mut self) {
    match self.choose_user() {
      Some(_) => {
        self.write_to_files();
        self.logged_in_action();
      },
      None => (),
    }
  }
  fn display_decrypt_files() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_yel!("{:-^58}", "-");
    println_yel!("{:-^58}", " Files not readable ");
    println_yel!("{:-^58}", "-");
    
    println_yel!("{:-^58}", "-");
    println_yel!(
      "{: >15} | {: <40}",
      " DECRPYT / D ", " Attempt to decrypt files with a password "
    );
    println_yel!(
      "{: >15} | {: <40}",
      " DELETE ", " Delete all data and start over "
    );
    println_yel!(
      "{: >15} | {: <40}",
      " QUIT / Q ", " Close program "
    );
    println_yel!("{:-^58}", "-");

  }
  fn delete_from_blanks(&mut self, data_type: String, id: u32) {
    match &data_type[..] {
      "client" => {
        for n in self.current_client_notes_mut() {
          let mut new_blanks = n.blanks.clone();
          for (i, (b, s, v)) in n.blanks.clone() {
            match b {
              CurrentClientName | Pronoun1ForClient | Pronoun2ForClient | Pronoun3ForClient | Pronoun4ForClient => {
                new_blanks.insert(i, (b, s, v.iter().cloned().filter(|c_id| c_id != &id ).collect() ) );
              },
              _ => (),
            }
          }
          n.blanks = new_blanks;
        }
      },
      "goal" => {
        for n in self.current_client_notes_mut() {
          let mut new_blanks = n.blanks.clone();
          for (i, (b, s, v)) in n.blanks.clone() {
            match b {
              ClientGoal => {
                new_blanks.insert(i, (b, s, v.iter().cloned().filter(|c_id| c_id != &id ).collect() ) );
              },
              _ => (),
            }
          }
          n.blanks = new_blanks;
        }
      },
      "note_day" => {
        for n in self.current_note_day_notes_mut() {
          let mut new_blanks = n.blanks.clone();
          for (i, (b, s, v)) in n.blanks.clone() {
            match b {
              NoteDayDate => {
                new_blanks.insert(i, (b, s, v.iter().cloned().filter(|c_id| c_id != &id ).collect() ) );
              },
              _ => (),
            }
          }
          n.blanks = new_blanks;
        }
      },
      "collateral" => {
        for n in self.current_collateral_notes_mut() {
          let mut new_blanks = n.blanks.clone();
          for (i, (b, s, v)) in n.blanks.clone() {
            match b {
              Collaterals | AllCollaterals | PartnerICCOrFP | PrimaryContact | Guardian | CarePlanTeam => {
                new_blanks.insert(i, (b, s, v.iter().cloned().filter(|c_id| c_id != &id ).collect() ) );
              },
              _ => (),
            }
          }
          n.blanks = new_blanks;
        }
      },
      _ => panic!("String other than 'client' or 'collateral' passed to fn 'delete_from_blanks'."),
    }
  }
  fn choose_decrypt_files(
    user_filepath: &str,
    client_filepath: &str,
    goal_filepath: &str,
    collateral_filepath: &str,
    general_collateral_filepath: &str,
    pronouns_filepath: &str,
    note_day_filepath: &str,
    note_template_filepath: &str,
    note_filepath: &str) -> bool {
    loop {
      Self::display_decrypt_files();
      let mut choice = String::new();
      let choice_attempt = io::stdin().read_line(&mut choice);
      match choice_attempt {
        Ok(_) => (),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
        }
      }
      choice = choice.to_ascii_lowercase().trim().to_string();
      match &choice[..] {
        "decrypt" | "d" => {
          println_inst!("Enter password to attempt decryption.");
          let mut pw = String::new();
          let pw_attempt = io::stdin().read_line(&mut pw);
          match pw_attempt {
            Ok(_) => (),
            Err(e) => {
              println_err!("Failed to read input: {}.", e);
            }
          }
          pw = pw.trim().to_string();
          break match Self::decrypt_all_files(
            user_filepath,
            client_filepath,
            goal_filepath,
            collateral_filepath,
            general_collateral_filepath,
            pronouns_filepath,
            note_day_filepath,
            note_template_filepath,
            note_filepath,
            &pw
          ) {
            Ok(_) => true,
            Err(_) => {
              println_err!("Incorrect password. Option to try again in 10 seconds.");
              thread::sleep(time::Duration::from_secs(10));
              continue;
            }
          }
        },
        "delete" => {
          fs::remove_file(user_filepath).unwrap();
          fs::remove_file(client_filepath).unwrap();
          fs::remove_file(goal_filepath).unwrap();
          fs::remove_file(collateral_filepath).unwrap();
          fs::remove_file(general_collateral_filepath).unwrap();
          fs::remove_file(pronouns_filepath).unwrap();
          fs::remove_file(note_day_filepath).unwrap();
          fs::remove_file(note_template_filepath).unwrap();
          fs::remove_file(note_filepath).unwrap();
          break true;
        },
        "quit" | "q" => {
          break false;
        },
        _ => {
          println_err!("Invalid command.");
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      }
    }
  }
  pub fn new(filepaths: HashMap<String, String>) -> NoteArchive {
    let foreign_key: HashMap<String, u32> = HashMap::new();
    let foreign_keys: HashMap<String, Vec<u32>> = HashMap::new();
    let encrypted = false;
    let mut build_note_archive = true;
    match Self::read_users(&filepaths["user_filepath"]) {
      Ok(_) => (),
      Err(_) => {
        build_note_archive = Self::choose_decrypt_files(
          &filepaths["user_filepath"],
          &filepaths["client_filepath"],
          &filepaths["goal_filepath"],
          &filepaths["collateral_filepath"],
          &filepaths["general_collateral_filepath"],
          &filepaths["pronouns_filepath"],
          &filepaths["note_day_filepath"],
          &filepaths["note_template_filepath"],
          &filepaths["note_filepath"],
        );
      }
    }
    if build_note_archive {
      let mut a = NoteArchive {
        users: Self::read_users(&filepaths["user_filepath"]).unwrap(),
        clients: Self::read_clients(&filepaths["client_filepath"]).unwrap(),
        goals: Self::read_goals(&filepaths["goal_filepath"]).unwrap(),
        collaterals: Self::read_collaterals(&filepaths["collateral_filepath"]).unwrap(),
        general_collaterals: Self::read_general_collaterals(&filepaths["general_collateral_filepath"]).unwrap(),
        pronouns: vec![],
        note_days: Self::read_note_days(&filepaths["note_day_filepath"]).unwrap(),
        note_templates: Self::read_note_templates(&filepaths["note_template_filepath"]).unwrap(),
        notes: Self::read_notes(&filepaths["note_filepath"]).unwrap(),
        foreign_key,
        foreign_keys,
        encrypted,
        filepaths,
      };
      a.pronouns = a.read_pronouns().unwrap();
      a
    } else {
      panic!("Unable to access data.");
    }
  }
  pub fn new_test(filepaths: HashMap<String, String>) -> NoteArchive {
    let user_1 = User::new(
      1,
      String::from("Pete"),
      String::from("Peteson"),
      Icc,
      1,
      vec![1, 2],
      vec![1, 2],
    );
    let user_2 = User::new(
      2,
      String::from("Sandy"),
      String::from("Sandyson"),
      Fp,
      1,
      vec![1],
      vec![1],
    );
    let users = vec![user_1, user_2];
    let client_1 = Client::new(
      1,
      String::from("Pete"),
      String::from("McLastName"),
      NaiveDate::from_ymd(2006, 1, 2),
      1,
      vec![1, 2],
    );
    let client_2 = Client::new(
      2,
      String::from("Sandy"),
      String::from("O'Lastnymn"),
      NaiveDate::from_ymd(2007, 2, 3),
      1,
      vec![],
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
      false,
      false,
      false,
      false,
      Local::now().naive_local().date(),
    );
    let collateral_2 = Collateral::new(
      2,
      String::from("Barry"),
      String::from("Plith"),
      String::from("OPT"),
      Some(String::from("Family Solutions, Inc.")),
      1,
      Formal,
      false,
      false,
      false,
      false,
      Local::now().naive_local().date(),
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
    let p3 = Pronouns::new(
      3,
      String::from("they"),
      String::from("them"),
      String::from("their"),
      String::from("theirs"),
    );
    let pronouns = vec![p1, p2, p3];
    let nd1 = NoteDay::new(
      1,
      Local::now().naive_local().date(),
      1,
      1,
      vec![],
    );
    let nd2 = NoteDay::new(
      2,
      Local::now().naive_local().date(),
      1,
      1,
      vec![],
    );
    let note_days = vec![nd1, nd2];

    let mut notes = NoteArchive::new(filepaths);

    let nt1 = NoteTemplate::new(
      1,
      CarePlan,
      true,
      String::from("ICC met with (---co---) for a Care Plan Meeting for (---c---)."),
      vec![2],
    );
    let nt2 = NoteTemplate::new(
      2,
      PhoneCall,
      true,
      String::from("ICC called (---co---) to discuss a referral for IHT services."),
      vec![1],
    );

    let note_templates = vec![nt1, nt2];

    notes.users = users;
    notes.clients = clients;
    notes.collaterals = collaterals;
    notes.pronouns = pronouns;
    notes.note_days = note_days;
    notes.note_templates = note_templates;
    notes.write_to_files();

    notes
  }
  fn write_to_files(&mut self) {
    self.write_users().unwrap();
    self.write_clients().unwrap();
    self.write_goals().unwrap();
    self.write_collaterals().unwrap();
    self.write_general_collaterals().unwrap();
    self.write_pronouns().unwrap();
    self.write_note_days().unwrap();
    self.write_note_templates().unwrap();
    self.write_notes().unwrap();
  }
  fn encrypt_all_files(&self, pw: &str) -> Result<(), Error> {
    match Self::read_users(&self.filepaths["user_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["user_filepath"], pw)?,
      Err(_) => (),
    }
    match Self::read_clients(&self.filepaths["client_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["client_filepath"], pw)?,
      Err(_) => (),
    }
    match Self::read_goals(&self.filepaths["goal_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["goal_filepath"], pw)?,
      Err(_) => (),
    }
    match Self::read_collaterals(&self.filepaths["collateral_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["collateral_filepath"], pw)?,
      Err(_) => (),
    }
    match Self::read_general_collaterals(&self.filepaths["general_collateral_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["general_collateral_filepath"], pw)?,
      Err(_) => (),
    }
    match Self::read_pronouns_from_file_without_reindexing(&self.filepaths["pronouns_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["pronouns_filepath"], pw)?,
      Err(_) => (),
    }
    match Self::read_note_days(&self.filepaths["note_day_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["note_day_filepath"], pw)?,
      Err(_) => (),
    }
    match Self::read_note_templates(&self.filepaths["note_template_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["note_template_filepath"], pw)?,
      Err(_) => (),
    }
    match Self::read_notes(&self.filepaths["note_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["note_filepath"], pw)?,
      Err(_) => (),
    }
    Ok(())
  }
  fn decrypt_all_files(
    user_filepath: &str,
    client_filepath: &str,
    goal_filepath: &str,
    collateral_filepath: &str,
    general_collateral_filepath: &str,
    pronouns_filepath: &str,
    note_day_filepath: &str,
    note_template_filepath: &str,
    note_filepath: &str,
    pw: &str) -> Result<(), Error> {
    decrypt_file(user_filepath, "decrypt_attempt_user.txt", pw)?;
    decrypt_file(client_filepath, "decrypt_attempt_client.txt", pw)?;
    decrypt_file(goal_filepath, "decrypt_attempt_goal.txt", pw)?;
    decrypt_file(collateral_filepath, "decrypt_attempt_collateral.txt", pw)?;
    decrypt_file(general_collateral_filepath, "decrypt_attempt_general_collateral.txt", pw)?;
    decrypt_file(pronouns_filepath, "decrypt_attempt_pronouns.txt", pw)?;
    decrypt_file(note_day_filepath, "decrypt_attempt_note_day.txt", pw)?;
    decrypt_file(note_template_filepath, "decrypt_attempt_note_template.txt", pw)?;
    decrypt_file(note_filepath, "decrypt_attempt_note.txt", pw)?;
    let user_result = Self::read_users("decrypt_attempt_user.txt");
    let client_result = Self::read_clients("decrypt_attempt_client.txt");
    let goal_result = Self::read_goals("decrypt_attempt_goal.txt");
    let collateral_result = Self::read_collaterals("decrypt_attempt_collateral.txt");
    let general_collateral_result = Self::read_general_collaterals("decrypt_attempt_collateral.txt");
    let pronouns_result = Self::read_pronouns_from_file_without_reindexing("decrypt_attempt_pronouns.txt");
    let note_day_result = Self::read_note_days("decrypt_attempt_note_day.txt");
    let note_template_result = Self::read_note_templates("decrypt_attempt_note_template.txt");
    let note_result = Self::read_notes("decrypt_attempt_note.txt");
    fs::remove_file("decrypt_attempt_user.txt")?;
    fs::remove_file("decrypt_attempt_client.txt")?;
    fs::remove_file("decrypt_attempt_collateral.txt")?;
    fs::remove_file("decrypt_attempt_pronouns.txt")?;
    fs::remove_file("decrypt_attempt_note_day.txt")?;
    fs::remove_file("decrypt_attempt_note_template.txt")?;
    fs::remove_file("decrypt_attempt_note.txt")?;
    match (
      user_result,
      client_result,
      goal_result,
      collateral_result,
      general_collateral_result,
      pronouns_result,
      note_day_result,
      note_template_result,
      note_result
    ) {
      (Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_)) => {
        decrypt_file(user_filepath, user_filepath, pw)?;
        decrypt_file(client_filepath, client_filepath, pw)?;
        decrypt_file(goal_filepath, goal_filepath, pw)?;
        decrypt_file(collateral_filepath, collateral_filepath, pw)?;
        decrypt_file(general_collateral_filepath, general_collateral_filepath, pw)?;
        decrypt_file(pronouns_filepath, pronouns_filepath, pw)?;
        decrypt_file(note_day_filepath, note_day_filepath, pw)?;
        decrypt_file(note_template_filepath, note_template_filepath, pw)?;
        decrypt_file(note_filepath, note_filepath, pw)?;
        Ok(())
      },
      _ => Err(Error::new(
        ErrorKind::Other,
        "Failed to decrypt files with the given password.",
      )),
    }
  }
  fn display_actions(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^58}", "-");
    let heading_with_spaces = format!(" Notes archive for {} ", self.current_user().name_and_title()); 
    println_on_bg!("{:-^58}", heading_with_spaces);
    println_on_bg!("{:-^58}", "-");
    
    println_on_bg!("{: >15} | {: <40}", " NOTE / N ", " Write, view, and edit note records ");
    println_on_bg!("{: >15} | {: <40}", " CLIENT / C ", " View/edit client records ");
    println_on_bg!("{: >15} | {: <40}", " COL / CO ", " View/edit collateral records ");
    
    println_on_bg!("{:-^58}", "-");
    
    println_on_bg!("{: >15} | {: <40}", " USER / U ", " Switch user ");
    println_on_bg!("{: >15} | {: <40}", " EDIT / E ", " Edit current user info ");
    println_on_bg!("{: >15} | {: <40}", " PRNS / P ", " View/edit pronoun records ");
    
    println_on_bg!("{:-^58}", "-");
    
    println_on_bg!("{: >15} | {: <40}", " DELETE / D ", " Delete current user ");
    println_on_bg!("{: >15} | {: <40}", " SECURITY / S ", " Security options ");
    println_on_bg!("{: >15} | {: <40}", " QUIT / Q ", " End program ");

    println_on_bg!("{:-^58}", "-");
  }
  fn sort_data_by_dates(&mut self) {
    let current_collateral_id: Option<u32> = self.foreign_key.get("current_collateral_id").map_or(None, |v| Some(*v) );
    let current_note_day_id: Option<u32> = self.foreign_key.get("current_note_day_id").map_or(None, |v| Some(*v) );
    let current_note_id: Option<u32> = self.foreign_key.get("current_note_id").map_or(None, |v| Some(*v) );
    self.sort_collaterals();
    let new_collateral_id = self.reindex_collaterals(current_collateral_id);
    self.note_days.sort_by(|a, b| b.date.cmp(&a.date) );
    let new_note_day_id = self.reindex_note_days(current_note_day_id);
    self.notes.sort_by(|a, b| b.date.cmp(&a.date) );
    let new_note_id = self.reindex_notes(current_note_id);
    let new_collateral_id_return_option = match new_collateral_id {
      Some(collateral_id) => self.foreign_key.insert(String::from("current_collateral_id"), collateral_id),
      None => None,
    };
    match new_collateral_id_return_option {
      _ => (),
    }
    let new_note_day_id_return_option = match new_note_day_id {
      Some(note_day_id) => {
        let current_client = self.get_client_by_note_day_id(note_day_id).unwrap().clone();
        self.foreign_key.insert(String::from("current_client_id"), current_client.id);
        self.foreign_key.insert(String::from("current_note_day_id"), note_day_id)
      }
      None => None,
    };
    match new_note_day_id_return_option {
      _ => (),
    }
    let new_note_id_return_option = match new_note_id {
      Some(note_id) => self.foreign_key.insert(String::from("current_note_id"), note_id),
      None => None,
    };
    match new_note_id_return_option {
      _ => (),
    }
  }
  fn logged_in_action(&mut self) {
    loop {
      self.sort_data_by_dates();
      self.display_actions();

      let mut choice = String::new();
      let choice_attempt = io::stdin().read_line(&mut choice);
      match choice_attempt {
        Ok(_) => (),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
        }
      }
      choice = choice.to_ascii_lowercase().trim().to_string();
      match &choice[..] {
        "note" | "n" => {
          self.choose_note_days();
        }
        "client" | "c" => {
          self.choose_clients();
        },
        "collateral" | "co" | "col" => {
          self.choose_collaterals();
        },
        "edit" | "e" => {
          self.choose_edit_user();
        },
        "user" | "u" => {
          self.foreign_key.remove("current_user_id");
          self.foreign_key.remove("current_client_id");
          self.foreign_key.remove("current_collateral_id");
          self.foreign_key.remove("current_note_id");
          self.foreign_key.remove("current_note_day_id");
          self.foreign_key.remove("current_note_template_id");
          self.foreign_key.remove("current_goal_id");
          match self.choose_user() {
            Some(_) => (),
            None => break,
          }
        },
        "p" | "prns" | "pronouns" => {
          let chosen_pronoun = self.choose_pronouns_option();
          match chosen_pronoun {
            Some(prn) => {
              self.view_pronoun(prn);
            },
            None => (),
          }
        },
        "delete" | "d" => {
          match self.choose_delete_user() {
            Some(_) => {
              loop {
                match self.choose_user() {
                  _ => break,
                }

              }
            }
            None => (),
          }
        },
        "security" | "s" => {
          self.choose_security_options();
          if self.encrypted {
            break;
          }
        },
        "quit" | "q" => {
          break ();
        },
        _ => {
          println_err!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
        },
      }
      self.write_to_files();
    }
  }
  fn choose_security_options(&mut self) {
    self.display_security_options();
    loop {
      let mut choice = String::new();
      let choice_attempt = io::stdin().read_line(&mut choice);
      match choice_attempt {
        Ok(_) => (),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
          continue;
        }
      }
      choice = choice.to_ascii_lowercase().trim().to_string();
      match &choice[..] {
        "encrypt" => {
          self.choose_encrypt_all_files();
          break;
        },
        "quit" | "q" => {
          break ();
        },
        _ => {
          println_err!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      }
    }

  }
  fn display_security_options(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{:-^58}", " Security ");
    println_on_bg!("{:-^58}", "-");
    
    println_on_bg!(
      "{: >15} | {: <40}",
      " ENCRYPT ", " Encrypt all files and protect with a password | WARNING: cannot be reversed if password is lost. "
    );
    println_on_bg!(
      "{: >15} | {: <40}",
      " QUIT / Q ", " Cancel "
    );
    
    println_on_bg!("{:-^58}", "-");
  }
  fn choose_encrypt_all_files(&mut self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_yel!("If you forget your password, accessing this program's data will be impossible.");
    println_yel!("This applies to all data for all users and clients associated with this program on your computer.");
    println_yel!("Encryption is not and is not intended to be HIPPA compliant.");
    println_yel!("By continuing, you agree that the responsibility to store and transfer data privately and securely is entirely your own.");
    println_yel!("To cancel and return to the program, enter QUIT / Q.");
    println_yel!("To continue with secure encryption, enter YES / Y.");

    loop {
      let mut choice = String::new();
      let choice_attempt = io::stdin().read_line(&mut choice);
      match choice_attempt {
        Ok(_) => (),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
          continue;
        }
      }
      choice = choice.to_ascii_lowercase().trim().to_string();
      match &choice[..] {
        "yes" | "y" => {
          let new_password = loop {
            println_inst!("Enter new password for encryption (minimum 8 characters):");
            let mut choice = String::new();
            let choice_attempt = io::stdin().read_line(&mut choice);
            match choice_attempt {
              Ok(_) => {
                if choice.trim().len() < 8 {
                  println_err!("Password not long enough.");
                  continue;
                } else {
                  println_inst!("Confirm password:");
                  let mut confirm = String::new();
                  let confirm_attempt = io::stdin().read_line(&mut confirm);
                  match confirm_attempt {
                    Ok(_) => {
                      if confirm.trim() != choice.trim() {
                        println_err!("Passwords do not match.");
                        continue;
                      } else {
                        break confirm.trim().to_string()
                      }
                    },
                    Err(e) => {
                      println_err!("Passwords do not match (error: {})", e);
                      continue;
                    }
                  }
                  
                }
              },
              Err(e) => {
                println_err!("Failed to read input: {}.", e);
                continue;
              }
            }
          };
          match self.encrypt_all_files(&new_password) {
            Ok(_) => (),
            Err(e) => {
              println_err!("Failed to encrypt files: {}", e);
              continue;
            },
          }
          self.encrypted = true;
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          println_suc!("Files encrypted successfully.");
          thread::sleep(time::Duration::from_secs(2));
          break;
        },
        "quit" | "q" => {
          break ();
        },
        _ => {
          println_err!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      }
    }
  }

  // users

  fn current_user_mut(&mut self) -> &mut User {
    let user_id = match self.foreign_key.get("current_user_id") {
      Some(id) => id,
      None => panic!("There is no user loaded."),
    };
    let maybe_current: Option<&mut User> = self.users.iter_mut().find(|u| u.id == *user_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded user ID does not match any saved users."),
    }
  }
  fn current_user(&self) -> &User {
    let user_id = match self.foreign_key.get("current_user_id") {
      Some(id) => id,
      None => panic!("There is no user loaded."),
    };
    let maybe_current: Option<&User> = self.users.iter().find(|u| u.id == *user_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded user id does not match any saved users."),
    }
  }
  fn display_users(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^66}", "-");
    println_on_bg!("{:-^66}", " Users ");
    println_on_bg!("{:-^66}", "-");
    println_on_bg!("{:-^10} | {:-^10} | {:-^40}", " ID ", " Role ", " Name ");
    for u in &self.users {
      println_on_bg!(
        "{: ^10} | {: ^10} | {: ^40}",
        u.id,
        u.role.to_string(),
        u.full_name()
      );
    }
    println_on_bg!("{:-^66}", "-");
  }
  fn display_edit_user(&self) {
    let pronouns_option = self.get_pronouns_by_id(self.current_user().pronouns);
    let display_pronouns = match pronouns_option {
      Some(p) => p.short_string(),
      None => String::from("-----"),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^90}", "-");
    println_on_bg!("{:-^90}", " Edit user ");
    println_on_bg!("{:-^90}", "-");
    println_on_bg!(
      "{:-^10} | {:-^20} | {:-^20} | {:-^30}",
      "Role", "First name", "Last name", "Pronouns"
    );
    println_on_bg!(
      "{: ^10} | {: ^20} | {: ^20} | {: ^30}",
      self.current_user().role.to_string(),
      self.current_user().first_name,
      self.current_user().last_name,
      display_pronouns,
    );
    println_on_bg!("{:-^90}", "-");
    println_on_bg!("Choose field to edit (FIRST, LAST, ROLE, PRNS).");
    println_on_bg!("'Q'/'QUIT' to return to previous menu.");
  }
  fn load_user(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&User> = self.users.iter().find(|u| u.id == id);
    match current {
      Some(u) => {
        let prns_id = u.pronouns;
        self.foreign_key.insert(String::from("current_user_id"), u.id);

        match self.load_pronouns(prns_id) {
          Ok(_) => (),
          Err(e) => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println_err!("Error: {} Pronoun record not found. Please select pronouns again.", e);
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
  fn choose_user(&mut self) -> Option<u32> {
    'outer: loop {
      self.display_users();
      let chosen_id = loop {
        let input = loop {
          let mut choice = String::new();
          println_inst!("| {} | {} | {}", "Enter ID to choose user.", "NEW / N: new user", "QUIT / Q: Quit program");
          let read_attempt = io::stdin().read_line(&mut choice);
          match read_attempt {
            Ok(_) => break choice.to_ascii_lowercase(),
            Err(e) => {
              println_err!("Could not read input; try again ({}).", e);
              continue;
            }
          }
        };
        let input = input.trim();
        match input {
          "new" | "n" => {
            let maybe_user_id = self.create_user_get_id();
            match maybe_user_id {
              Some(num) => break Some(num),
              None => continue 'outer,
            }
          }
          "cancel" | "c" | "quit" | "q" => {
            break None;
          }
          _ => match input.parse() {
            Ok(num) => break Some(num),
            Err(e) => {
              println_err!("Could not read input as a number; try again ({}).", e);
              continue;
            }
          },
        }
      };
      match chosen_id {
        Some(id) => {
          match self.load_user(id) {
            Ok(_) => break Some(id),
            Err(e) => {
              println_err!("Unable to load user with id {}: {}", id, e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        },
        None => break None,
      }
    }
  }
  fn create_user_get_id(&mut self) -> Option<u32> {
    let user = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        print_inst!("{}", "Enter 'CANCEL' at any time to cancel.");
        print!("\n");
        print_inst!("First name:");
        print!("\n");
        let first_name_attempt = io::stdin().read_line(&mut first_name_choice);
        match first_name_attempt {
          Ok(_) => break String::from(first_name_choice.trim()),
          Err(e) => {
            println_err!("Invalid first name: {}", e);
            continue;
          }
        };
      };
      if first_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let last_name = loop {
        let mut last_name_choice = String::new();
        println_inst!("Last name:");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println_err!("Invalid last name: {}", e);
            continue;
          }
        };
      };
      if last_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let role: EmployeeRole = loop {
        let mut role_choice = String::new();
        println_inst!("Role ('ICC' or 'FP'):");
        let role_attempt = io::stdin().read_line(&mut role_choice);
        match role_attempt {
          Ok(_) => match &role_choice.trim().to_ascii_lowercase()[..] {
            "icc" => break Icc,
            "fp" => break Fp,
            "cancel" => return None,
            _ => {
              println_err!("Please choose role 'FP' or 'ICC.'");
              continue;
            }
          },
          Err(e) => {
            println_err!("Unreadable entry: {}", e);
            continue;
          }
        };
      };
      let pronouns = 'pronouns: loop {
        match self.choose_pronouns_option() {
          Some(p) => break p,
          None => {
            loop {
              println_inst!("Cancel? (Y/N)");
              let mut cancel = String::new();
              let cancel_attempt = io::stdin().read_line(&mut cancel);
              match cancel_attempt {
                Ok(_) => match &cancel.trim().to_lowercase()[..] {
                  "yes" | "y"  => return None,
                  "no" | "n" | "cancel" => continue 'pronouns,
                  _ => {
                    println_err!("Please choose either 'yes/y' or 'no/n'.");
                    continue;
                  }
                },
                Err(e) => {
                  println_err!("Failed to read input: {}.", e);
                  continue;
                }
              }
  
            }
          }
        } 
      };
      let user_attempt = self.generate_unique_new_user(first_name, last_name, role, pronouns);
      match user_attempt {
        Ok(user) => break user,
        Err(e) => {
          println_err!("User could not be generated: {}.", e);
          continue;
        }
      }
    };
    let id = user.id;
    self.save_user(user);
    Some(id)
  }
  fn user_dup_id_option(&self, first_name: &str, last_name: &str, role: &EmployeeRole) -> Option<u32> {
    let names_and_roles: Vec<(&str, &str, &EmployeeRole, u32)> = self
      .users
      .iter()
      .map(|u| (&u.first_name[..], &u.last_name[..], &u.role, u.id))
      .collect();

      match names_and_roles
      .iter()
      .find(|(f, l, r, _)| f == &first_name && l == &last_name && r == &role) {
        Some(name_and_role_tup) => Some(name_and_role_tup.3),
        None => None,
      }

  }
  fn generate_unique_new_user(
    &mut self,
    first_name: String,
    last_name: String,
    role: EmployeeRole,
    pronouns: u32,
  ) -> Result<User, String> {
    if first_name.contains(" | ") || last_name.contains(" | ") {
      return Err(String::from("Name cannot contain ' | '."));
    }
    
    let id: u32 = self.users.len() as u32 + 1;

    match self.user_dup_id_option(&first_name, &last_name, &role) {
      Some(_) => Err(format!("There is already a {} with the name '{} {}'.", role, first_name, last_name)),
      None => Ok(User::new(id, first_name, last_name, role, pronouns, vec![], vec![])),
    }
  }
  fn save_user(&mut self, user: User) {
    let pos = self.users.binary_search_by(|u| u.id.cmp(&user.id) ).unwrap_or_else(|e| e);
    self.users.insert(pos, user);
    self.write_to_files();
  }
  pub fn write_users(&mut self) -> std::io::Result<()> {
    let mut lines = String::from("##### users #####\n");
    for u in &self.users {
      lines.push_str(&u.to_string());
    }
    lines.push_str("##### users #####");
    let mut file = File::create(self.filepaths["user_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn read_users(filepath: &str) -> Result<Vec<User>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
    }

    let mut users: Vec<User> = vec![];

    for line in lines {
      let line_string = line?;

      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      if values.len() < 7 {
        return Err(Error::new(ErrorKind::Other, "Failed to read users from filepath."));
      }

      let id: u32 = values[0].parse().unwrap();
      let first_name = String::from(&values[1]);
      let last_name = String::from(&values[2]);
      let role = match &values[3][..] {
        "FP" => Ok(Fp),
        "ICC" => Ok(Icc),
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

      let collaterals: Vec<u32> = match &values[6][..] {
        "" => vec![],
        _ => values[6]
        .split("#")
        .map(|val| val.parse().unwrap())
          .collect(),
      };

      
      let u = User::new(id, first_name, last_name, role, pronouns, clients, collaterals);
      users.push(u);
    }
    users.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(users)
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
        Err(_) => {
          println_err!("Failed to read input. Please try again.");
          continue;
        }
      }
      field_to_edit = field_to_edit.to_ascii_lowercase().trim().to_string();
      match &field_to_edit[..] {
        "quit" | "q" => {
          break ();
        }
        _ => (),
      }
      match &field_to_edit[..] {
        "first" | "fst" | "f" | "1st" | "first name" => {
          println_inst!("Enter new first name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_user_first_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              continue;
            }
          }
        }
        "last" | "lst" | "l" | "last name" => {
          println_inst!("Enter new last name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_user_last_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              continue;
            }
          }
        }
        "role" | "r" => match self.current_user().role {
          Icc => {
            self.change_role(&Fp).unwrap();
          }
          Fp => {
            self.change_role(&Icc).unwrap();
          }
        },
        "prns" | "p" | "pronouns" => {
          let pronouns_option = self.choose_pronouns_option();
          match pronouns_option {
            Some(p) => self.current_user_mut().pronouns = p,
            None => (),
          }
        }
        _ => println_err!("Invalid entry."),
      }
    }
  }
  fn choose_delete_user(&mut self) -> Option<()> {
    loop {
      self.display_delete_user();
      println_yel!("Are you sure you want to delete this user?");
      println_yel!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
      let mut choice = String::new();
      let input_attempt = io::stdin().read_line(&mut choice);
      match input_attempt {
        Ok(_) => choice = choice.trim().to_string(),
        Err(e) => {
          println_err!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      }
      match &choice.to_ascii_lowercase()[..] {
        "yes" | "y" => {
          self.delete_current_user();
          self.reindex_users();
          self.write_to_files();
          break Some(());
        },
        _ => {
          break None;
        },
      }
    }
  }
  fn display_delete_user(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^79}", "-");
    println_on_bg!("{:-^79}", " Delete user ");
    println_on_bg!("{:-^79}", "-");
    println_on_bg!(
      "{:-^10} | {:-^20} | {:-^20} | {:-^20}",
      "Role", "First name", "Last name", "Client records",
    );
    println_on_bg!(
      "{: ^10} | {: ^20} | {: ^20} | {: ^20}",
      self.current_user().role.to_string(),
      self.current_user().first_name,
      self.current_user().last_name,
      self.current_user().foreign_keys["client_ids"].len(),
    );
    println_on_bg!("{:-^79}", "-");
  }
  fn delete_current_user(&mut self) {
    let current_clients = self.get_current_clients().iter().map(|c| c.id ).collect::<Vec<u32>>();
    for c_id in current_clients {
      match self.load_client(c_id) {
        Err(_) => panic!("Failed to delete client for current user."),
        Ok(_) => self.delete_current_client(),
      }
    }
    self.reindex_clients();
    let current_note_days = self.current_user_note_days().iter().map(|nd| nd.id ).collect::<Vec<u32>>();
    for nd_id in current_note_days {
      match self.load_note_day(nd_id) {
        Err(_) => panic!("Failed to delete note day for current user."),
        Ok(_) => self.delete_current_note_day(),
      }
    }
    let new_note_day_id = self.reindex_note_days(None);
    match new_note_day_id {
      _ => (),
    }
    let current_notes = self.current_user_notes().iter().map(|n| n.id ).collect::<Vec<u32>>();
    for n_id in current_notes {
      match self.load_note(n_id) {
        Err(_) => panic!("Failed to delete note day for current user."),
        Ok(_) => self.delete_current_note(),
      }
    }
    let new_note_id = self.reindex_notes(None);
    match new_note_id {
      _ => (),
    }

    let id = self.foreign_key.get("current_user_id").unwrap().to_owned();
    self.delete_from_blanks(String::from("user"), id);
    self.users.retain(|u| u.id != id);
    self.note_templates.retain(|nt| nt.foreign_keys["user_ids"].len() > 0 );
    self.foreign_key.remove("current_user_id");
    self.foreign_key.remove("current_client_id");
    self.foreign_key.remove("current_collateral_id");
  }
  fn reindex_users(&mut self) {
    let mut i: u32 = 1;
    let mut new_note_days: Vec<NoteDay> = self.note_days.clone();
    let mut new_notes: Vec<Note> = self.notes.clone();
    let mut changes: HashMap<u32, u32> = HashMap::new();
    for mut u in &mut self.users {
      for nd in &mut new_note_days {
        if nd.foreign_key["user_id"] == u.id {
          nd.foreign_key.insert(String::from("user_id"), i);
        }
      }
      for n in &mut new_notes {
        if n.foreign_key["user_id"] == u.id {
          n.foreign_key.insert(String::from("user_id"), i);
        }
      }
      if u.id == i {
        ()
      } else {
        changes.insert(u.id, i);
        u.id = i;
      }
      i += 1;
    }

    for nt in &mut self.note_templates {
      let old_ids = nt.foreign_keys["user_ids"].clone();
      let new_ids: Vec<u32> = old_ids.iter()
        .map(|co_id| match changes.get(co_id) { Some(val) => *val, None => *co_id } )
        .collect();
      nt.foreign_keys.insert(String::from("user_ids"), new_ids);
    }
    self.note_days = new_note_days;
    self.notes = new_notes;
  }

  // clients
  fn current_client_mut(&mut self) -> &mut Client {
    let client_id = match self.foreign_key.get("current_client_id") {
      Some(id) => id,
      None => panic!("There is no current client selected."),
    };
    let maybe_current: Option<&mut Client> = self.clients.iter_mut().find(|c| c.id == *client_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded client ID does not match any saved clients."),
    }
  }
  fn current_client(&self) -> &Client {
    let client_id = match self.foreign_key.get("current_client_id") {
      Some(id) => id,
      None => panic!("There is no current client selected."),
    };
    let maybe_current: Option<&Client> = self.clients.iter().find(|c| c.id == *client_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded client ID does not match any saved clients."),
    }
  }
  fn get_current_clients(&self) -> Vec<&Client> {
    self.clients.iter().filter(|client| self.current_user().foreign_keys["client_ids"]
        .iter()
        .any(|&id| id == client.id)
      )
      .collect()
  }
  fn display_clients(&self) {
    let heading = format!(" {}'s clients ", &self.current_user().full_name()[..]);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^96}", heading);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Name ", " DOB ");
    match self.foreign_key.get("current_user_id") {
      Some(_) => {
        for c in self.get_current_clients() {
          println_on_bg!(
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
    println_on_bg!("{:-^96}", "-");
    println_inst!("| {} | {} | {}", "Choose client by ID.", "NEW / N: new client", "ADD / A: Add from other user");
    println_inst!("| {} | {}", "EDIT / E: edit records", "QUIT / Q: quit menu");
  }
  fn display_select_clients(&self) {
    let mut heading = String::from(" ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^96}", heading);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Name ", " DOB ");
    match self.foreign_key.get("current_user_id") {
      Some(_) => {
        for c in self.get_current_clients() {
          println_on_bg!(
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
    println_on_bg!("{:-^96}", "-");
    println_inst!("| {} | {} | {}", "Choose client for note by ID.", "NEW / N: new client", "ADD / A: Add from other user");
    println_inst!("| {}", "EDIT / E: edit records");
  }
  fn display_edit_clients(&self) {
    let mut heading = String::from(" Edit ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^96}", heading);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Name ", " DOB ");
    match self.foreign_key.get("current_user_id") {
      Some(_) => {
        for c in self.get_current_clients() {
          println_on_bg!(
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
    println_on_bg!("{:-^96}", "-");
    println_inst!("| {} | {}", "Choose client by ID.", "QUIT / Q: quit menu");
  }
  fn get_noncurrent_clients(&self) -> Vec<&Client> {
    self.clients.iter().filter(|client| !self.current_user().foreign_keys["client_ids"]
        .iter()
        .any(|&id| id == client.id ||
          (self.get_client_by_id(id).unwrap().first_name == client.first_name
          && self.get_client_by_id(id).unwrap().last_name == client.last_name
          && self.get_client_by_id(id).unwrap().dob == client.dob)
        )
      )
      .collect()
  }
  fn display_add_client(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^96}", " Clients ");
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Name ", " DOB ");
    match &self.foreign_key.get("current_user_id") {
      Some(_) => {
        for c in self.get_noncurrent_clients() {
          println_on_bg!(
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
          println_on_bg!(
            "{: ^10} | {: ^40} | {: <12} {: >26}",
            c.id,
            c.full_name(),
            c.fmt_dob(),
            c.fmt_date_of_birth()
          );
        }
      },
    }
    println_on_bg!("{:-^96}", "-");
    println_inst!("| {} | {} | {}", "Enter ID to add client.", "NEW / N: create new", "QUIT / Q: quit menu");
  }
  fn display_client(&self) {
    let pronouns_option = self.get_pronouns_by_id(self.current_client().pronouns);
    let display_pronouns = match pronouns_option {
      Some(p) => p.short_string(),
      None => String::from("-----"),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^119}", "-");
    println_on_bg!("{:-^119}", " View client record ");
    println_on_bg!("{:-^119}", "-");
    println_on_bg!(
      "{:-^20} | {:-^20} | {:-^30} | {:-^40}",
      "First name", "Last name", "Pronouns", "DOB"
    );
    println_on_bg!(
      "{: ^20} | {: ^20} | {: ^30} | {: <12} {: >26}",
      self.current_client().first_name,
      self.current_client().last_name,
      display_pronouns,
      self.current_client().fmt_dob(),
      self.current_client().fmt_date_of_birth(),
    );
    println_on_bg!("{:-^119}", "-");
  }
  fn load_client(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&Client> = self.clients.iter().find(|c| c.id == id);
    match current {
      Some(c) => {
        self.foreign_key.insert(String::from("current_client_id"), c.id);
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
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        },
        "new" | "n" => {
          let maybe_new_id = self.create_client_get_id();
          match maybe_new_id {
            Some(new_id) => self.update_current_clients(new_id),
            None => continue,
          }
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.get_noncurrent_clients()
              .iter()
              .map(|c| c.id )
              .any(|id| id == num) {
              println_err!("Please select from the available choices.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            } else {
              match self.load_client(num) {
                Ok(_) => {
                  let to_copy = self.get_client_by_id(num).unwrap().clone();
                  let copied = Client::new(
                    (self.clients.len() + 1) as u32,
                    to_copy.first_name.clone(),
                    to_copy.last_name.clone(),
                    to_copy.dob.clone(),
                    to_copy.pronouns,
                    vec![],
                  );
                  self.update_current_clients(copied.id);
                  self.save_client(copied);
                  break;
                }
                Err(e) => {
                  println_err!("Unable to load client with id {}: {}", num, e);
                  continue;
                }
              }
            }
          },
          Err(e) => {
            println_err!("Failed to read input: {}.", e);
            thread::sleep(time::Duration::from_secs(2));
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
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_client_get_id();
          match maybe_new_id {
            Some(new_id) => self.update_current_clients(new_id),
            None => (),
          }
          continue;
        },
        "add" | "a" => {
          self.add_client();
          continue;
        },
        "edit" | "e" => {
          self.choose_edit_clients();
          continue;
        },
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.get_current_clients()
              .iter()
              .any(|&c| c.id == num) {
                println_err!("Please choose from among the listed clients, or add a client from another user.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_client(num) {
              Ok(_) => self.choose_client(),
              Err(e) => {
                println_err!("Unable to load client with id {}: {}", num, e);
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn select_client(&mut self) -> u32 {
    loop {
      let input = loop {
        self.display_select_clients();
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_client_get_id();
          match maybe_new_id {
            Some(new_id) => self.update_current_clients(new_id),
            None => (),
          }
          continue;
        },
        "add" | "a" => {
          self.add_client();
          continue;
        },
        "edit" | "e" => {
          self.choose_edit_clients();
          continue;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.get_current_clients()
              .iter()
              .any(|&c| c.id == num) {
                println_err!("Please choose from among the listed clients, or add a client from another user.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_client(num) {
              Ok(_) => return num,
              Err(e) => {
                println_err!("Unable to load client with id {}: {}", num, e);
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
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
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        },
        _ => {
          match input.parse() {
            Ok(num) => {
              if !self.get_current_clients().iter().any(|c| c.id == num) {
                println_err!("Please choose from among the listed IDs.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
              match self.load_client(num) {
                Ok(_) => self.choose_client(),
                Err(e) => {
                  println_err!("Failed to load client with ID {}: {}", num, e);
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              }
            },
            Err(e) => {
              println_err!("Failed to read input '{}' as a number: {}", input, e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        }
      }
    }
  }
  fn display_specify_clients(&self, purpose: String) {
    let heading = format!(" Choose client for {} ", purpose);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^96}", heading);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^40}", " ID ", " NAME ", " DOB ");
    match self.foreign_key.get("current_user_id") {
      Some(_) => {
        for c in self.get_current_clients() {
          println_on_bg!(
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
    println_on_bg!("{:-^96}", "-");
    println_inst!("| {} | {} | {} | {}", "Choose client by ID.", "NEW / N: new client", "ADD / A: Add from other user", "QUIT / Q: quit menu");
  }
  fn specify_client(&mut self, purpose: String) -> Option<u32> {
    let id: u32 = loop {
      let input = loop {
        self.display_specify_clients(purpose.clone());
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_client_get_id();
          match maybe_new_id {
            Some(new_id) => self.update_current_clients(new_id),
            None => (),
          }
          continue;
        },
        "add" | "a" => {
          self.add_client();
          continue;
        },
        "quit" | "q" => return None,
        _ => match input.parse() {
          Ok(num) => {
            if !self.get_current_clients()
              .iter()
              .any(|c| c.id == num) {
                println_err!("Please choose from among the listed clients.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              } else {
                match self.load_client(num) {
                  Ok(_) => {
                    break num;
                  }
                  Err(e) => {
                    println_err!("Unable to load client with id {}: {}", num, e);
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                  }
                }
              }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    };
    Some(id)
  }
  fn choose_client(&mut self) {
    loop {
      self.display_client();
      println_inst!("| {} | {} | {}", "EDIT / E: edit client", "DELETE: delete client", "COLLATERAL / CO: view/edit client collaterals");
      println_inst!("| {} | {}", "GOALS / G: View and edit client goals", "QUIT / Q: quit menu");
      let mut choice = String::new();
      let read_attempt = io::stdin().read_line(&mut choice);
      let input = match read_attempt {
        Ok(_) => choice.to_ascii_lowercase(),
        Err(e) => {
          println_err!("Could not read input; try again ({}).", e);
          continue;
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          self.foreign_key.remove("current_client_id");
          break;
        }
        "delete" | "d" => {
          self.choose_delete_client();
          break;
        }
        "edit" | "e" => {
          self.choose_edit_client();
        }
        "collateral" | "collat" | "col" | "co" => {
          self.choose_client_collaterals();
        }
        "goals" | "g" => {
          self.choose_client_goals();
        }
        _ => println_err!("Invalid command."),
      }
    }
  }
  fn create_client_get_id(&mut self) -> Option<u32> {
    let client = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        print_inst!("Enter 'CANCEL' at any time to cancel.");
        print!("\n");
        print_inst!("Enter client's first name.");
        print!("\n");
        let first_name_attempt = io::stdin().read_line(&mut first_name_choice);
        match first_name_attempt {
          Ok(_) => break String::from(first_name_choice.trim()),
          Err(e) => {
            println_err!("Invalid first name: {}", e);
            continue;
          }
        };
      };
      if first_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let last_name = loop {
        let mut last_name_choice = String::new();
        println_inst!("Enter client's last name.");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println_err!("Invalid last name: {}", e);
            continue;
          }
        };
      };
      if last_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let dob: NaiveDate = loop {
        let birth_year = loop {
          let mut birth_year_choice = String::new();
          println_inst!("Enter client's birth year.");
          let birth_year_attempt = io::stdin().read_line(&mut birth_year_choice);
          let birth_year_attempt = match birth_year_attempt {
            Ok(_) => {
              if birth_year_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                return None;
              }
              birth_year_choice.trim().parse()
            },
            Err(e) => {
              println_err!("Invalid birth year: {}", e);
              continue;
            }
          };
          let birth_year_input = match birth_year_attempt {
            Ok(val) => val,
            Err(e) => {
              println_err!("Invalid birth year: {}", e);
              continue;
            }
          };
          if birth_year_input > 9999 || birth_year_input < 1000 {
            println_err!("Please enter a valid year.");
            continue;
          }
          break birth_year_input;
        };
        let birth_month = loop {
          let mut birth_month_choice = String::new();
          println_inst!("Enter client's birth month as a decimal number (1-12).");
          let birth_month_attempt = io::stdin().read_line(&mut birth_month_choice);
          let birth_month_attempt = match birth_month_attempt {
            Ok(_) => {
              if birth_month_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                return None;
              }
              birth_month_choice.trim().parse()
            },
            Err(e) => {
              println_err!("Invalid birth month: {}", e);
              continue;
            }
          };
          let birth_month_input = match birth_month_attempt {
            Ok(val) => val,
            Err(e) => {
              println_err!("Invalid birth month: {}", e);
              continue;
            }
          };
          if birth_month_input > 12 || birth_month_input < 1 {
            println_err!("Please enter a valid month using decimal numbers 1-12.");
            continue;
          }
          break birth_month_input;
        };
        let birth_day = loop {
          let mut birth_day_choice = String::new();
          println_inst!("Enter client's birth day as a decimal number (1-31).");
          let birth_day_attempt = io::stdin().read_line(&mut birth_day_choice);
          let birth_day_attempt = match birth_day_attempt {
            Ok(_) => {
              if birth_day_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                return None;
              }
              birth_day_choice.trim().parse()
            },
            Err(e) => {
              println_err!("Invalid birth day: {}", e);
              continue;
            }
          };
          let birth_day_input = match birth_day_attempt {
            Ok(val) => val,
            Err(e) => {
              println_err!("Invalid birth day: {}", e);
              continue;
            }
          };
          if birth_day_input > 31 || birth_day_input < 1 {
            println_err!("Please enter a valid day using decimal numbers 1-12.");
            continue;
          }
          break birth_day_input;
        };

        match NaiveDate::from_ymd_opt(birth_year, birth_month, birth_day) {
          Some(date) => break date,
          None => {
            println_err!(
              "{}-{}-{} does not appear to be a valid date. Please try again.",
              birth_year, birth_month, birth_day
            );
            continue;
          }
        };
      };

      let pronouns = 'pronouns: loop {
        match self.choose_pronouns_option() {
          Some(p) => break p,
          None => {
            loop {
              println_yel!("Cancel? (Y/N)");
              let mut cancel = String::new();
              let cancel_attempt = io::stdin().read_line(&mut cancel);
              match cancel_attempt {
                Ok(_) => match &cancel.trim().to_lowercase()[..] {
                  "yes" | "y"  => return None,
                  "no" | "n" | "cancel" => continue 'pronouns,
                  _ => {
                    println_err!("Please choose either 'yes/y' or 'no/n'.");
                    continue;
                  }
                },
                Err(e) => {
                  println_err!("Failed to read line: {}", e);
                  continue;
                }
              }
  
            }
          }
        } 
      };

      let client_attempt = self.generate_unique_new_client(first_name, last_name, dob, pronouns);
      match client_attempt {
        Ok(client) => break client,
        Err(error_hash) => {
          println_err!(
            "Client could not be generated. Errors: '{}'.",
            error_hash.keys().cloned().collect::<Vec<String>>().join(", "),
          );
          match error_hash.get("duplicate") {
            Some(id_ref) => {
              match self.get_client_by_id(*id_ref) {
                Some(client) => {
                  let new_vec = vec![];
                  let current_clients = match self.foreign_key.get("current_user_id") {
                    Some(_) => self.get_current_clients().iter().map(|c| c.id).collect::<Vec<u32>>().clone(),
                    None => new_vec,
                  };
                  if !current_clients.iter().any(|c_id| c_id == id_ref ) {
                    let mut conf = String::new();
                    let choice = loop {
                      println_inst!("Would you like to use the existing record? (Y/N)");
                      let conf_attempt = io::stdin().read_line(&mut conf);
                      match conf_attempt {
                        Ok(_) => break String::from(conf.trim()),
                        Err(_) => {
                          println_err!("Failed to read input.");
                          continue;
                        }
                      }
                    };
                    match &choice.to_ascii_lowercase()[..] {
                      "yes" | "y" => {
                        let mut new_client = client.clone();
                        new_client.id = (self.clients.len() + 1) as u32;
                        break new_client;
                      }
                      "no" | "n" => continue,
                      _ => println_err!("Invalid command.")
                    }
                  }
                },
                None => {
                  thread::sleep(time::Duration::from_secs(1));
                  continue;
                }
              }
            },
            None => {
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        }
      }
    };

    let id = client.id;
    self.save_client(client);
    Some(id)
  }
  fn client_dup_id_option(&self, first_name: &str, last_name: &str, dob: &NaiveDate) -> Option<u32> {
    let names_and_dobs: Vec<(&str, &str, &NaiveDate, u32)> = self
      .clients
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.dob, c.id))
      .collect();

    match names_and_dobs
      .iter()
      .find(|(f, l, d, _)| f == &first_name && l == &last_name && d == &dob) {
        Some(name_and_dob_tup) => Some(name_and_dob_tup.3),
        None => None,
      }
  }
  fn generate_unique_new_client(
    &mut self,
    first_name: String,
    last_name: String,
    dob: NaiveDate,
    pronouns: u32,
  ) -> Result<Client, HashMap<String, u32>> {
    if first_name.contains(" | ") || last_name.contains(" | ") {
      return Err([(String::from("invalid character string: ' | ' "), 0)].iter().cloned().collect::<HashMap<String, u32>>());
    }
    
    let id: u32 = self.clients.len() as u32 + 1;
    
    match self.client_dup_id_option(&first_name, &last_name, &dob ) {
      Some(dup_id) => Err([(String::from("duplicate"), dup_id)].iter().cloned().collect::<HashMap<String, u32>>()),
      None => Ok(Client::new(id, first_name, last_name, dob, pronouns, vec![]))
    }
  }
  pub fn read_clients(filepath: &str) -> Result<Vec<Client>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
    }

    let mut clients: Vec<Client> = vec![];

    for line in lines {
      let line_string = line?;

      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      if values.len() < 6 {
        return Err(Error::new(ErrorKind::Other, "Failed to read clients from filepath."));
      }

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
    Ok(clients)
  }
  pub fn write_clients(&self) -> std::io::Result<()> {
    let mut lines = String::from("##### clients #####\n");
    for c in &self.clients {
      lines.push_str(&c.to_string()[..]);
    }
    lines.push_str("##### clients #####");
    let mut file = File::create(self.filepaths["client_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn save_client(&mut self, client: Client) {
    let pos = self.clients.binary_search_by(|c| c.id.cmp(&client.id) ).unwrap_or_else(|e| e);
    self.clients.insert(pos, client);
    self.write_to_files();
  }
  fn update_current_clients(&mut self, id: u32) {
    self.current_user_mut().foreign_keys.get_mut("client_ids").unwrap().push(id);
  }
  fn choose_edit_client(&mut self) {
    loop {
      self.display_client();
      println_inst!("| {} | {} | {} | {}", "FIRST / F: edit first name", "LAST / L: edit surname", "PRNS / P: edit pronouns", "QUIT / Q: quit menu");
      let mut field_to_edit = String::new();
      let input_attempt = io::stdin().read_line(&mut field_to_edit);
      match input_attempt {
        Ok(_) => (),
        Err(_) => {
          println_err!("Failed to read input. Please try again.");
          continue;
        }
      }
      field_to_edit = field_to_edit.to_ascii_lowercase().trim().to_string();
      match &field_to_edit[..] {
        "quit" | "q" => {
          break ();
        }
        _ => (),
      }
      match &field_to_edit[..] {
        "first" | "fst" | "f" | "1st" | "first name" => {
          println_inst!("Enter new first name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_client_first_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              continue;
            }
          }
        }
        "last" | "lst" | "l" | "last name" => {
          println_inst!("Enter new last name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_client_last_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
                thread::sleep(time::Duration::from_secs(1));
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        }
        "prns" | "p" | "pronouns" => {
          let maybe_pronouns = self.choose_pronouns_option();
          match maybe_pronouns {
            Some(p) => self.current_client_mut().pronouns = p,
            None => (),
          }
        }
        _ => {
          println_err!("Invalid entry.");
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
      println_yel!("Are you sure you want to delete this client?");
      println_inst!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
      let mut confirm = String::new();
      let input_attempt = io::stdin().read_line(&mut confirm);
      let command = match input_attempt {
        Ok(_) => confirm.trim().to_string(),
        Err(e) => {
          println_err!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &command.to_ascii_lowercase()[..] {
        "yes" | "y" => {
          self.delete_current_client();
          self.reindex_clients();
          self.write_to_files();
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
    println_on_bg!("{:-^114}", "-");
    println_on_bg!("{:-^114}", " DELETE CLIENT ");
    println_on_bg!("{:-^114}", "-");
    println_on_bg!(
      "{:-^20} | {:-^20} | {:-^40} | {:-^25}",
      "First name", "Last name", "DOB", "Collateral records",
    );
    println_on_bg!(
      "{: ^20} | {: ^20} | {: <12} {: >26} | {: ^25}",
      self.current_client().first_name,
      self.current_client().last_name,
      self.current_client().fmt_dob(),
      self.current_client().fmt_date_of_birth(),
      self.get_current_collaterals().len(),
    );
    println_on_bg!("{:-^114}", "-");
  }
  fn delete_current_client(&mut self) {
    let current_collaterals = self.get_current_collaterals().iter().map(|co| co.id ).collect::<Vec<u32>>();
    for co_id in current_collaterals {
      match self.load_collateral(co_id) {
        Err(_) => panic!("Failed to delete collateral for current client."),
        Ok(_) => self.delete_current_collateral(),
      }
    }
    let new_collateral_id = self.reindex_collaterals(None);
    match new_collateral_id {
      _ => (),
    }
    let current_goals = self.current_client_goals().iter().map(|co| co.id ).collect::<Vec<u32>>();
    for g_id in current_goals {
      match self.load_goal(g_id) {
        Err(_) => panic!("Failed to delete goal for current client."),
        Ok(_) => self.delete_current_goal(),
      }
    }
    self.reindex_goals();
    let current_note_days = self.current_client_note_days().iter().map(|co| co.id ).collect::<Vec<u32>>();
    for nd_id in current_note_days {
      match self.load_note_day(nd_id) {
        Err(_) => panic!("Failed to delete note day for current client."),
        Ok(_) => self.delete_current_note_day(),
      }
    }
    let new_note_day_id = self.reindex_note_days(None);
    match new_note_day_id {
      _ => (),
    }
    let id = self.foreign_key.get("current_client_id").unwrap().to_owned();
    self.delete_from_blanks(String::from("client"), id);
    for u in &mut self.users {
      let mut new_ids = u.foreign_keys["client_ids"].clone();
      new_ids.retain(|co_id| co_id != &id );
      u.foreign_keys.insert(String::from("client_ids"), new_ids);
    }
    self.clients.retain(|c| c.id != id);
    self.foreign_key.remove("current_client_id");
    self.foreign_key.remove("current_collateral_id");
  }
  fn reindex_clients(&mut self) {
    let mut i: u32 = 1;
    let mut new_note_days: Vec<NoteDay> = self.note_days.clone();
    let mut new_notes: Vec<Note> = self.notes.clone();
    let mut changes: HashMap<u32, u32> = HashMap::new();
    for mut c in &mut self.clients {
      for nd in &mut new_note_days {
        if nd.foreign_key["client_id"] == c.id {
          nd.foreign_key.insert(String::from("client_id"), i);
        }
      }
      for n in &mut new_notes {
        if n.foreign_key["client_id"] == c.id {
          n.foreign_key.insert(String::from("client_id"), i);
        }
      }
      if c.id == i {
        ()
      } else {
        changes.insert(c.id, i);
        c.id = i;
      }
      i += 1;
    }
    for u in &mut self.users {
      let old_ids = u.foreign_keys["client_ids"].clone();
      let new_ids: Vec<u32> = old_ids.iter()
        .map(|co_id| match changes.get(co_id) { Some(val) => *val, None => *co_id } )
        .collect();
      u.foreign_keys.insert(String::from("client_ids"), new_ids);
    }
    self.note_days = new_note_days;
    self.notes = new_notes;
  }
  fn get_client_by_id(&self, id: u32) -> Option<&Client> {
    self.clients.iter().find(|c| c.id == id)
  }
  fn get_client_by_id_mut(&mut self, id: u32) -> Option<&mut Client> {
    self.clients.iter_mut().find(|c| c.id == id)
  }

  // collaterals
  fn current_collateral_mut(&mut self) -> &mut Collateral {
    let collateral_id = match self.foreign_key.get("current_collateral_id") {
      Some(id) => id,
      None => panic!("There is no current collateral selected."),
    };
    let maybe_current: Option<&mut Collateral> = self.collaterals.iter_mut().find(|c| c.id == *collateral_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded collateral ID does not match any saved collaterals."),
    }
  }
  fn current_general_collateral_mut(&mut self) -> &mut Collateral {
    let collateral_id = match self.foreign_key.get("current_general_collateral_id") {
      Some(id) => id,
      None => panic!("There is no current general collateral selected."),
    };
    let maybe_current: Option<&mut Collateral> = self.general_collaterals.iter_mut().find(|c| c.id == *collateral_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded general collateral ID does not match any saved collaterals."),
    }
  }
  fn current_collateral(&self) -> &Collateral {
    let collateral_id = match self.foreign_key.get("current_collateral_id") {
      Some(id) => id,
      None => panic!("There is no current collateral selected."),
    };
    let maybe_current: Option<&Collateral> = self.collaterals.iter().find(|c| c.id == *collateral_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded collateral ID does not match any saved collaterals."),
    }
  }
  fn current_general_collateral(&self) -> &Collateral {
    let collateral_id = match self.foreign_key.get("current_general_collateral_id") {
      Some(id) => id,
      None => panic!("There is no current general collateral selected."),
    };
    let maybe_current: Option<&Collateral> = self.general_collaterals.iter().find(|c| c.id == *collateral_id);
    match maybe_current {
      Some(c) => c,
      None => panic!("The loaded general collateral ID does not match any saved general collaterals."),
    }
  }
  fn current_user_collaterals(&self) -> Vec<&Collateral> {
    let collats = self.collaterals
      .iter()
      .filter(|co|
        self.current_user().foreign_keys["collateral_ids"].iter().any(|co_id| &co.id == co_id )
      )
      .collect();
    collats
  }
  fn current_client_collaterals(&self) -> Vec<&Collateral> {
    let collats = self.collaterals
      .iter()
      .filter(|co|
        self.current_client().foreign_keys["collateral_ids"]
          .iter()
          .any(|co_id| co_id == &co.id )
      )
      .collect();
    collats
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
    } else if clients.len() == 0 {
      String::from("")
    } else {
      clients[0].full_name()
    }

  }
  fn display_client_collaterals(&self, selected: Option<Vec<u32>>) {
    let mut heading = String::from(" ");
    heading.push_str(&self.current_client().full_name()[..]);
    heading.push_str("'s Collaterals ");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^113}", heading);
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^10} | {:-<100}", " ID ", "Info ");
    match self.foreign_key.get("current_client_id") {
      Some(_) => {
        for c in self.get_current_collaterals() {
          match selected.clone() {
            None => {
              println_on_bg!(
                "{: ^10} | {: <100}",
                c.id,
                c.full_name_and_title(),
              );
            },
            Some(ids) => {
              if ids.iter().any(|id| *id == c.id ) {
                println_suc!(
                  "{: ^10} | {: <100}",
                  c.id,
                  c.full_name_and_title(),
                );
              } else {
                println_on_bg!(
                  "{: ^10} | {: <100}",
                  c.id,
                  c.full_name_and_title(),
                );
              }
            }
          }
        }
      }
      None => (),
    }
    println_on_bg!("{:-^113}", "-");
    println_inst!("| {} | {} | {}",
      "Enter ID to choose collateral.",
      "NEW / N: new collateral",
      "ADD / A: add from other client/user",
    );
    println_inst!("| {} | {}",
      "EDIT / E: edit",
      "QUIT / Q: quit menu",
    );
  }
  fn display_select_general_collaterals(&self, selected: Option<Vec<u32>>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^113}", " Select general collaterals ");
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^10} | {:-<100}", " ID ", "Info ");
    for c in self.general_collaterals.clone() {
      match selected.clone() {
        None => {
          println_on_bg!(
            "{: ^10} | {: <100}",
            c.id,
            c.full_name_and_title(),
          );
        },
        Some(ids) => {
          if ids.iter().any(|id| *id == c.id ) {
            println_suc!(
              "{: ^10} | {: <100}",
              c.id,
              c.full_name_and_title(),
            );
          } else {
            println_on_bg!(
              "{: ^10} | {: <100}",
              c.id,
              c.full_name_and_title(),
            );
          }
        }
      }
    }
    println_on_bg!("{:-^113}", "-");
    println_inst!("| {} | {} | {} | {}",
      "Enter ID to choose collateral.",
      "NEW / N: new general collateral",
      "EDIT / E: edit",
      "QUIT / Q: quit menu",
    );
  }
  fn get_current_collaterals(&self) -> Vec<&Collateral> {
    self.collaterals.iter().filter(|collateral| {
      self.current_client().foreign_keys["collateral_ids"]
        .iter()
        .any(|&id| id == collateral.id)
    })
      .collect()
  }
  fn get_owned_current_collaterals(&self) -> Vec<Collateral> {
    self.collaterals.iter().filter(|collateral| {
      self.current_client().foreign_keys["collateral_ids"]
        .iter()
        .any(|&id| id == collateral.id)
    })
      .cloned()
      .collect()
  }
  fn get_noncurrent_collaterals(&self) -> Vec<&Collateral> {
    self.collaterals.iter().filter(|collateral| {
      !self.current_client().foreign_keys["collateral_ids"]
        .iter()
        .any(|&id| id == collateral.id)
    })
      .collect()
  }
  fn display_edit_client_collaterals(&self) {
    let mut heading = self.current_client().first_name.clone();
    heading.push_str(" Edit ");
    heading.push_str(&self.current_client().last_name);
    heading.push_str("'s Collateral records");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^113}", heading);
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^10} | {:-<100}", " ID ", " Info ");
    match self.foreign_key.get("current_client_id") {
      Some(_) => {
        for c in self.get_current_collaterals() {
          println_on_bg!(
            "{: ^10} | {: <100}",
            c.id,
            c.full_name_and_title(),
          );
        }
      }
      None => (),
    }
    println_on_bg!("{:-^113}", "-");
    println_inst!("| {} | {}", "Enter ID of collateral record to edit.", "QUIT / Q: quit menu");
  }
  fn display_user_collaterals(&self) {
    let current = self.current_user();
    let heading = format!(
      " Collaterals for {} {}'s clients ",
      current.first_name,
      current.last_name,  
    );

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^166}", "-");
    println_on_bg!("{:-^166}", heading);
    println_on_bg!("{:-^166}", "-");
    println_on_bg!("{:-^5} | {:-<30} | {:-<62} | {:-<60}", " ID ", "Name ", "Title ", "Youth(s) ");
    println_on_bg!("{:-^166}", "-");
    
    for co in self.current_user_collaterals() {

      println_on_bg!(
        "{: ^5} | {: <30} | {: <62} | {: <60}",
        co.id,
        co.full_name(),
        co.title(),
        self.collateral_clients_string(co.id),
      );
    }
    println_on_bg!("{:-^166}", "-");
    println_inst!("| {} | {} | {}",
      "Enter ID to choose collateral.",
      "EDIT / E: edit",
      "NEW / N: new collateral",
    );
    println_inst!("| {} | {} | {}",
      "ADD / A: Add from other user/client",
      "GENERAL / G: View general collaterals",
      "QUIT / Q: quit menu"
    );
  }
  fn display_general_collaterals(&self) {
    let heading = String::from(" General collaterals for Wraparound youth ");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^116}", "-");
    println_on_bg!("{:-^116}", heading);
    println_on_bg!("{:-^116}", " These collaterals can be selected for any youth. ");
    println_on_bg!("{:-^116}", " (E.g. intake coordinators, insurance contacts, office managers extraordinaire.) ");
    println_on_bg!("{:-^116}", "-");
    println_on_bg!("{:-^10} | {:-^30} | {:-^70}", " ID ", " Name ", " Title ");

    for co in self.general_collaterals.clone() {
      println_on_bg!(
        "{: ^10} | {: <30} | {: <70}",
        co.id,
        co.full_name(),
        co.title(),
      );
    }
    println_on_bg!("{:-^116}", "-");
    println_inst!("| {} | {} | {} | {}",
      "Enter ID to choose collateral.",
      "NEW / N: new general collateral",
      "EDIT / E: edit",
      "QUIT / Q: quit menu"
    );
  }
  fn display_edit_general_collaterals(&self) {
    let heading = String::from(" Edit general collaterals for Wraparound youth ");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^116}", "-");
    println_on_bg!("{:-^116}", heading);
    println_on_bg!("{:-^116}", "-");
    println_on_bg!("{:-^10} | {:-<30} | {:-<70}", " ID ", "Name ", "Title ");

    for co in self.current_user_collaterals() {

      println_on_bg!(
        "{: ^10} | {:-<30} | {:-<70}",
        co.id,
        co.full_name(),
        co.title(),
      );
    }
    println_on_bg!("{:-^116}", "-");
    println_inst!("| {} | {}", "Enter ID of collateral to edit.", "QUIT / Q: quit menu");
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
    println_on_bg!("{:-^146}", "-");
    println_on_bg!("{:-^146}", heading);
    println_on_bg!("{:-^146}", "-");
    println_on_bg!("{:-^10} | {:-<30} | {:-<70} | {:-<27}", " ID ", "Name ", "Title ", "Youth(s) ");

    for co in self.current_user_collaterals() {

      println_on_bg!(
        "{: ^10} | {:-<30} | {:-<70} | {: <27}",
        co.id,
        co.full_name(),
        co.title(),
        self.collateral_clients_string(co.id),
      );
    }
    println_on_bg!("{:-^146}", "-");
    println_inst!("| {} | {}", "Enter ID of collateral to edit.", "QUIT / Q: quit menu");
  }
  fn display_collateral(&self) {
    let current = self.current_collateral();
    let c_string = format!("Collateral for {}", &self.collateral_clients_string(current.id));

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
    let display_direct = match current.indirect_support {
      true => "N",
      false => "Y",
    };


    let heading = format!(" {} ", current.full_name_and_title());

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^146}", "-");
    println_on_bg!("{:-^146}", heading);
    println_on_bg!("{:-^146}", "-");
    println_on_bg!("{:-^146}", c_string);
    if current.primary_contact {
      println_on_bg!("{: <146}", "*Primary contact");
    }
    if current.guardian {
      println_on_bg!("{: <146}", "*Guardian");
    }
    if current.care_plan_team {
      println_on_bg!("{: <146}", "*Care Plan Team member");
    }
    println_on_bg!("{:-^146}", "-");
    println_on_bg!(
      "{:-^96} | {:-^13} | {:-^27}",
      "-", "Support type", "-",
    );
    println_on_bg!(
      "{:-^20} | {:-^20} | {:-^20} | {:-^30} | {:-^5} | {:-^5} | {:-^27}",
      " Role/Title ",
      " First name ",
      " Last name ",
      " Institution ",
      " Nat ",
      " Dir ",
      " Pronouns ",
    );
    println_on_bg!(
      "{: ^20} | {: ^20} | {: ^20} | {: ^30} | {: ^5} | {: ^5} | {: ^27}",
      current.title,
      current.first_name,
      current.last_name,
      display_inst,
      display_type,
      display_direct,
      display_pronouns,
    );
    println_on_bg!("{:-^146}", "-");
  }
  fn display_general_collateral(&self) {
    let current = self.current_general_collateral();

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
    println_on_bg!("{:-^143}", "-");
    println_on_bg!("{:-^143}", " View general collateral record ");
    println_on_bg!("{:-^143}", "-");
    println_on_bg!("{:-^143}", "-");
    println_on_bg!(
      "{:-^102} | {:-^13} | {:-^25}",
      "-", "Support type", "-",
    );
    println_on_bg!(
      "{:-^20} | {:-^20} | {:-^20} | {:-^30} | {:-^5} | {:-^5} | {:-^25}",
      " First name ",
      " Last name ",
      " Role/Title ",
      " Institution ",
      " Nat ",
      " Dir ",
      " Pronouns ",
    );
    println_on_bg!(
      "{: ^20} | {: ^20} | {: ^20} | {: ^30} | {: ^5} | {: ^5} | {: ^25}",
      current.first_name,
      current.last_name,
      current.title,
      display_inst,
      display_type,
      display_indirect,
      display_pronouns,
    );
    println_on_bg!("{:-^143}", "-");
    println_inst!("| {} | {} | {}", "EDIT / E: edit collateral", "DELETE: delete collateral", "QUIT / Q: quit menu");
  }
  fn display_edit_collateral(&self) {
    let current = self.current_collateral();
    let c_string = format!("Collateral for {}", &self.collateral_clients_string(current.id));

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
    let opposite_primary_command = match current.primary_contact {
      true => "PRIMARY: remove 'primary contact' label",
      false => "PRIMARY: add 'primary contact' label",
    };
    let opposite_guardian_command = match current.guardian {
      true => "GUARDIAN: remove 'guardian' label",
      false => "GUARDIAN: add 'guardian' label",
    };
    let opposite_cpt_command = match current.care_plan_team {
      true => "CPT: remove 'Care Plan Team member' label",
      false => "CPT: add 'Care Plan Team member' label",
    };
    let heading = format!(" Edit collateral: {} ", current.full_name_and_title());
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^146}", "-");
    println_on_bg!("{:-^146}", heading);
    println_on_bg!("{:-^146}", "-");
    println_on_bg!("{:-^146}", c_string);
    if current.primary_contact {
      println_on_bg!("{: <146}", "*Primary contact");
    }
    if current.guardian {
      println_on_bg!("{: <146}", "*Guardian");
    }
    if current.care_plan_team {
      println_on_bg!("{: <146}", "*Care Plan Team member");
    }
    println_on_bg!("{:-^146}", "-");
    println_on_bg!(
      "{:-^99} | {:-^13} | {:-^27}",
      "-", "Support type", "-",
    );
    println_on_bg!(
      "{:-^20} | {:-^20} | {:-^20} | {:-^30} | {:-^5} | {:-^5} | {:-^27}",
      " Role/Title ",
      " First name ",
      " Last name ",
      " Institution ",
      " Nat ",
      " Dir ",
      " Pronouns ",
    );
    println_on_bg!(
      "{: ^20} | {: ^20} | {: ^20} | {: ^30} | {: ^5} | {: ^5} | {: ^27}",
      current.title,
      current.first_name,
      current.last_name,
      display_inst,
      display_type,
      display_indirect,
      display_pronouns,
    );
    println_on_bg!("{:-^146}", "-");
    println_inst!(
      "| {} | {} | {}",
      "FIRST / F: edit first name",
      "LAST / L: edit surname",
      "TITLE / T: edit title/role",
    );
    println_inst!(
      "| {} | {} | {}",
      "INST / I: edit institution",
      "PRNS / P: edit pronouns",
      "QUIT / Q: quit menu",
    );
    println_inst!(
      "| {} | {}",
      opposite_type_command,
      opposite_indirect_command,
    );
    if current.support_type == Natural {
      println_inst!(
        "| {}",
        opposite_primary_command,
      );
      println_inst!(
        "| {}",
        opposite_guardian_command,
      );
      println_inst!(
        "| {}",
        opposite_cpt_command,
      );
    } else {
      println_inst!(
        "| {}",
        opposite_cpt_command,
      );
    }
    println_on_bg!("{:-^146}", "-");
  }
  fn display_edit_general_collateral(&self) {
    let current = self.current_general_collateral();

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

    let (display_indirect, opposite_indirect_command) = match current.indirect_support {
      true => ("N", "DIRECT: Change to direct support (e.g., 'for youth')"),
      false => ("Y", "INDIRECT: Change to indirect support (e.g., not 'for youth')"),
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^165}", "-");
    println_on_bg!("{:-^165}", " Edit collateral record ");
    println_on_bg!("{:-^165}", "-");
    println_on_bg!(
      "{:-^20} | {:-^20} | {:-^30} | {:-^25} | {:-^50} | {:-^5}",
      " First name ", " Last name ", " Pronouns ", " Role/Title ", " Institution ", " Dir "
    );
    println_on_bg!(
      "{: ^20} | {: ^20} | {: ^25} | {: ^30} | {: ^50} | {: ^5}",
      current.first_name,
      current.last_name,
      display_pronouns,
      current.title,
      display_inst,
      display_indirect,
    );
    println_on_bg!("{:-^165}", "-");
    println_inst!(
      "| {} | {} | {}",
      "FIRST / F: edit first name",
      "LAST / L: edit surname",
      "TITLE / T: edit title/role",
    );
    println_inst!(
      "| {} | {} | {}",
      "INST / I: edit institution",
      "PRNS / P: edit pronouns",
      "QUIT / Q: quit menu"
    );
    println_inst!(
      "| {}",
      opposite_indirect_command,
    );
    println_on_bg!("{:-^165}", "-");
  }
  fn display_add_collateral(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^113}", " Other collateral records ");
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^10} | {:-<100}", " ID ", "Info ");
    match self.foreign_key.get("current_client_id") {
      Some(_) => {
        for c in self.get_noncurrent_collaterals() {
          println_on_bg!(
            "{: ^10} | {: <100}",
            c.id,
            c.name_and_title(),
          );
        }
      }
      None => (),
    }
    println_on_bg!("{:-^113}", "-");
    println_inst!("| {} | {} | {}", "Enter ID to add collateral.", "NEW / N: new collateral", "QUIT / Q: quit menu");
  }
  fn load_collateral(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&Collateral> = self.collaterals.iter().find(|c| c.id == id);
    match current {
      Some(c) => {
        self.foreign_key.insert(String::from("current_collateral_id"), c.id);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to find collateral.",
      )),
    }
  }
  fn load_general_collateral(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&Collateral> = self.general_collaterals.iter().find(|c| c.id == id);
    match current {
      Some(c) => {
        self.foreign_key.insert(String::from("current_general_collateral_id"), c.id);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to find general collateral.",
      )),
    }
  }
  fn select_collaterals(&mut self) -> (String, Vec<u32>) {
    let mut collats: Vec<Collateral> = vec![];
    let mut general_collats: Vec<Collateral> = vec![];
    loop {
      let collat_ids = collats.clone().iter().map(|co| co.id ).collect::<Vec<u32>>();
      let blank_string = if collats.len() > 1 {
        format!(
          "{} {} {}",
          collats[..collats.len()-1].iter().map(|co| co.full_name_and_title()).collect::<Vec<String>>().join(", "),
          "and",
          collats[collats.len()-1].full_name_and_title()
        )
      } else if collats.len() > 0 {
        collats[0].full_name_and_title()
      } else {
        String::new()
      };
      let initial_input = loop {
        self.display_client_collaterals(Some(collat_ids.clone()));
        println!("");
        if &blank_string[..] != "" {
          println_suc!("Current content: {}", blank_string);
        }
        println!("");
        println_inst!("ALL: Select all currently shown collaterals.");
        println_inst!("GENERAL: Select from general/universal collaterals.");
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.trim().to_string(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      match &initial_input.to_ascii_lowercase()[..] {
        "new" | "n" => {
          let maybe_new_id = self.create_collateral_get_id();
          match maybe_new_id {
            _ => (),
          }
          continue;
        },
        "add" | "a" => {
          self.add_collateral();
          continue;
        },
        "general" | "g" => {
          loop {
            let general_collat_ids = general_collats.iter().map(|co| co.id ).collect::<Vec<u32>>();
            let general_input = loop {
              self.display_select_general_collaterals(Some(general_collat_ids.clone()));
              println_inst!("ALL: Select all");
              let mut choice = String::new();
              let read_attempt = io::stdin().read_line(&mut choice);
              match read_attempt {
                Ok(_) => break choice.trim().to_string(),
                Err(e) => {
                  println_err!("Could not read input; try again ({}).", e);
                  continue;
                }
              }
            };
            match &general_input.to_ascii_lowercase()[..] {
              "new" | "n" => {
                let maybe_new_id = self.create_general_collateral_get_id();
                match maybe_new_id {
                  Some(_) => (),
                  None => (),
                }
                continue;
              },
              "edit" | "e" => {
                self.choose_edit_general_collaterals();
                continue;
              },
              "all" | "a" => {
                general_collats = self.general_collaterals.clone();
                break;
              },
              "quit" | "q" => {
                break;
              },
              "" => {
                break;
              }
              _ => match general_input.parse() {
                Ok(num) => {
                  let collat = match self.general_collaterals.iter().find(|co| co.id == num) {
                    Some(co) => co.clone(),
                    None => continue,
                  };
                  if !general_collats.iter().any(|co| co == &collat ) {
                    general_collats.push(collat);
                    if general_collats.len() == self.general_collaterals.len() {
                      break;
                    }
                  } else {
                    general_collats.retain(|co| co != &collat );
                  }
                },
                Err(e) => {
                  println_err!("Invalid input: {}; error: {}", general_input, e);
                  thread::sleep(time::Duration::from_secs(3));
                  continue;
                }
              }
            }
          }
          for collat in general_collats.clone() {
            collats.push(collat.clone());
          }
          continue;
        }
        "edit" | "e" => {
          self.choose_edit_client_collaterals();
          continue;
        },
        "quit" | "q" => {
          break;
        },
        "all" => {
          for collat in collats.clone() {
            if !collats.clone().iter().any(|co| co.id == collat.id ) {
              collats.push(collat);
            }
          }
          break;
        },
        "" => {
          if collats.len() > 0 {
            return (blank_string, collat_ids);
          } else {
            println_err!("Please choose at least one collateral to add to the current blank.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
        },
        _ => {
          let selected_id_res: Result<u32, _> = initial_input.parse();
          match selected_id_res {
            Ok(num) => {
              let collat = match self.get_current_collaterals().iter().find(|co| co.id == num) {
                Some(co) => co.clone(),
                None => continue,
              };
              if !collats.iter().any(|co| co == collat ) {
                collats.push(collat.to_owned());
                if collats.len() == self.current_client_collaterals().len() {
                  break;
                }
              } else {
                collats.retain(|co| co != collat );
              }
            },
            Err(e) => {
              println_err!("Invalid input: {}; error: {}", initial_input, e);
              thread::sleep(time::Duration::from_secs(3));
              continue;
            }
          }
        }
      }
    }
    let blank_string = if collats.len() > 1 {
      format!(
        "{} {} {}",
        collats[..collats.len()-1].iter().map(|co| co.full_name_and_title()).collect::<Vec<String>>().join(", "),
        "and",
        collats[collats.len()-1].full_name_and_title()
      )
    } else if collats.len() > 0 {
      collats[0].full_name_and_title()
    } else {
      String::new()
    };
    let id_vec: Vec<u32> = collats.clone().iter().map(|co| co.id ).collect();
    (blank_string, id_vec)
  }
  fn choose_client_collaterals(&mut self) {
    loop {
      self.sort_data_by_dates();
      let input = loop {
        self.display_client_collaterals(None);
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_collateral_get_id();
          match maybe_new_id {
            _ => (),
          }
          continue;
        },
        "add" | "a" => {
          self.add_collateral();
          continue;
        },
        "edit" | "e" => {
          self.choose_edit_client_collaterals();
          continue;
        },
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.get_current_collaterals().iter().any(|co| co.id == num) {
              println_err!("Please select one of the listed IDs.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
            match self.load_collateral(num) {
              Ok(_) => self.choose_collateral(),
              Err(e) => {
                println_err!("Unable to load collateral with id {}: {}", num, e);
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
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
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.get_current_collaterals().iter().any(|co| co.id == num) {
              println_err!("Please select one of the listed IDs.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
            match self.load_collateral(num) {
              Ok(_) => self.choose_edit_collateral(),
              Err(e) => {
                println_err!("Unable to load collateral with id {}: {}", num, e);
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn choose_collaterals(&mut self) {
    loop {
      self.sort_data_by_dates();
      let input = loop {
        self.display_user_collaterals();
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_collateral_get_id();
          match maybe_new_id {
            Some(_) => (),
            None => (),
          }
          continue;
        },
        "edit" | "e" => {
          self.choose_edit_collaterals();
          continue;
        },
        "general" | "g" => {
          self.choose_general_collaterals();
          continue;
        }
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user().foreign_keys["collateral_ids"].iter().any(|n| n == &num) {
            println_err!("Please select one of the listed IDs.");
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
            match self.load_collateral(num) {
              Ok(_) => self.choose_collateral(),
              Err(e) => {
                println_err!("Unable to load collateral with id {}: {}", num, e);
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn choose_general_collaterals(&mut self) {
    loop {
      let input = loop {
        self.display_general_collaterals();
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_general_collateral_get_id();
          match maybe_new_id {
            Some(_) => (),
            None => (),
          }
          continue;
        },
        "edit" | "e" => {
          self.choose_edit_general_collaterals();
          continue;
        },
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.general_collaterals.iter().any(|gco| gco.id == num) {
              println_err!("Please select one of the listed IDs.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
            match self.load_general_collateral(num) {
              Ok(_) => self.choose_general_collateral(),
              Err(e) => {
                println_err!("Unable to load collateral with id {}: {}", num, e);
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
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
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user().foreign_keys["collateral_ids"].iter().any(|n| n == &num) {
            println_err!("Please select one of the listed IDs.");
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
            match self.load_collateral(num) {
              Ok(_) => self.choose_edit_collateral(),
              Err(e) => {
                println_err!("Unable to load collateral with id {}: {}", num, e);
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn choose_edit_general_collaterals(&mut self) {
    loop {
      let input = loop {
        self.display_edit_general_collaterals();
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice,
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.general_collaterals.iter().any(|gco| gco.id == num) {
            println_err!("Please select one of the listed IDs.");
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
            match self.load_general_collateral(num) {
              Ok(_) => self.choose_edit_general_collateral(),
              Err(e) => {
                println_err!("Unable to load collateral with id {}: {}", num, e);
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
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
      println_inst!(
        "| {} | {}",
        "EDIT / E: edit collateral",
        "DELETE: delete collateral",
      );
      println_inst!(
        "| {} | {}",
        "CLIENT: add collateral to another client",
        "QUIT / Q: quit menu"
      );
      let mut choice = String::new();
      let read_attempt = io::stdin().read_line(&mut choice);
      let input = match read_attempt {
        Ok(_) => choice.to_ascii_lowercase(),
        Err(e) => {
          println_err!("Could not read input; try again ({}).", e);
          continue;
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        }
        "client" | "c" => {
          let c_id = match self.specify_client(String::from("collateral")) {
            Some(id) => id,
            None => continue,
          };
          let collat_id = self.foreign_key["current_collateral_id"];
          if !self.get_client_by_id(c_id)
            .unwrap()
            .foreign_keys
            .get("collateral_ids")
            .unwrap()
            .iter()
            .any(|co_id| co_id == &collat_id ) {
              self.get_client_by_id_mut(c_id)
                .unwrap()
                .foreign_keys
                .get_mut("collateral_ids")
                .unwrap()
                .push(collat_id);
            }
        }
        "delete" | "d" => {
          if self.choose_delete_collateral() {
            break;
          }
        }
        "edit" | "e" => {
          self.choose_edit_collateral();
        }
        _ => {
          println_err!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
        }
      }
    }
  }
  fn choose_general_collateral(&mut self) {
    loop {
      self.display_general_collateral();
      let mut choice = String::new();
      let read_attempt = io::stdin().read_line(&mut choice);
      let input = match read_attempt {
        Ok(_) => choice.to_ascii_lowercase(),
        Err(e) => {
          println_err!("Could not read input; try again ({}).", e);
          continue;
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        }
        "delete" | "d" => {
          if self.choose_delete_general_collateral() {
            break;
          }
        }
        "edit" | "e" => {
          self.choose_edit_general_collateral();
        }
        _ => {
          println_err!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
        }
      }
    }
  }
  fn create_collateral_get_id(&mut self) -> Option<u32> {
    let collateral = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        print_inst!("Enter 'CANCEL' at any time to cancel.");
        print!("\n");
        print_inst!("Collateral's first name:");
        print!("\n");
        let first_name_attempt = io::stdin().read_line(&mut first_name_choice);
        match first_name_attempt {
          Ok(_) => break String::from(first_name_choice.trim()),
          Err(e) => {
            println_err!("Invalid first name: {}", e);
            continue;
          }
        };
      };
      if first_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let last_name = loop {
        let mut last_name_choice = String::new();
        println_inst!("Collateral's last name:");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println_err!("Invalid last name: {}", e);
            continue;
          }
        };
      };
      if last_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let title = loop {
        let mut title_choice = String::new();
        println_inst!("Enter collateral's role/title.");
        let title_attempt = io::stdin().read_line(&mut title_choice);
        match title_attempt {
          Ok(_) => break String::from(title_choice.trim()),
          Err(e) => {
            println_err!("Invalid title: {}", e);
            continue;
          }
        };
      };
      if title.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }

      let pronouns = 'pronouns: loop {
        match self.choose_pronouns_option() {
          Some(p) => break p,
          None => {
            loop {
              println_yel!("Cancel? (Y/N)");
              let mut cancel = String::new();
              let cancel_attempt = io::stdin().read_line(&mut cancel);
              match cancel_attempt {
                Ok(_) => match &cancel.trim().to_lowercase()[..] {
                  "yes" | "y" => return None,
                  "no" | "n" | "cancel" => continue 'pronouns,
                  _ => {
                    println_err!("Please choose either 'yes/y' or 'no/n'.");
                    continue;
                  }
                },
                Err(e) => {
                  println_err!("Failed to read input: {}.", e);
                  continue;
                }
              }
  
            }
          }
        } 
      };

      let (support_type, indirect_support, institution) = if FAMILY_ROLES.iter().any(|role| role == &title.to_ascii_lowercase() ) {
        (Natural, false, None)
      } else if FORMAL_ROLES.iter().any(|role| role == &title.to_ascii_lowercase() ) {
        let institution = loop {
          let mut institution_choice = String::new();
          println_inst!("Enter collateral's institution.");
          let institution_attempt = io::stdin().read_line(&mut institution_choice);
          match institution_attempt {
            Ok(_) => break String::from(institution_choice.trim()),
            Err(e) => {
              println_err!("Invalid institution: {}", e);
              continue;
            }
          }
        };
        (Formal, false, Some(institution))
      } else if INDIRECT_ROLES.iter().any(|role| role == &title.to_ascii_lowercase() ) {
        let institution = loop {
          let mut institution_choice = String::new();
          println_inst!("Enter collateral's institution.");
          let institution_attempt = io::stdin().read_line(&mut institution_choice);
          match institution_attempt {
            Ok(_) => break String::from(institution_choice.trim()),
            Err(e) => {
              println_err!("Invalid institution: {}", e);
              continue;
            }
          }
        };
        (Formal, true, Some(institution))
      } else {
        loop {
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          let mut support_type_choice = String::new();
          println_inst!("Natural or Formal support?");
          println_inst!("NATURAL / N | FORMAL / F");
          let support_type_attempt = io::stdin().read_line(&mut support_type_choice);
          let s = match support_type_attempt {
            Ok(_) => match support_type_choice.to_ascii_lowercase().trim() {
              "natural" | "nat" | "n" => Natural,
              "formal" | "form" | "f" => Formal,
              "cancel" => return None,
              _ => {
                println_err!("Please choose NATURAL or FORMAL.");
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
            Err(e) => {
              println_err!("Failed to read input: {}.", e);
              continue;
            }
          };
          let i: bool;
          match s {
            Natural => i = false,
            Formal => {
              print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
              let mut indirect_choice = String::new();
              println_inst!("Is this collateral a provider for the selected client?");
              println_inst!("YES / Y | NO / N");
              let indirect_attempt = io::stdin().read_line(&mut indirect_choice);
              i = match indirect_attempt {
                Ok(_) => match indirect_choice.to_ascii_lowercase().trim() {
                  "yes" | "y" => false,
                  "no" | "n" => true,
                  "cancel" => return None,
                  _ => {
                    println_err!("Please choose YES or NO.");
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                  }
                }
                Err(e) => {
                  println_err!("Failed to read input: {}.", e);
                  continue;
                }
              };
            }
          }
          let institution = loop {
            if s == Formal {
              let mut institution_choice = String::new();
              println_inst!("Enter collateral's institution.");
              let institution_attempt = io::stdin().read_line(&mut institution_choice);
              match institution_attempt {
                Ok(_) => {
                  if institution_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                    return None;
                  } else {
                    break Some(String::from(institution_choice.trim()));
                  }
                }
                Err(e) => {
                  println_err!("Invalid institution: {}", e);
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

      let guardian = match support_type {
        Formal => false,
        Natural => {
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          let mut guardian_choice = String::new();
          println_inst!("Is this collateral the youth's guardian?");
          println_inst!("YES / Y | NO / N");
          let guardian_attempt = io::stdin().read_line(&mut guardian_choice);
          match guardian_attempt {
            Ok(_) => match guardian_choice.to_ascii_lowercase().trim() {
              "yes" | "y" => true,
              "no" | "n" => false,
              "cancel" => return None,
              _ => {
                println_err!("Please choose YES or NO.");
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
            Err(e) => {
              println_err!("Failed to read input: {}.", e);
              continue;
            }
          }
        }
      };

      let primary_contact = if guardian { true } else { match support_type {
        Formal => false,
        Natural => {
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          let mut primary_choice = String::new();
          println_inst!("Is this collateral the primary contact for the family?");
          println_inst!("YES / Y | NO / N");
          let primary_attempt = io::stdin().read_line(&mut primary_choice);
          match primary_attempt {
            Ok(_) => match primary_choice.to_ascii_lowercase().trim() {
              "yes" | "y" => true,
              "no" | "n" => false,
              "cancel" => return None,
              _ => {
                println_err!("Please choose YES or NO.");
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
            Err(e) => {
              println_err!("Failed to read input: {}.", e);
              continue;
            }
          }
        }
      } };

      let care_plan_team =  if guardian || primary_contact { true } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut care_plan_team_choice = String::new();
        println_inst!("Is this collateral on the youth's Care Plan Team?");
        println_inst!("YES / Y | NO / N");
        let care_plan_team_attempt = io::stdin().read_line(&mut care_plan_team_choice);
        match care_plan_team_attempt {
          Ok(_) => match care_plan_team_choice.to_ascii_lowercase().trim() {
            "yes" | "y" => true,
            "no" | "n" => false,
            "cancel" => return None,
            _ => {
              println_err!("Please choose YES or NO.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
          Err(e) => {
            println_err!("Failed to read input: {}.", e);
            continue;
          }
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
        primary_contact,
        guardian,
        care_plan_team,
      );

      match collateral_attempt {
        Ok(collateral) => break collateral,
        Err(error_hash) => {
          println_err!(
            "Collateral could not be generated. Errors: '{}'.",
            error_hash.keys().cloned().collect::<Vec<String>>().join(", "),
          );
          match error_hash.get("duplicate") {
            Some(id_ref) => {
              match self.get_collateral_by_id(*id_ref) {
                Some(collat) => {
                  let new_vec = vec![];
                  let current_collats = match self.foreign_key.get("current_client_id") {
                    Some(_) => self.get_current_collaterals().iter().map(|co| co.id).collect::<Vec<u32>>().clone(),
                    None => new_vec.clone(),
                  };
                  if !self.current_user_collaterals().iter().any(|co| co.id == *id_ref ) || !current_collats.iter().any(|co_id| co_id == id_ref ) {
                    let mut conf = String::new();
                    let choice = loop {
                      println_inst!("Would you like to use the existing record? (Y/N)");
                      let conf_attempt = io::stdin().read_line(&mut conf);
                      match conf_attempt {
                        Ok(_) => break String::from(conf.trim()),
                        Err(_) => {
                          println_err!("Failed to read input.");
                          continue;
                        }
                      }
                    };
                    match &choice[..] {
                      "YES" | "yes" | "Y" | "y" => {
                        let mut c = collat.clone();
                        c.id = (self.collaterals.len() + 1) as u32;
                        break c;
                      }
                      "NO" | "no" | "N" | "n" => continue,
                      "CANCEL" | "Cancel" | "cancel" => return None,
                      _ => println_err!("Invalid command."),
                    }
                  }
                },
                None => {
                  thread::sleep(time::Duration::from_secs(1));
                  continue;
                }
              }
            },
            None => {
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        }
      }
    };
    let id = collateral.id;
    match self.foreign_key.get("current_client_id") {
      Some(_) => {
        self.update_current_collaterals(id);
      }
      None => {
        let c_id = loop {
          match self.specify_client(String::from("collateral")) {
            Some(id) => break id,
            None => {
              println_yel!("A collateral must be connected with a client. Cancel creating collateral ( Y / N )?");
              let mut answer = String::new();
              let answer_attempt = io::stdin().read_line(&mut answer);
              let final_answer = match answer_attempt {
                Ok(_) => answer.trim().to_ascii_lowercase(),
                Err(e) => {
                  println_err!("Failed to read line: {}", e);
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              };
              match &final_answer[..] {
                "yes" | "y" => return None,
                _ => continue,
              }
            }
          }
        };
        self.foreign_key.insert(String::from("current_client_id"), c_id);
        if !self.get_client_by_id(c_id).unwrap().foreign_keys.get("collateral_ids").unwrap().iter().any(|co_id| co_id == &id ) {
          self.get_client_by_id_mut(c_id).unwrap().foreign_keys.get_mut("collateral_ids").unwrap().push(id);
        }
        if !self.current_user().foreign_keys["collateral_ids"].iter().any(|co_id| co_id == &id ) {
          self.current_user_mut().foreign_keys.get_mut("collateral_ids").unwrap().push(id);
        }
      }
    }
    match self.get_collateral_by_id(id) {
      Some(_) => (),
      None => self.save_collateral(collateral),
    }
    Some(id)
  }
  fn create_general_collateral_get_id(&mut self) -> Option<u32> {
    let collateral = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        print_inst!("Enter 'CANCEL' at any time to cancel.");
        print!("\n");
        print_inst!("General collateral's first name:");
        print!("\n");
        let first_name_attempt = io::stdin().read_line(&mut first_name_choice);
        match first_name_attempt {
          Ok(_) => break String::from(first_name_choice.trim()),
          Err(e) => {
            println_err!("Invalid first name: {}", e);
            continue;
          }
        };
      };
      if first_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let last_name = loop {
        let mut last_name_choice = String::new();
        println_inst!("General collateral's last name:");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println_err!("Invalid last name: {}", e);
            continue;
          }
        };
      };
      if last_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let title = loop {
        let mut title_choice = String::new();
        println_inst!("Enter general collateral's role/title.");
        let title_attempt = io::stdin().read_line(&mut title_choice);
        match title_attempt {
          Ok(_) => break String::from(title_choice.trim()),
          Err(e) => {
            println_err!("Invalid title: {}", e);
            continue;
          }
        };
      };
      if title.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }

      let pronouns = 'pronouns: loop {
        match self.choose_pronouns_option() {
          Some(p) => break p,
          None => {
            loop {
              println_yel!("Cancel? (Y/N)");
              let mut cancel = String::new();
              let cancel_attempt = io::stdin().read_line(&mut cancel);
              match cancel_attempt {
                Ok(_) => match &cancel.trim().to_lowercase()[..] {
                  "yes" | "y" => return None,
                  "no" | "n" | "cancel" => continue 'pronouns,
                  _ => {
                    println_err!("Please choose either 'yes/y' or 'no/n'.");
                    continue;
                  }
                },
                Err(e) => {
                  println_err!("Failed to read input: {}.", e);
                  continue;
                }
              }
  
            }
          }
        } 
      };

      let support_type = Formal;

      let (indirect_support, institution) = if FAMILY_ROLES.iter().any(|role| role == &title.to_ascii_lowercase() ) {
        (false, None)
      } else if FORMAL_ROLES.iter().any(|role| role == &title.to_ascii_lowercase() ) {
        let institution = loop {
          let mut institution_choice = String::new();
          println_inst!("Enter general collateral's institution.");
          let institution_attempt = io::stdin().read_line(&mut institution_choice);
          match institution_attempt {
            Ok(_) => break String::from(institution_choice.trim()),
            Err(e) => {
              println_err!("Invalid institution: {}", e);
              continue;
            }
          }
        };
        (false, Some(institution))
      } else if INDIRECT_ROLES.iter().any(|role| role == &title.to_ascii_lowercase() ) {
        let institution = loop {
          let mut institution_choice = String::new();
          println_inst!("Enter general collateral's institution.");
          let institution_attempt = io::stdin().read_line(&mut institution_choice);
          match institution_attempt {
            Ok(_) => break String::from(institution_choice.trim()),
            Err(e) => {
              println_err!("Invalid institution: {}", e);
              continue;
            }
          }
        };
        (true, Some(institution))
      } else {
        loop {
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          let mut indirect_choice = String::new();
          println_inst!("Should this general collateral be displayed as if a direct provider for every youth, e.g., '[some role] for youth'?");
          println_inst!("YES / Y | NO / N");
          let indirect_attempt = io::stdin().read_line(&mut indirect_choice);
          let i = match indirect_attempt {
            Ok(_) => match indirect_choice.to_ascii_lowercase().trim() {
              "yes" | "y" => false,
              "no" | "n" => true,
              "cancel" => return None,
              _ => {
                println_err!("Please choose YES or NO.");
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
            Err(e) => {
              println_err!("Failed to read input: {}.", e);
              continue;
            }
          };
          let institution = loop {
            let mut institution_choice = String::new();
            println_inst!("Enter general collateral's institution.");
            let institution_attempt = io::stdin().read_line(&mut institution_choice);
            match institution_attempt {
              Ok(_) => {
                if institution_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                  return None;
                } else {
                  break Some(String::from(institution_choice.trim()));
                }
              }
              Err(e) => {
                println_err!("Invalid institution: {}", e);
                continue;
              }
            }
          };
          break (i, institution)
        }
      };

      let collateral_attempt = self.generate_unique_new_general_collateral(
        first_name,
        last_name,
        title,
        institution,
        pronouns,
        support_type,
        indirect_support,
        false, // general collateral will never be primary contact, guardian or care plan team member
        false, // general collateral will never be primary contact, guardian or care plan team member
        false, // general collateral will never be primary contact, guardian or care plan team member
      );
      
      match collateral_attempt {
        Ok(collateral) => break collateral,
        Err(error_hash) => {
          println_err!(
            "General collateral could not be generated. Errors: '{}'.",
            error_hash.keys().cloned().collect::<Vec<String>>().join(", "),
          );
          continue;
        }
      }
    };
    let id = collateral.id;
    match self.get_general_collateral_by_id(id) {
      Some(_) => (),
      None => self.save_general_collateral(collateral),
    }
    Some(id)
  }
  fn collateral_dup_id_option(&self, first_name: &str, last_name: &str, title: &str, institution: &Option<String>) -> Option<u32> {
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>, u32)> = self
      .collaterals
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.title[..], &c.institution, c.id))
      .collect();

    let maybe_id = match names_and_roles
      .iter()
      .find(|(f, l, t, i, _)| f == &first_name && l == &last_name && t == &title && i == &institution) {
        Some(name_and_role_tup) => Some(name_and_role_tup.4),
        None => None,
      };

    maybe_id

  }
  fn general_collateral_dup_id_option(&self, first_name: &str, last_name: &str, title: &str, institution: &Option<String>) -> Option<u32> {
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>, u32)> = self
      .general_collaterals
      .iter()
      .map(|c| (&c.first_name[..], &c.last_name[..], &c.title[..], &c.institution, c.id))
      .collect();

    let maybe_id = match names_and_roles
      .iter()
      .find(|(f, l, t, i, _)| f == &first_name && l == &last_name && t == &title && i == &institution) {
        Some(name_and_role_tup) => Some(name_and_role_tup.4),
        None => None,
      };

    maybe_id

  }
  fn generate_unique_new_collateral(
    &mut self,
    first_name: String,
    last_name: String,
    title: String,
    institution: Option<String>,
    pronouns: u32,
    support_type: SupportType,
    indirect_support: bool,
    primary_contact: bool,
    guardian: bool,
    care_plan_team: bool,
  ) -> Result<Collateral, HashMap<String, u32>> {

    if first_name.contains(" | ") || last_name.contains(" | ") || title.contains(" | ") {
      return Err([(String::from("invalid character string: ' | ' "), 0)].iter().cloned().collect::<HashMap<String, u32>>());
    }
    
    match institution.clone() {
      None => (),
      Some(s) => {
        if s.contains(" | ") {
          return Err([(String::from("invalid character string: ' | ' "), 0)].iter().cloned().collect::<HashMap<String, u32>>());
        }
      }
    }

    let id: u32 = self.collaterals.len() as u32 + 1;

    match self.collateral_dup_id_option(&first_name, &last_name, &title, &institution) {
      Some(match_id) => Err([(String::from("duplicate"), match_id)].iter().cloned().collect::<HashMap<String, u32>>()),
      None => Ok(Collateral::new(
        id,
        first_name,
        last_name,
        title,
        institution,
        pronouns,
        support_type,
        indirect_support,
        primary_contact,
        guardian,
        care_plan_team,
        Local::now().naive_local().date(),
      ))
    }

  }
  fn generate_unique_new_general_collateral(
    &mut self,
    first_name: String,
    last_name: String,
    title: String,
    institution: Option<String>,
    pronouns: u32,
    support_type: SupportType,
    indirect_support: bool,
    primary_contact: bool,
    guardian: bool,
    care_plan_team: bool,
  ) -> Result<Collateral, HashMap<String, u32>> {

    if first_name.contains(" | ") || last_name.contains(" | ") || title.contains(" | ") {
      return Err([(String::from("invalid character string: ' | ' "), 0)].iter().cloned().collect::<HashMap<String, u32>>());
    }
    
    match institution.clone() {
      None => (),
      Some(s) => {
        if s.contains(" | ") {
          return Err([(String::from("invalid character string: ' | ' "), 0)].iter().cloned().collect::<HashMap<String, u32>>());
        }
      }
    }

    let id: u32 = self.general_collaterals.len() as u32 + 1;

    match self.general_collateral_dup_id_option(&first_name, &last_name, &title, &institution) {
      Some(match_id) => Err([(String::from("duplicate"), match_id)].iter().cloned().collect::<HashMap<String, u32>>()),
      None => Ok(Collateral::new(
        id,
        first_name,
        last_name,
        title,
        institution,
        pronouns,
        support_type,
        indirect_support,
        primary_contact,
        guardian,
        care_plan_team,
        Local::now().naive_local().date(),
      ))
    }

  }
  pub fn read_collaterals(filepath: &str) -> Result<Vec<Collateral>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
    }

    let mut collaterals: Vec<Collateral> = vec![];

    for line in lines {
      let line_string = line?;
      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      if values.len() < 8 {
        return Err(Error::new(ErrorKind::Other, "Failed to read collaterals from filepath."));
      }

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
        _ => panic!("Invalid SupportType saved in file."),
      };
      let indirect_support = match &values[7][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid 'indirect support boolean' value stored in file."),
      };
      let primary_contact = match &values[8][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid 'primary_contact boolean' value stored in file."),
      };
      let guardian = match &values[9][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid 'guardian boolean' value stored in file."),
      };
      let care_plan_team = match &values[10][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid 'care plan team boolean' value stored in file."),
      };

      let date = if values.len() > 11 {
        let date_vec: Vec<i32> = match &values[11][..] {
          "" => vec![],
          _ => values[11]
            .split("-")
            .map(|val| val.parse().unwrap())
            .collect(),
        };

        let (year, month, day): (i32, u32, u32) = (date_vec[0], date_vec[1] as u32, date_vec[2] as u32);
        NaiveDate::from_ymd(year, month, day)
      } else {
        Local::now().naive_local().date()
      };

      let c = Collateral::new(
        id,
        first_name,
        last_name,
        title,
        institution,
        pronouns,
        support_type,
        indirect_support,
        primary_contact,
        guardian,
        care_plan_team,
        date,
      );
      collaterals.push(c);
    }
    Ok(collaterals)
  }
  pub fn read_general_collaterals(filepath: &str) -> Result<Vec<Collateral>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
    }

    let mut general_collaterals: Vec<Collateral> = vec![];

    for line in lines {
      let line_string = line?;
      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      if values.len() < 8 {
        return Err(Error::new(ErrorKind::Other, "Failed to read general_collaterals from filepath."));
      }

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
        _ => panic!("Invalid SupportType saved in file."),
      };
      let indirect_support = match &values[7][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid 'indirect support boolean' value stored in file."),
      };
      let primary_contact = match &values[8][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid 'primary_contact boolean' value stored in file."),
      };
      let guardian = match &values[9][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid 'guardian boolean' value stored in file."),
      };
      let care_plan_team = match &values[10][..] {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid 'care plan team boolean' value stored in file."),
      };

      let date = if values.len() > 11 {
        let date_vec: Vec<i32> = match &values[11][..] {
          "" => vec![],
          _ => values[11]
            .split("-")
            .map(|val| val.parse().unwrap())
            .collect(),
        };

        let (year, month, day): (i32, u32, u32) = (date_vec[0], date_vec[1] as u32, date_vec[2] as u32);
        NaiveDate::from_ymd(year, month, day)
      } else {
        Local::now().naive_local().date()
      };

      let c = Collateral::new(
        id,
        first_name,
        last_name,
        title,
        institution,
        pronouns,
        support_type,
        indirect_support,
        primary_contact,
        guardian,
        care_plan_team,
        date,
      );
      general_collaterals.push(c);
    }
    Ok(general_collaterals)
  }
  pub fn write_collaterals(&self) -> std::io::Result<()> {
    let mut lines = String::from("##### collaterals #####\n");
    for c in &self.collaterals {
      lines.push_str(&c.to_string()[..]);
    }
    lines.push_str("##### collaterals #####");
    let mut file = File::create(self.filepaths["collateral_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn write_general_collaterals(&self) -> std::io::Result<()> {
    let mut lines = String::from("##### global collaterals #####\n");
    for c in &self.general_collaterals {
      lines.push_str(&c.to_string()[..]);
    }
    lines.push_str("##### global collaterals #####");
    let mut file = File::create(self.filepaths["general_collateral_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  fn get_first_client_with_collat_id(&self, id: u32) -> Option<&Client> {
    self.clients.iter().find(|&c| c.foreign_keys["collateral_ids"].iter().any(|c_id| c_id == &id ))
  }
  fn sort_collaterals(&mut self) {

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

    let first_client_ids: Vec<u32> = self.collaterals
      .iter()
      .map(|co| match self.get_first_client_with_collat_id(co.id) {Some(c)=>c.id,None=>0 as u32} )
      .collect();

    let mut sorted_collaterals: Vec<(usize, &Collateral)> = self.collaterals
      .iter()
      .enumerate()
      .collect();
      
      sorted_collaterals.sort_by(| (i_a, _), (i_b, _)| first_client_ids[*i_a].cmp(&first_client_ids[*i_b]) );
      
    let mut new_collaterals: Vec<Collateral> = sorted_collaterals
      .iter()
      .map(|(_, co)| (*co).clone() )
      .collect();

    new_collaterals.sort_by(|a, b| b.date.cmp(&a.date) );
    self.collaterals = new_collaterals;
  }
  fn sort_general_collaterals(&mut self) {

    // sort by institution

    self.general_collaterals.sort_by(|a, b|
      match (a.institution.as_ref(), b.institution.as_ref()) {
        (Some(a_i), Some(b_i)) => a_i.cmp(&b_i),
        _ => a.institution.cmp(&b.institution),
      }
    );
    self.general_collaterals.sort_by(|a, b|
      match (a.institution.as_ref(), b.institution.as_ref()) {
        (Some(a_i), Some(b_i)) => a_i.cmp(&b_i),
        _ => b.date.cmp(&a.date),
      }
    );

  }
  pub fn save_collateral(&mut self, collateral: Collateral) {
    self.collaterals.push(collateral);
    self.sort_collaterals();
    self.write_to_files();
  }
  pub fn save_general_collateral(&mut self, collateral: Collateral) {
    self.general_collaterals.push(collateral);
    self.sort_general_collaterals();
    self.reindex_general_collaterals();
    self.write_to_files();
  }
  fn update_current_collaterals(&mut self, id: u32) {
    if !self.current_client().foreign_keys["collateral_ids"].iter().any(|co_id| co_id == &id ) {
      self.current_client_mut().foreign_keys.get_mut("collateral_ids").unwrap().push(id);
    }
    if !self.current_user().foreign_keys["collateral_ids"].iter().any(|co_id| co_id == &id ) {
      self.current_user_mut().foreign_keys.get_mut("collateral_ids").unwrap().push(id);
    }
    self.sort_collaterals();
  }
  fn add_collateral(&mut self) {
    loop {
      self.display_add_collateral();
      let input = loop {
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        },
        "new" | "n" => {
          let maybe_new_id = self.create_collateral_get_id();
          match maybe_new_id {
            Some(_) => {
              println_suc!("Collateral added to client '{}'.", self.current_client().full_name());
              thread::sleep(time::Duration::from_secs(2));
              break;
            },
            None => continue,
          }
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.get_noncurrent_collaterals()
              .iter()
              .any(|co| co.id == num) {
              println_err!("Please select from the available choices.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            } else {
              match self.load_collateral(num) {
                Ok(_) => {
                  self.update_current_collaterals(num);
                  self.write_to_files();
                  break;
                }
                Err(e) => {
                  println_err!("Unable to load collateral with id {}: {}", num, e);
                  continue;
                }
              }
            }
          },
          Err(e) => {
            println_err!("Failed to read input as a number: {}.", e);
            continue;
          }
        }
      }
    }
  }
  fn choose_edit_collateral(&mut self) {
    loop {
      self.display_edit_collateral();
      let mut field_to_edit = String::new();
      let input_attempt = io::stdin().read_line(&mut field_to_edit);
      match input_attempt {
        Ok(_) => (),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
          continue;
        }
      }
      field_to_edit = field_to_edit.to_ascii_lowercase().trim().to_string();
      match &field_to_edit[..] {
        "quit" | "q" => {
          break ();
        }
        _ => (),
      }
      match &field_to_edit[..] {
        "first" | "fst" | "f" | "1st" | "first name" => {
          println_inst!("Enter new first name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_collateral_first_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              continue;
            }
          }
        },
        "last" | "lst" | "l" | "last name" => {
          println_inst!("Enter new last name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_collateral_last_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
                thread::sleep(time::Duration::from_secs(1));
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        },
        "title" | "t" => {
          println_inst!("Enter new title:");
          let mut title_choice = String::new();
          let title_attempt = io::stdin().read_line(&mut title_choice);
          match title_attempt {
            Ok(_) => match self.change_collateral_title(title_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
                thread::sleep(time::Duration::from_secs(1));
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        },
        "institution" | "inst" | "i" => {
          if self.current_collateral().support_type == Natural {
            println_err!("Unable to add institution for a natural support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          println_inst!("Enter new institution or NONE to remove:");
          let mut inst_choice = String::new();
          let inst_attempt = io::stdin().read_line(&mut inst_choice);
          match inst_attempt {
            Ok(_) => match (self.current_collateral().institution.as_ref(), &inst_choice.trim()[..]) {
              (None, "NONE") => {
                println_err!("Collateral currently has no institution.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              },
              (Some(_), "NONE") => {
                match self.change_collateral_institution(None) {
                  Ok(_) => (),
                  Err(e) => {
                    println_err!("Error: {}", e);
                    thread::sleep(time::Duration::from_secs(1));
                  }
                }
              },
              (Some(i), inst_choice_slice) => {
                if &i[..] == inst_choice_slice {
                  println_err!("Collateral institution already matches.");
                  thread::sleep(time::Duration::from_secs(2));
                } else {
                  let new_inst = String::from(inst_choice.trim());
                  match self.change_collateral_institution(Some(new_inst)) {
                    Ok(_) => (),
                    Err(e) => {
                      println_err!("Error: {}", e);
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
                    println_err!("Error: {}", e);
                    thread::sleep(time::Duration::from_secs(1));
                  }
                }
              }
            }
            Err(e) => {
              println_err!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        },
        "prns" | "p" | "pronouns" => {
          let maybe_pronouns = self.choose_pronouns_option();
          match maybe_pronouns {
            Some(p) => self.current_collateral_mut().pronouns = p,
            None => (),
          }
        },
        "formal" => {
          if self.current_collateral().support_type == Formal {
            println_err!("Collateral is already a formal support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            loop {
              println_inst!("Institution required for formal support. Enter institution below, or 'NONE' to cancel:");
              let mut inst_choice = String::new();
              let inst_attempt = io::stdin().read_line(&mut inst_choice);
              match inst_attempt {
                Ok(_) => match &inst_choice.trim()[..] {
                  "NONE" => break,
                  _ => {
                    let current = self.current_collateral();
                    match self.collateral_dup_id_option(
                      &current.first_name,
                      &current.last_name,
                      &current.title,
                      &Some(String::from(inst_choice.trim()))
                    ) {
                      Some(_) => {
                        println_err!("A collateral already exists with that information. Consider selecting ADD from the collateral menu.");
                        thread::sleep(time::Duration::from_secs(3));
                        break;
                      },
                      None => self.current_collateral_mut().institution = Some(inst_choice.trim().to_string()),
                    }
                  }
                },
                Err(e) => {
                  println_err!("Failed to read line: {}", e);
                  thread::sleep(time::Duration::from_secs(1));
                  continue;
                },
              }
              self.current_collateral_mut().support_type = Formal;
              break;
            }
          }
        },
        "natural" => {
          if self.current_collateral().support_type == Natural {
            println_err!("Collateral is already a natural support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            if self.current_collateral().institution != None {
              let current = self.current_collateral();
              match self.collateral_dup_id_option(
                &current.first_name,
                &current.last_name,
                &current.title,
                &None
              ) {
                Some(_) => {
                  println_err!("That action would create a duplicate record because another natural support has the same name and no institution.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                },
                None => {
                  println_yel!("Setting collateral as a natural support will remove the name of any associated institution.");
                  println_yel!("Proceed? (Y/N)");
                  let mut proceed_choice = String::new();
                  let proceed_attempt = io::stdin().read_line(&mut proceed_choice);
                  match proceed_attempt {
                    Ok(_) => match &proceed_choice.trim()[..] {
                      "Y" | "y" | "YES" | "Yes" | "yes" => (),
                      _ => {
                        println_yel!("Canceled.");
                        thread::sleep(time::Duration::from_secs(1));
                        continue;
                      }
                    }
                    Err(e) => {
                      println_err!("Failed to read input: {}.", e);
                      thread::sleep(time::Duration::from_secs(1));
                      continue;
                    }
                  }
                },
              }
            }
            self.current_collateral_mut().support_type = Natural;
            self.current_collateral_mut().indirect_support = false;
            self.current_collateral_mut().institution = None;
          }
        },
        "indirect" => {
          if self.current_collateral().indirect_support == true {
            println_err!("Collateral is already an indirect support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            self.current_collateral_mut().indirect_support = true;
          }
        },
        "direct" => {
          if self.current_collateral().indirect_support == false {
            println_err!("Collateral is already a direct support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            self.current_collateral_mut().indirect_support = false;
          }
        },
        "primary" => {
          if self.current_collateral().support_type == Formal {
            println_err!("A formal support cannot be the primary contact for a family.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          if self.current_collateral().primary_contact == false {
            self.current_collateral_mut().primary_contact = true;
          } else {
            self.current_collateral_mut().primary_contact = false;
          }
        },
        "guardian" => {
          if self.current_collateral().support_type == Formal {
            println_err!("A formal support cannot be a youth's guardian.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          if self.current_collateral().guardian == false {
            self.current_collateral_mut().guardian = true;
          } else {
            self.current_collateral_mut().guardian = false;
          }
        },
        "cpt" | "care plan team" | "care plan team member" => {
          if self.current_collateral().care_plan_team == false {
            self.current_collateral_mut().care_plan_team = true;
          } else {
            self.current_collateral_mut().care_plan_team = false;
          }
        },
        _ => {
          println_err!("Invalid entry.");
          thread::sleep(time::Duration::from_secs(1));
        }
      }
    }
  }
  fn choose_edit_general_collateral(&mut self) {
    loop {
      self.display_edit_general_collateral();
      let mut field_to_edit = String::new();
      let input_attempt = io::stdin().read_line(&mut field_to_edit);
      match input_attempt {
        Ok(_) => (),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
          continue;
        }
      }
      field_to_edit = field_to_edit.to_ascii_lowercase().trim().to_string();
      match &field_to_edit[..] {
        "quit" | "q" => {
          break ();
        }
        _ => (),
      }
      match &field_to_edit[..] {
        "first" | "fst" | "f" | "1st" | "first name" => {
          println_inst!("Enter new first name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_general_collateral_first_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              continue;
            }
          }
        },
        "last" | "lst" | "l" | "last name" => {
          println_inst!("Enter new last name:");
          let mut name_choice = String::new();
          let name_attempt = io::stdin().read_line(&mut name_choice);
          match name_attempt {
            Ok(_) => match self.change_general_collateral_last_name(name_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
                thread::sleep(time::Duration::from_secs(1));
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        },
        "title" | "t" => {
          println_inst!("Enter new title:");
          let mut title_choice = String::new();
          let title_attempt = io::stdin().read_line(&mut title_choice);
          match title_attempt {
            Ok(_) => match self.change_general_collateral_title(title_choice.trim()) {
              Ok(_) => (),
              Err(e) => {
                println_err!("Error: {}", e);
                thread::sleep(time::Duration::from_secs(1));
              }
            },
            Err(e) => {
              println_err!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        },
        "institution" | "inst" | "i" => {
          if self.current_general_collateral().support_type == Natural {
            println_err!("Unable to add institution for a natural support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          println_inst!("Enter new institution or NONE to remove:");
          let mut inst_choice = String::new();
          let inst_attempt = io::stdin().read_line(&mut inst_choice);
          match inst_attempt {
            Ok(_) => match (self.current_general_collateral().institution.as_ref(), &inst_choice.trim()[..]) {
              (None, "NONE") => {
                println_err!("General collateral currently has no institution.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              },
              (Some(_), "NONE") => {
                match self.change_general_collateral_institution(None) {
                  Ok(_) => (),
                  Err(e) => {
                    println_err!("Error: {}", e);
                    thread::sleep(time::Duration::from_secs(1));
                  }
                }
              },
              (Some(i), inst_choice_slice) => {
                if &i[..] == inst_choice_slice {
                  println_err!("General collateral institution already matches.");
                  thread::sleep(time::Duration::from_secs(2));
                } else {
                  let new_inst = String::from(inst_choice.trim());
                  match self.change_general_collateral_institution(Some(new_inst)) {
                    Ok(_) => (),
                    Err(e) => {
                      println_err!("Error: {}", e);
                      thread::sleep(time::Duration::from_secs(1));
                    }
                  }
                }
              },
              (None, &_) => {
                let new_inst = String::from(inst_choice.trim());
                match self.change_general_collateral_institution(Some(new_inst)) {
                  Ok(_) => (),
                  Err(e) => {
                    println_err!("Error: {}", e);
                    thread::sleep(time::Duration::from_secs(1));
                  }
                }
              }
            }
            Err(e) => {
              println_err!("Error: {}", e);
              thread::sleep(time::Duration::from_secs(1));
            }
          }
        },
        "prns" | "p" | "pronouns" => {
          let maybe_pronouns = self.choose_pronouns_option();
          match maybe_pronouns {
            Some(p) => self.current_general_collateral_mut().pronouns = p,
            None => (),
          }
        },
        "indirect" => {
          if self.current_general_collateral().indirect_support == true {
            println_err!("Collateral is already an indirect support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            self.current_general_collateral_mut().indirect_support = true;
          }
        },
        "direct" => {
          if self.current_general_collateral().indirect_support == false {
            println_err!("Collateral is already a direct support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            self.current_general_collateral_mut().indirect_support = false;
          }
        },
        _ => {
          println_err!("Invalid entry.");
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
  fn change_general_collateral_first_name(&mut self, new_name: &str) -> Result<(), String> {
    let current = self.current_general_collateral();
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .general_collaterals
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
      self.current_general_collateral_mut().first_name = String::from(new_name);
      Ok(())
    };
    result
  }
  fn change_general_collateral_last_name(&mut self, new_name: &str) -> Result<(), String> {
    let current = &self.current_general_collateral();
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .general_collaterals
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
      self.current_general_collateral_mut().last_name = String::from(new_name);
      Ok(())
    };
    result
  }
  fn change_general_collateral_title(&mut self, new_title: &str) -> Result<(), String> {
    let current = &self.current_general_collateral();
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .general_collaterals
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
      self.current_general_collateral_mut().title = String::from(new_title);
      Ok(())
    };
    result
  }
  fn change_general_collateral_institution(&mut self, new_inst: Option<String>) -> Result<(), String> {
    let current = self.current_general_collateral();
    let names_and_roles: Vec<(&str, &str, &str, &Option<String>)> = self
      .general_collaterals
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
      self.current_general_collateral_mut().institution = new_inst;
      Ok(())
    };
    result
  }
  fn choose_delete_collateral(&mut self) -> bool {
    loop {
      self.sort_data_by_dates();
      self.display_delete_collateral();
      println_yel!("Are you sure you want to delete this collateral?");
      println_inst!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
      let mut confirm = String::new();
      let input_attempt = io::stdin().read_line(&mut confirm);
      let command = match input_attempt {
        Ok(_) => confirm.trim().to_string(),
        Err(e) => {
          println_err!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &command[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          self.delete_current_collateral();
          let new_collateral_id = self.reindex_collaterals(None);
          match new_collateral_id {
            _ => (),
          }
          self.write_to_files();
          break true;
        }
        _ => {
          break false;
        }
      }
    }
  }
  fn choose_delete_general_collateral(&mut self) -> bool {
    loop {
      self.display_delete_general_collateral();
      println_yel!("Are you sure you want to delete this general collateral?");
      println_inst!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
      let mut confirm = String::new();
      let input_attempt = io::stdin().read_line(&mut confirm);
      let command = match input_attempt {
        Ok(_) => confirm.trim().to_string(),
        Err(e) => {
          println_err!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &command[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          self.delete_current_general_collateral();
          self.reindex_general_collaterals();
          self.write_to_files();
          break true;
        }
        _ => {
          break false;
        }
      }
    }
  }
  fn get_clients_by_collateral_id(&self, id: u32) -> Vec<&Client> {
    let clients: Vec<&Client> = self.clients
      .iter()
      .filter(|c|
        c.foreign_keys["collateral_ids"]
          .iter()
          .any(|co_id|
            co_id == &id))
      .collect();
    
    clients
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
    println_on_bg!("{:-^162}", "-");
    println_on_bg!("{:-^162}", " DELETE COLLATERAL ");
    println_on_bg!("{:-^162}", "-");
    println_on_bg!(
      "{:-^30} | | {:-^30} | {:-^30} | {:-^40}",
      "Name", "Role/Title", "Institution", "Client(s)"
    );
    println_on_bg!(
      "{: ^30} | {: ^30} | {: ^30} | {: ^40}",
      current.full_name(),
      current.title,
      display_inst,
      all_client_names,
    );
    println_on_bg!("{:-^162}", "-");
  }
  fn display_delete_general_collateral(&self) {
    let current = self.current_general_collateral();

    let inst_option = &current.institution;
    let display_inst = match inst_option {
      Some(i) => i.to_string(),
      None => String::from("n/a"),
    };

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{:-^96}", " DELETE COLLATERAL ");
    println_on_bg!("{:-^96}", "-");
    println_on_bg!(
      "{:-^30} | | {:-^30} | {:-^30}",
      "Name", "Role/Title", "Institution",
    );
    println_on_bg!(
      "{: ^30} | {: ^30} | {: ^30}",
      current.full_name(),
      current.title,
      display_inst,
    );
    println_on_bg!("{:-^96}", "-");
  }
  fn delete_current_collateral(&mut self) {
    let id = self.foreign_key.get("current_collateral_id").unwrap().to_owned();
    self.delete_from_blanks(String::from("collateral"), id);
    let mut new_user_ids = self.current_user().foreign_keys["collateral_ids"].clone();
    new_user_ids.retain(|co_id| co_id != &id );
    self.current_user_mut().foreign_keys.insert(String::from("collateral_ids"), new_user_ids);
    let c_ids: Vec<u32> = self.get_clients_by_collateral_id(id).iter().map(|c| c.id ).collect();
    for c_id in c_ids {
      let mut new_client_ids = self.get_client_by_id(c_id).unwrap().foreign_keys["collateral_ids"].clone();
      new_client_ids.retain(|co_id| co_id != &id );
      self.get_client_by_id_mut(c_id).unwrap().foreign_keys.insert(String::from("collateral_ids"), new_client_ids);
    }
    for n in &mut self.notes {
      let mut new_ids = n.foreign_keys["collateral_ids"].clone();
      new_ids.retain(|co_id| co_id != &id );
      n.foreign_keys.insert(String::from("collateral_ids"), new_ids);
    }
    self.collaterals.retain(|c| c.id != id);
    self.foreign_key.remove("current_collateral_id");
  }
  fn delete_current_general_collateral(&mut self) {
    let id = self.foreign_key.get("current_general_collateral_id").unwrap().to_owned();
    self.delete_from_blanks(String::from("collateral"), id);
    self.general_collaterals.retain(|c| c.id != id);
    self.foreign_key.remove("current_general_collateral_id");
  }
  fn reindex_collaterals(&mut self, current_id: Option<u32>) -> Option<u32> {
    let mut i: u32 = 1;
    let mut changes: HashMap<u32, u32> = HashMap::new();
    let mut new_current_id: Option<u32> = None;
    for co in &mut self.collaterals {
      match current_id {
        Some(id) => {
          if co.id == id {
            new_current_id = Some(i);
          }
        },
        None => (),
      }
      if co.id == i {
        ()
      } else {
        changes.insert(co.id, i);
        co.id = i;
      }
      i += 1;
    }
    for cl in &mut self.clients {
      let old_ids = cl.foreign_keys["collateral_ids"].clone();
      let new_ids: Vec<u32> = old_ids.iter()
        .map(|co_id| match changes.get(co_id) { Some(val) => *val, None => *co_id } )
        .collect();
      cl.foreign_keys.insert(String::from("collateral_ids"), new_ids);
    }
    for u in &mut self.users {
      let old_ids = u.foreign_keys["collateral_ids"].clone();
      let new_ids: Vec<u32> = old_ids.iter()
        .map(|co_id| match changes.get(co_id) { Some(val) => *val, None => *co_id } )
        .collect();
      u.foreign_keys.insert(String::from("collateral_ids"), new_ids);
    }
    new_current_id
  }
  fn reindex_general_collaterals(&mut self) {
    let mut i: u32 = 1;
    for mut co in &mut self.general_collaterals {
      co.id = i;
      i += 1;
    }
  }
  fn get_collateral_by_id(&self, id: u32) -> Option<&Collateral> {
    self.collaterals.iter().find(|p| p.id == id)
  }
  fn get_general_collateral_by_id(&self, id: u32) -> Option<&Collateral> {
    self.general_collaterals.iter().find(|p| p.id == id)
  }

  // pronouns
  pub fn read_pronouns(&mut self) -> Result<Vec<Pronouns>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(self.filepaths["pronouns_filepath"].clone())
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
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
      let line_string = line?;
      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      if values.len() < 5 {
        return Err(Error::new(ErrorKind::Other, "Failed to read pronouns from filepath."));
      }
        
      // if any pronouns have a matching ID
      // due to someone editing the default values,
      // change ID to last item in vector + 1, continuing count
        
      let saved_id: u32 = values[0].parse().unwrap();
      let next_id = pronouns[pronouns.len() - 1].id + 1;
        
      let subject = String::from(&values[1]);
      let object = String::from(&values[2]);
      let possessive_determiner = String::from(&values[3]);
      let possessive = String::from(&values[4]);
      
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
    Ok(pronouns)
  }
  pub fn read_pronouns_from_file_without_reindexing(filepath: &str) -> Result<Vec<Pronouns>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
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
      let line_string = line?;
      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      if values.len() < 5 {
        return Err(Error::new(ErrorKind::Other, "Failed to read pronouns from filepath."));
      }
        
      // if any pronouns have a matching ID
      // due to someone editing the default values,
      // change ID to last item in vector + 1, continuing count
        
      let next_id = pronouns[pronouns.len() - 1].id + 1;
        
      let subject = String::from(&values[1]);
      let object = String::from(&values[2]);
      let possessive_determiner = String::from(&values[3]);
      let possessive = String::from(&values[4]);

      let p = Pronouns::new(next_id, subject, object, possessive_determiner, possessive);
      pronouns.push(p);
    }
    Ok(pronouns)
  }
  fn reassign_pronouns_id(&mut self, old_id: u32, new_id: u32) {
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
        println_inst!(
          "| {} | {} | {} | {}",
          "Enter ID to choose pronouns.",
          "NEW / N: create new pronouns",
          "EDIT / E: edit pronouns",
          "DELETE: delete pronouns",
        );
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();

      match input {
        "new" | "n" => {
          let pronouns_option = self.create_get_pronouns();
          match pronouns_option {
            Some(p) => {
              let new_id = p.id;
              break new_id;
            },
            None => continue,
          }
        },
        "edit" | "e" => {
          self.choose_edit_pronouns();
          continue;
        },
        "delete" | "d" => {
          self.choose_delete_pronouns();
          continue;
        },
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println_err!("Could not read input as a number; try again ({}).", e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          };
          match self.load_pronouns(id) {
            Ok(_) => break id,
            Err(e) => {
              println_err!("Unable to load pronouns with id {}: {}", input, e);
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
        println_inst!("| {} | {}", "NEW / N: new", "EDIT / E: edit (for all data)");
        println_inst!("| {} | {}", "DELETE / D: delete (for all data)", "QUIT / Q: quit menu/cancel");
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();

      match input {
        "new" | "n" => {
          let pronouns_option = self.create_get_pronouns();
          match pronouns_option {
            Some(p) => {
              let new_id = p.id;
              break Some(new_id);
            },
            None => continue,
          }
        },
        "edit" | "e" => {
          self.choose_edit_pronouns();
          continue;
        },
        "delete" | "d" => {
          self.choose_delete_pronouns();
          continue;
        },
        "quit" | "q" => {
          break None;
        },
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println_err!("Could not read input as a number; try again ({}).", e);
              continue;
            }
          };
          match self.load_pronouns(id) {
            Ok(_) => break Some(id),
            Err(e) => {
              println_err!("Unable to load pronouns with id {}: {}", input, e);
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
    println_on_bg!("{:-^44}", "-");
    println_on_bg!("{:-^44}", " Pronouns ");
    println_on_bg!("{:-^44}", "-");
    println_on_bg!("{:-^10} | {:-^31}", " ID ", " Pronouns ");
    for p in &self.pronouns {
      println_on_bg!("{: ^10} | {: ^31}", p.id, p.short_string());
    }
    println_on_bg!("{:-^44}", "-");
  }
  fn display_view_pronoun(&self, prns_id: u32) {
    let prns = self.get_pronouns_by_id(prns_id).unwrap();
    let mut title = String::from(" ");
    title.push_str(&prns.short_string()[..]);
    title.push_str(" ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^69}", "-");
    println_on_bg!("{:-^69}", " Edit pronouns ");
    println_on_bg!("{:-^69}", title);
    println_on_bg!("{:-^69}", "-");
    println_on_bg!("{:-^15} | {:-^15} | {:-^15} | {:-^15}", "Subject", "Object", "Pos. Det.", "Pos.");
    println_on_bg!("{: ^15} | {: ^15} | {: ^15} | {: ^15}", prns.subject, prns.object, prns.possessive_determiner, prns.possessive);
    println_on_bg!("{:-^69}", "-");
  }
  fn display_pronoun_examples(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^109}", "-");
    println_on_bg!("{: ^25} | {: ^25} | {: ^25} | {: ^25}", "Subject pronoun", "Object pronoun", "Possessive determiner", "Possessive pronoun");
    println_on_bg!("{:-^109}", "-");
    println_on_bg!("{: ^25} | {: ^25} | {: ^25} | {: ^25}", "he", "him", "his", "his");
    println_on_bg!("{: ^25} | {: ^25} | {: ^25} | {: ^25}", "she", "her", "her", "hers");
    println_on_bg!("{: ^25} | {: ^25} | {: ^25} | {: ^25}", "they", "them", "their", "theirs");
    println_on_bg!("{:-^109}", "-");
  }
  fn create_get_pronouns(&mut self) -> Option<Pronouns> {
    let pronouns_option = 'pronouns: loop {
      self.display_pronoun_examples();
      let subject = loop {
        let mut subject_choice = String::new();
        println_inst!("Enter your subject pronoun (e.g., he, she, they). Example: [pronoun] attended a Care Plan Meeting.");
        let subject_attempt = io::stdin().read_line(&mut subject_choice);
        match subject_attempt {
          Ok(_) => match subject_choice.to_ascii_lowercase().trim() {
            "quit" | "q" => break 'pronouns None,
            _ => break String::from(subject_choice.trim()),
          }
          Err(e) => {
            println_err!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let object = loop {
        let mut object_choice = String::new();
        println_inst!(
          "Enter your object pronoun (e.g., him, her, them). Example: Guidance counselor called ICC and left a message for [pronoun]."
        );
        let object_attempt = io::stdin().read_line(&mut object_choice);
        match object_attempt {
          Ok(_) => match object_choice.to_ascii_lowercase().trim() {
            "quit" | "q" => break 'pronouns None,
            _ => break String::from(object_choice.trim()),
          }
          Err(e) => {
            println_err!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let possessive_determiner = loop {
        let mut possessive_determiner_choice = String::new();
        println_inst!(
          "Enter your possessive determiner (e.g., his, her, their). Example: ICC used [pronoun] personal vehicle to transport youth home."
        );
        let possessive_determiner_attempt =
          io::stdin().read_line(&mut possessive_determiner_choice);
        match possessive_determiner_attempt {
          Ok(_) => match possessive_determiner_choice.trim() {
            "quit" | "q" => break 'pronouns None,
            _ => break String::from(possessive_determiner_choice.trim()),
          }
          Err(e) => {
            println_err!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let possessive = loop {
        let mut possessive_choice = String::new();
        println_inst!(
          "Enter your possessive pronoun (e.g., his, hers, theirs). Example: OPT for youth provided her contact information, and ICC provider [pronoun]."
        );
        let possessive_attempt = io::stdin().read_line(&mut possessive_choice);
        match possessive_attempt {
          Ok(_) => match possessive_choice.to_ascii_lowercase().trim() {
            "quit" | "q" => break 'pronouns None,
            _ => break String::from(possessive_choice.trim()),
          },
          Err(e) => {
            println_err!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let pronouns_attempt =
        self.generate_unique_new_pronouns(subject, object, possessive_determiner, possessive);
      match pronouns_attempt {
        Ok(pronouns) => break Some(pronouns),
        Err(e) => {
          println_err!("Pronouns could not be generated: {}.", e);
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
        println_suc!("Pronouns records updated.");
        thread::sleep(time::Duration::from_secs(1));
        Some(new_pronouns)
      }
      None => None,
    }
  }
  fn generate_unique_new_pronouns(
    &mut self,
    subject: String,
    object: String,
    possessive_determiner: String,
    possessive: String,
  ) -> Result<Pronouns, String> {

    if subject.contains(" | ") || object.contains(" | ") || possessive_determiner.contains(" | ") || possessive.contains(" | ") {
      return Err(String::from("Name cannot contain ' | '."));
    }

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
    let mut file = File::create(self.filepaths["pronouns_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn save_pronouns(&mut self, pronouns: Pronouns) {
    self.pronouns.push(pronouns);
    self.write_to_files();
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
      println_yel!("Warning: Duplicate pronouns will be deleted automatically.");
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
            println_inst!("Enter ID to edit.");
            let read_attempt = io::stdin().read_line(&mut choice);
            match read_attempt {
              Ok(_) => break choice,
              Err(e) => {
                println_err!("Could not read input; try again ({}).", e);
                continue;
              }
            }
          };
          match &input.to_ascii_lowercase().trim().to_string()[..] {
            "quit" | "q" => break 'choose_edit_pronouns,
            _ => {
              let id = match input.trim().parse::<u32>() {
                Ok(num) => num,
                Err(e) => {
                  println_err!("Could not read input as a number; try again ({}).", e);
                  continue;
                }
              };
              match self.get_pronouns_by_id(id) {
                Some(p) => break p,
                None => {
                  println_err!("Unable to load pronouns with ID {}.", id);
                  continue;
                }
              }
            }
          }
        };
        final_pronouns_id = pronouns.id;
        println_inst!("Choose the pronoun to edit (SUBJ, OBJ, POSDET, POS).");
        println_inst!("'Q'/'QUIT' to quit menu.");
        let mut pronoun_to_edit = String::new();
        let input_attempt = io::stdin().read_line(&mut pronoun_to_edit);
        match input_attempt {
          Ok(_) => (),
          Err(_) => {
            println_err!("Failed to read input. Please try again.");
            continue;
          }
        }
        pronoun_to_edit = pronoun_to_edit.to_ascii_lowercase().trim().to_string();
        match &pronoun_to_edit[..] {
          "quit" | "q" => {
            continue;
          }
          _ => (),
        }
        final_pronoun_to_edit = pronoun_to_edit.clone();
        let new_pronoun = match &pronoun_to_edit[..] {
          "subj" | "SUBJ" | "subject" | "SUBJECT" | "Subject" => {
            println_inst!("Enter your subject pronoun (e.g., he, she, they). Example: [pronoun] attended a Care Plan Meeting.");
            let mut subject_choice = String::new();
            let subject_attempt = io::stdin().read_line(&mut subject_choice);
            let p = match subject_attempt {
              Ok(_) => String::from(subject_choice.trim()),
              Err(e) => {
                println_err!("Failed to read line: {}", e);
                continue;
              }
            };
            p
          },
          "obj" | "OBJ" | "object" | "OBJECT" | "Object" => {
            println_inst!(
              "Enter your object pronoun (e.g., him, her, them). Example: Guidance counselor called ICC and left a message for [pronoun]."
            );
            let mut object_choice = String::new();
            let object_attempt = io::stdin().read_line(&mut object_choice);
            let p = match object_attempt {
              Ok(_) => String::from(object_choice.trim()),
              Err(e) => {
                println_err!("Failed to read line: {}", e);
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
            println_inst!(
              "Enter your possessive determiner (e.g., his, her, their). Example: ICC used [pronoun] personal vehicle to transport youth home."
            );
            let mut posdet_choice = String::new();
            let posdet_attempt = io::stdin().read_line(&mut posdet_choice);
            let p = match posdet_attempt {
              Ok(_) => String::from(posdet_choice.trim()),
              Err(e) => {
                println_err!("Failed to read line: {}", e);
                continue;
              }
            };
            p
          },
          "possessive" | "POSSESSIVE" | "Possessive" | "pos" | "POS" | "possess" | "Possess"
          | "POSSESS" => {
            println_inst!(
              "Enter your possessive pronoun (e.g., his, hers, theirs). Example: OPT for youth provided her contact information, and ICC provider [pronoun]."
            );
            let mut possessive_choice = String::new();
            let possessive_attempt = io::stdin().read_line(&mut possessive_choice);
            let p = match possessive_attempt {
              Ok(_) => String::from(possessive_choice.trim()),
              Err(e) => {
                println_err!("Failed to read line: {}", e);
                continue;
              }
            };
            p
          },
          _ => {
            println_err!("Invalid entry.");
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
        println_inst!("| {} | {}", "Enter ID to delete.", "QUIT / Q: cancel");
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();
    
      match input {
        "quit" | "q" => {
          break;
        },
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println_err!("Could not read input as a number; try again ({}).", e);
              continue;
            }
          };
          match self.load_pronouns(id) {
            Ok(_) => {
              self.display_view_pronoun(id);
              println_yel!("Are you sure you want to delete this set of pronouns?");
              println_inst!("'YES'/'Y' to confirm.");
              let mut confirm = String::new();
              let input_attempt = io::stdin().read_line(&mut confirm);
              let command = match input_attempt {
                Ok(_) => confirm.trim().to_string(),
                Err(e) => {
                  println_err!("Failed to read input: {}", e);
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
              println_err!("Unable to load pronouns with id {}: {}", input, e);
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
      println_inst!("| {} | {} | {}", "EDIT / E: edit (for all data)", "DELETE / D: delete (for all data)", "QUIT / Q: quit menu");
      let mut decision = String::new();
      let input_attempt = io::stdin().read_line(&mut decision);
      match input_attempt {
        Ok(_) => (),
        Err(_) => {
          println_err!("Failed to read input. Please try again.");
          continue;
        }
      }
      decision = decision.to_ascii_lowercase().trim().to_string();
      match &decision[..] {
        "quit" | "q" => break,
        "delete" => {
          self.delete_pronouns(prns_id);
          self.display_pronouns();
          thread::sleep(time::Duration::from_secs(1));
          continue;
        },
        "edit" | "e" => {
          println_inst!("Choose the pronoun to edit (SUBJ, OBJ, POSDET, POS).");
          println_inst!("'Q'/'QUIT' to quit menu.");
          let mut pronoun_to_edit = String::new();
          let field_input = io::stdin().read_line(&mut pronoun_to_edit);
          match field_input {
            Ok(_) => (),
            Err(_) => {
              println_err!("Failed to read input. Please try again.");
              continue;
            }
          }
          pronoun_to_edit = pronoun_to_edit.to_ascii_lowercase().trim().to_string();
          match &pronoun_to_edit[..] {
            "quit" | "q" => {
              continue;
            },
            "subj" | "subject" => {
              println_inst!("Enter your subject pronoun (e.g., he, she, they). Example: [pronoun] attended a Care Plan Meeting.");
            },
            "obj" | "object" => {
              println_inst!(
                "Enter your object pronoun (e.g., him, her, them). Example: Guidance counselor called ICC and left a message for [pronoun]."
              );
            },
            "posdet" | "possessive determiner" => {
              println_inst!(
                "Enter your possessive determiner (e.g., his, her, their). Example: ICC used [pronoun] personal vehicle to transport youth home."
              );
            },
            "possessive" | "pos" | "possess" => {
              println_inst!(
                "Enter your possessive pronoun (e.g., his, hers, theirs). Example: OPT for youth provided her contact information, and ICC provider [pronoun]."
              );
            },
            _ => {
              println_err!("Invalid command.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
          let mut choice = String::new();
          let attempt = io::stdin().read_line(&mut choice);
          let new_prn = match attempt {
            Ok(_) => String::from(choice.trim()),
            Err(e) => {
              println_err!("Failed to read line: {}", e);
              continue;
            }
          };
          self.update_pronouns_record(prns_id, pronoun_to_edit, new_prn);
          self.display_pronouns();
          thread::sleep(time::Duration::from_secs(1));
        },
        _ => {
          println_err!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
          continue;
        },
      }
    }
  }
  fn delete_pronouns(&mut self, prns_id: u32) {
    self.pronouns.retain(|p| p.id != prns_id);
    match self.foreign_key.get("current_user_id") {
      Some(_) => {
        if self.current_user().pronouns == prns_id {
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          println_yel!("Please select new pronouns before continuing.");
          thread::sleep(time::Duration::from_secs(1));
          self.current_user_mut().pronouns = self.choose_pronouns();
          self.write_to_files();
        }
      },
      None => (),
    }
  }

  // goals
  pub fn read_goals(filepath: &str) -> Result<Vec<Goal>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
    }

    let mut goals: Vec<Goal> = vec![];

    for line in lines {
      let line_string = line?;
      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let id: u32 = values[0].parse().unwrap();
      let client_id: u32 = values[1].parse().unwrap();
      let subject = String::from(&values[2]);
        
      
      let g = Goal::new(id, client_id, subject);
      goals.push(g);
    }
      
    Ok(goals)
  }
  fn select_all_goals(&mut self, selected: Option<Vec<u32>>) -> Option<u32> {
    let chosen_id = loop {
      self.display_goals(selected.clone());
      let input = loop {
        let mut choice = String::new();
        println_inst!("Enter ID to choose goal, or 'CANCEL' to cancel.");
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();

      match input {
        "cancel" | "c" => return None,
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println_err!("Could not read input as a number; try again ({}).", e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          };
          match self.load_goal(id) {
            Ok(_) => {
              if self.current_client_goals().iter().any(|g| g.id == id)
              || self.current_client_goals().iter().any(|g| g.goal == self.get_goal_by_id(id).unwrap().goal) {
                println_err!("Please choose from among the listed IDs.");
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
              break id;
            }
            Err(e) => {
              println_err!("Unable to load goal with id {}: {}", input, e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        },
      }
    };
    Some(chosen_id)
  }
  fn select_client_goals(&mut self, selected: Option<Vec<u32>>) -> Option<u32> {
    let chosen_id = loop {
      self.display_current_client_goals(selected.clone());
      let input = loop {
        let mut choice = String::new();
        println_inst!("Enter ID to choose goal.");
        println_inst!("Press ENTER to continue with the selected options.");
        println_inst!("| {} | {} | {}", "NEW / N: Create a new goal", "ADD / A: Add a goal from the collective list", "CANCEL / C: cancel");
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();

      match input {
        "new" | "n" => {
          let goal_option = self.create_get_goal();
          match goal_option {
            Some(g) => {
              let new_id = g.id;
              break new_id;
            },
            None => continue,
          }
        },
        "add" | "a" => {
          let added_g_id = self.select_all_goals(None);
          match added_g_id {
            None => (),
            Some(g_id) => {
              let g_copy = self.get_goal_by_id(g_id).unwrap().clone();
              let new_goal = self.generate_unique_new_goal(g_copy.goal.clone());
              match new_goal {
                Err(_) => {
                  println_err!("Failed to generate goal.");
                  continue;
                },
                Ok(goal) => {
                  let new_id = goal.id;
                  self.save_goal(goal);
                  break new_id;
                }
              }
            }
          }
        }
        "cancel" | "c" | "" => return None,
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println_err!("Could not read input as a number; try again ({}).", e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          };
          if !self.current_client_goals().iter().any(|g| g.client_id == self.current_client().id ) {
            println_err!("Please select from among the listed IDs.");
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
          match self.load_goal(id) {
            Ok(_) => break id,
            Err(e) => {
              println_err!("Unable to load goal with id {}: {}", input, e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        },
      }
    };
    Some(chosen_id)
  }
  fn choose_client_goals(&mut self) {
    loop {
      self.display_current_client_goals(None);
      let input = loop {
        let mut choice = String::new();
        println_inst!("Enter ID to edit or delete goal.");
        println_inst!("| {} | {} | {}", "NEW / N: Create a new goal", "ADD / A: Add a goal from the collective list", "QUIT / Q: exit menu");
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
      };
      let input = input.trim();

      match input {
        "new" | "n" => {
          let goal_option = self.create_get_goal();
          match goal_option {
            Some(_) => continue,
            None => continue,
          }
        },
        "add" | "a" => {
          let added_g_id = self.select_all_goals(None);
          match added_g_id {
            None => (),
            Some(g_id) => {
              let g_copy = self.get_goal_by_id(g_id).unwrap().clone();
              let new_goal = self.generate_unique_new_goal(g_copy.goal.clone());
              match new_goal {
                Err(_) => {
                  println_err!("Failed to generate goal.");
                  continue;
                },
                Ok(goal) => {
                  self.save_goal(goal);
                  continue;
                }
              }
            }
          }
        }
        "quit" | "q" => break,
        _ => {
          let id = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
              println_err!("Could not read input as a number; try again ({}).", e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          };
          match self.load_goal(id) {
            Ok(_) => self.choose_edit_goal(),
            Err(e) => {
              println_err!("Unable to load goal with id {}: {}", input, e);
              thread::sleep(time::Duration::from_secs(1));
              continue;
            }
          }
        },
      }
    }
  }
  fn display_goals(&self, selected: Option<Vec<u32>>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^161}", "-");
    println_on_bg!("{:-^161}", " All goals ");
    println_on_bg!("{:-^161}", "-");
    println_on_bg!("{:->5} | {:-<30} | {:-<120}", "ID", "Client ", "Goal ");
    for g in &self.goals.clone() {
      if !self.current_client_goals().iter().any(|cg| cg.id == g.id) 
      && !self.current_client_goals().iter().any(|cg| cg.goal == g.goal ) {
        let client = self.get_client_by_id(g.client_id).unwrap().full_name();
        match selected.clone() {
          None => println_on_bg!("{: >5} | {: <30} | {: <120}", g.id, client, g.goal),
          Some(sel) => {
            if sel.iter().any(|id| id == &g.id ) {
              println_suc!("{: >5} | {: <30} | {: <120}", g.id, client, g.goal);
            } else {
              println_on_bg!("{: >5} | {: <30} | {: <120}", g.id, client, g.goal);
            }
          }
        }
      }
    }
    println_on_bg!("{:-^161}", "-");
  }
  fn display_goal(&self) {
    let g = self.current_goal().unwrap();
    let client = self.get_client_by_id(g.client_id).unwrap().full_name();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^161}", "-");
    let heading = format!(" Goal for {} ", client);
    println_on_bg!("{:-^161}", heading);
    println_on_bg!("{:-^161}", "-");
    println_on_bg!("{:->5} | {:-<120}", " ID", " Goal ");
    println_on_bg!("{: >5} | {: <120}", g.id, g.goal);
    println_on_bg!("{:-^161}", "-");
  }
  fn current_client_goals(&self) -> Vec<&Goal> {
    self.goals.iter().filter(|g| g.client_id == self.foreign_key["current_client_id"] ).collect()
  }
  fn display_current_client_goals(&self, selected: Option<Vec<u32>>) {
    let client = self.current_client();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^161}", "-");
    let heading = format!(" All goals for {} ", client.full_name());
    println_on_bg!("{:-^161}", heading);
    println_on_bg!("{:-^161}", "-");
    println_on_bg!("{:->5} | {:-<120}", " ID", "Goal ");
    for g in &self.current_client_goals() {
      match selected.clone() {
        None => println_on_bg!("{: >5} | {: <120}", g.id, g.goal),
        Some(sel) => {
          if sel.iter().any(|id| id == &g.id ) {
            println_suc!("{: >5} | {: <120}", g.id, g.goal);
          } else {
            println_on_bg!("{: >5} | {: <120}", g.id, g.goal);
          }
        }
      }
    }
    println_on_bg!("{:-^161}", "-");

  }
  fn create_get_goal(&mut self) -> Option<Goal> {
    let goal_option = 'goal: loop {
      let goal_string = loop {
        let mut subject_choice = String::new();
        println_inst!("Enter the client's goal.");
        let subject_attempt = io::stdin().read_line(&mut subject_choice);
        match subject_attempt {
          Ok(_) => match subject_choice.to_ascii_lowercase().trim() {
            "quit" | "q" => break 'goal None,
            _ => break String::from(subject_choice.trim()),
          }
          Err(e) => {
            println_err!("Failed to read line: {}", e);
            continue;
          }
        };
      };
      let goal_attempt = self.generate_unique_new_goal(goal_string);
      match goal_attempt {
        Ok(goal) => break Some(goal),
        Err(e) => {
          println_err!("Goal could not be generated: {}.", e);
          thread::sleep(time::Duration::from_secs(1));
          break None;
        }
      }
    };
    match goal_option {
      Some(g) => {
        let new_goal = g.clone();
        self.save_goal(g);
        self.display_current_client_goals(None);
        println_suc!("Goals updated.");
        thread::sleep(time::Duration::from_secs(1));
        Some(new_goal)
      }
      None => None,
    }
  }
  fn generate_unique_new_goal(
    &mut self,
    goal_string: String,
  ) -> Result<Goal, String> {

    if goal_string.contains(" | ") {
      return Err(String::from("Goal cannot contain ' | '."));
    }

    let id: u32 = self.goals.len() as u32 + 1;

    let new_goal = Goal::new(
      id,
      self.foreign_key["current_client_id"],
      goal_string,
    );

    let result = if self.goals.iter().any(|g| g == &new_goal) {
      Err(format!(
        "Goal already stored ({}).",
        new_goal.goal,
      ))
    } else {
      Ok(new_goal)
    };

    result
  }
  pub fn write_goals(&mut self) -> std::io::Result<()> {
    let mut lines = String::from("##### goals #####\n");
    for p in &self.goals {
      lines.push_str(&p.to_string()[..]);
    }
    lines.push_str("##### goals #####");
    let mut file = File::create(self.filepaths["goal_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  pub fn save_goal(&mut self, goal: Goal) {
    self.goals.push(goal);
    self.reindex_goals();
    self.write_to_files();
  }
  fn load_goal(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&Goal> = self.goals.iter().find(|c| c.id == id);
    match current {
      Some(c) => {
        self.foreign_key.insert(String::from("current_goal_id"), c.id);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to read goal from filepath.",
      )),
    }
  }
  pub fn get_goal_by_id(&self, id: u32) -> Option<&Goal> {
    self.goals.iter().find(|p| p.id == id)
  }
  pub fn get_goal_by_id_mut(&mut self, id: u32) -> Option<&mut Goal> {
    self.goals.iter_mut().find(|p| p.id == id)
  }
  pub fn current_goal(&self) -> Option<&Goal> {
    self.get_goal_by_id(self.foreign_key["current_goal_id"])
  }
  pub fn current_goal_mut(&mut self) -> Option<&mut Goal> {
    self.get_goal_by_id_mut(self.foreign_key["current_goal_id"])
  }
  fn choose_edit_goal(&mut self) {
    loop {
      self.display_goal();
      let input = loop {
        let mut choice = String::new();
        println_inst!("Enter new text for this goal, or 'CANCEL' to go back.");
        println_inst!("DELETE / D to delete.");
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.trim().to_string(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      match &input[..] {
        "cancel" | "c" => {
          break;
        }
        "delete" | "d" => {
          self.choose_delete_goal();
          break;
        }
        _ => (),
      }
      let confirm = loop {
        let mut choice = String::new();
        println_inst!("Change goal to '{}' ( Y / N )?", &input);
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.trim().to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      match &confirm[..] {
        "no" | "n" => continue,
        "yes" | "y" => {
          self.current_goal_mut().unwrap().goal = input.to_string();
          self.write_to_files();
          break;
        }
        _ => {
          println_err!("Please choose either 'yes' or 'no'.");
          continue;
        }
      }
    }
  }
  fn choose_delete_goal(&mut self) {
    loop {
      self.display_goal();
      println_yel!("Are you sure you want to delete this goal?");
      println_inst!("'YES'/'Y' to confirm.");
      let mut confirm = String::new();
      let input_attempt = io::stdin().read_line(&mut confirm);
      let command = match input_attempt {
        Ok(_) => confirm.trim().to_string(),
        Err(e) => {
          println_err!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &command[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          self.delete_current_goal();
          self.reindex_goals();
          break;
        },
        _ => {
          break;
        },
      }
    }
  }
  // fn choose_delete_goals(&mut self) {
  //   loop {
  //     self.display_current_client_goals(None);
  //     let input = loop {
  //       let mut choice = String::new();
  //       println_inst!("| {} | {}", "Enter ID to delete.", "CANCEL / C: cancel");
  //       let read_attempt = io::stdin().read_line(&mut choice);
  //       match read_attempt {
  //         Ok(_) => break choice.to_ascii_lowercase(),
  //         Err(e) => {
  //           println_err!("Could not read input; try again ({}).", e);
  //           thread::sleep(time::Duration::from_millis(10000));
  //           continue;
  //         }
  //       }
  //     };
  //     let input = input.trim();
    
  //     match input {
  //       "cancel" | "c" => {
  //         break;
  //       },
  //       _ => {
  //         let id = match input.trim().parse::<u32>() {
  //           Ok(num) => num,
  //           Err(e) => {
  //             println_err!("Could not read input as a number; try again ({}).", e);
  //             continue;
  //           }
  //         };
  //         match self.load_goal(id) {
  //           Ok(_) => {
  //             self.display_goal();
  //             println_yel!("Are you sure you want to delete this goal?");
  //             println_inst!("'YES'/'Y' to confirm.");
  //             let mut confirm = String::new();
  //             let input_attempt = io::stdin().read_line(&mut confirm);
  //             let command = match input_attempt {
  //               Ok(_) => confirm.trim().to_string(),
  //               Err(e) => {
  //                 println_err!("Failed to read input: {}", e);
  //                 thread::sleep(time::Duration::from_secs(1));
  //                 continue;
  //               }
  //             };
  //             match &command[..] {
  //               "YES" | "yes" | "Yes" | "Y" | "y" => {
  //                 self.delete_current_goal();
  //                 self.reindex_goals();
  //                 continue;
  //               },
  //               _ => {
  //                 break;
  //               },
  //             }
  //           },
  //           Err(e) => {
  //             println_err!("Unable to load goal with id {}: {}", input, e);
  //             continue;
  //           },
  //         }
  //       },
  //     }
  //   }
  // }
  fn delete_goal(&mut self, g_id: u32) {
    self.goals.retain(|g| g.id != g_id);
    self.write_to_files();
  }
  fn delete_current_goal(&mut self) {
    let id = self.foreign_key["current_goal_id"];
    self.delete_from_blanks(String::from("goal"), id);
    self.delete_goal(id);
  }
  fn reindex_goals(&mut self) {
    let mut i: u32 = 1;
    for mut g in &mut self.goals {
      g.id = i;
      i += 1;
    }
  }

  // note_days
  fn current_note_day_mut(&mut self) -> &mut NoteDay {
    let nd_id = match self.foreign_key.get("current_note_day_id") {
      Some(id) => id,
      None => panic!("There is no current date selected."),
    };
    let maybe_current: Option<&mut NoteDay> = self.note_days.iter_mut().find(|nd| nd.id == *nd_id);
    match maybe_current {
      Some(nd) => nd,
      None => panic!("The loaded date ID does not match any saved dates."),
    }
  }
  fn current_note_day(&self) -> &NoteDay {
    let nd_id = match self.foreign_key.get("current_note_day_id") {
      Some(id) => id,
      None => panic!("There is no current date selected."),
    };
    let maybe_current: Option<&NoteDay> = self.note_days.iter().find(|nd| nd.id == *nd_id);
    match maybe_current {
      Some(nd) => nd,
      None => panic!("The loaded date ID does not match any saved dates."),
    }
  }
  fn current_user_note_days(&self) -> Vec<&NoteDay> {
    self.note_days.iter().filter(|nd| nd.foreign_key["user_id"] == self.current_user().id ).collect()
  }
  fn current_client_note_days(&self) -> Vec<&NoteDay> {
    self.note_days.iter().filter(|nd| nd.foreign_key["client_id"] == self.current_client().id ).collect()
  }
  fn current_client_notes_mut(&mut self) -> Vec<&mut Note> {
    let nds: Vec<NoteDay> = self.current_client_note_days().iter().cloned().cloned().collect();
    self.notes.iter_mut().filter(|n| nds.iter().any(|nd| nd.foreign_keys["note_ids"].iter().any(|n_id| n_id == &n.id ) ) ).collect()
  }
  fn current_collateral_notes_mut(&mut self) -> Vec<&mut Note> {
    let co_id_b = &self.current_collateral().id.clone();
    self.notes.iter_mut().filter(|n| n.foreign_keys["collateral_ids"].iter().any(|co_id| co_id == co_id_b ) ).collect()
  }
  /// returns the first 10 notedays for the current user
  fn current_user_recent_15_note_days(&self) -> Vec<&NoteDay> {
    let user_note_days: Vec<&NoteDay> = self.note_days.iter().filter(|nd| nd.foreign_key["user_id"] == self.current_user().id )
      .collect();

    if user_note_days.len() == 0 {
      vec![]
    } else {
      user_note_days.iter().map(|nd| *nd ).take(15).collect()
    } 
    
  }
  fn display_user_all_note_days(&self) {
    let mut heading = String::from(" All notes for ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^77}", "-");
    println_on_bg!("{:-^77}", heading);
    println_on_bg!("{:-^77}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^21}", " ID ", " Client ", " Day ");
    for nd in self.current_user_note_days() {
      println_on_bg!(
        "{: ^10} | {: ^40} | {: <7} {: >12}",
        nd.id,
        self.get_client_by_id(nd.foreign_key["client_id"]).unwrap().full_name(),
        nd.fmt_date_short(),
        nd.fmt_day(),
      );
    }
    println_on_bg!("{:-^77}", "-");
  }
  fn display_user_recent_note_days(&self) {
    let mut heading = String::from(" Recent notes for ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^77}", "-");
    println_on_bg!("{:-^77}", heading);
    println_on_bg!("{:-^77}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^21}", " ID ", " Client ", " Day ");
    for nd in self.current_user_recent_15_note_days() {
      println_on_bg!(
        "{: ^10} | {: ^40} | {: <7} {: >12}",
        nd.id,
        self.get_client_by_id(nd.foreign_key["client_id"]).unwrap().full_name(),
        nd.fmt_date_short(),
        nd.fmt_day(),
      );
    }
    println_on_bg!("{:-^77}", "-");
  }
  fn load_note_day(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&NoteDay> = self.note_days.iter().find(|nd| nd.id == id);
    match current {
      Some(nd) => {
        self.foreign_key.insert(String::from("current_note_day_id"), nd.id);
        let c = self.get_client_by_note_day_id(nd.id).unwrap().clone();
        self.foreign_key.insert(String::from("current_client_id"), c.id);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to read record of notes for the selected date from filepath.",
      )),
    }
  }
  fn choose_edit_note_days(&mut self, mut display_all: bool) {
    loop {
      self.foreign_key.remove("current_client_id");
      let input = loop {
        if display_all {
          self.display_user_all_note_days();
        } else {
          self.display_user_recent_note_days();
        }
        println_inst!("| {} | {} | {}", "Choose note by ID.", "ALL / A: View all notes", "QUIT / Q: quit menu");
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "all" | "a" => {
          display_all = true;
          continue;
        },
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_note_days()
              .iter()
              .any(|&nd| nd.id == num) {
                println_err!("Please choose from among the listed dates, or 'NEW / N' to begin a new record.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note_day(num) {
              Ok(_) => self.edit_note_day(),
              Err(e) => {
                println_err!("Unable to load records with ID {}: {}", num, e);
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn edit_note_day(&mut self) {
    let today = Local::now().naive_local().date();
    let new_date = 'main: loop {
      let current_nd = self.current_note_day();
      self.display_note_day();
      print_inst!("Enter 'CANCEL' at any time to cancel.");
      print!("\n");
      let mut today_choice = String::new();
      if today != current_nd.date {
        'today: loop {

          println_inst!("Change date to today? (Y/N)");
          let today_attempt = io::stdin().read_line(&mut today_choice);
          match today_attempt {
            Ok(_) => (),
            Err(e) => {
              println_err!("Invalid repsonse: {}", e);
              continue;
            }
          }
          match &today_choice.trim()[..] {
            "Y" | "y" | "Yes" | "YES" | "yes" => {
              break 'main today;
            },
            "NO" | "no" | "No" | "N" | "n" => {
              break 'today;
            },
            "Cancel" | "CANCEL" | "cancel" => return (),
            _ => {
              println_err!("Please choose 'yes', 'no', or 'cancel'.");
              continue;
            }
          }
        }
      }
      let mut same_year = false;
      let year = loop {
        let mut this_year_choice = String::new();
        println_inst!("This year ({})? (Y/N)", today.year());
        let this_year_attempt = io::stdin().read_line(&mut this_year_choice);
        match this_year_attempt {
          Ok(_) => match &this_year_choice.trim()[..] {
            "YES" | "yes" | "Yes" | "Y" | "y" => {
              same_year = true;
              break today.year();
            }
            "NO" | "no" | "No" | "N" | "n" => {
              let mut year_choice = String::new();
              println_inst!("What year?");
              let year_attempt = io::stdin().read_line(&mut year_choice);
              match year_attempt {
                Ok(_) => {
                  if year_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                    return ();
                  }
                  match year_choice.trim().parse() {
                    Ok(val) => {
                      if val > 9999 || val < 1000 {
                        println_err!("Please enter a valid year.");
                        continue;
                      } else {
                        break val;
                      }
                    }
                    Err(e) => {
                      println_err!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                }
                Err(e) => {
                  println_err!("Invalid repsonse: {}", e);
                  continue;
                }
              }
            },
            "Cancel" | "CANCEL" | "cancel" => return (),
            _ => {
              println_err!("Please choose 'yes' or 'no.'");
              continue;
            }

          },
          Err(e) => {
            println_err!("Invalid repsonse: {}", e);
            continue;
          }
        }
      };
      let month = 'month: loop {
        if same_year {
          loop {
            let mut this_month_choice = String::new();
            println_inst!("This month ({})? (Y/N)", today.month());
            let this_month_attempt = io::stdin().read_line(&mut this_month_choice);
            match this_month_attempt {
              Ok(_) => match &this_month_choice.trim()[..] {
                "YES" | "yes" | "Yes" | "Y" | "y" => break 'month today.month(),
                "NO" | "no" | "No" | "N" | "n" => {
                  same_year = false;
                  break;
                },
                "CANCEL" | "cancel" | "Cancel" => return (),
                _ => {
                  println_err!("Please choose 'yes' or 'no.'");
                  continue;
                }
              },
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
          }
        }
        let mut month_choice = String::new();
        println_inst!("What month?");
        let month_attempt = io::stdin().read_line(&mut month_choice);
        match month_attempt {
          Ok(_) => {
            if month_choice.trim().to_ascii_lowercase() == String::from("cancel") {
              return ();
            }
            match month_choice.trim().parse() {
              Ok(val) => {
                if val > 12 || val < 1 {
                  println_err!("Please enter a valid month.");
                  continue;
                } else {
                  break val;
                }
              }
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
          }
          Err(e) => {
            println_err!("Invalid repsonse: {}", e);
            continue;
          }
        }
      };
      let day = loop {
        let mut day_choice = String::new();
        println_inst!("What day?");
        let day_attempt = io::stdin().read_line(&mut day_choice);
        match day_attempt {
          Ok(_) => {
            if day_choice.trim().to_ascii_lowercase() == String::from("cancel") {
              return ();
            }
            match day_choice.trim().parse() {
              Ok(val) => {
                if val > 31 || val < 1 {
                  println_err!("Please enter a valid day.");
                  continue;
                } else {
                  break val;
                }
              }
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
          }
          Err(e) => {
            println_err!("Invalid repsonse: {}", e);
            continue;
          }
        }
      };
      match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => break date,
        None => {
          println_err!(
            "{}-{}-{} does not appear to be a valid date. Please try again.",
            year, month, day
          );
          continue;
        }
      }
        
    };
    self.current_note_day_mut().date = new_date;
    self.write_to_files();
  }
  fn print_most_recent_note_days(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let nds = self.current_user_note_days();
    let max_nd = nds.iter().max_by(|a, b| a.date.cmp(&b.date) );
    match max_nd {
      None => println_on_bg!("The current user has no notes to display."),
      Some(nd) => {
        let max = nd.date.clone();
        let recents = nds.iter().filter(|note_day| note_day.date == max ).cloned().cloned();
        for note_day in recents {
          self.print_note_day(note_day);
        }
      }
    }
    println_inst!("{:-^150}", " Enter any input to return to the previous menu. ");
    let mut s = String::new();
    let input_attempt = io::stdin().read_line(&mut s);
    match input_attempt {
      _ => (),
    }
  }
  fn choose_note_days(&mut self) {
    let mut display_all = false;
    loop {
      self.sort_data_by_dates();
      self.foreign_key.remove("current_client_id");
      let input = loop {
        if display_all {
          self.display_user_all_note_days();
        } else {
          self.display_user_recent_note_days();
        }
        println_inst!(
          "| {} | {} | {} | {}",
          "Choose note by ID.",
          "NEW / N: New note day",
          "EDIT / E: Edit note days",
          "TEMPLATES / T: View/edit templates",
        );
        if display_all {
          println_inst!(
            "| {} | {} | {}",
            "RECENT / R: View only recent notes",
            "PRINT / P: Display all notes from most recent day",
            "QUIT / Q: quit menu"
          );
        } else {
          println_inst!(
            "| {} | {} | {}",
            "ALL / A: View all notes",
            "PRINT / P: Display all notes from most recent day",
            "QUIT / Q: quit menu"
          );
        }
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_note_day_get_id();
          match maybe_new_id {
            Some(id) => {
              match self.load_note_day(id) {
                Ok(_) => self.choose_note_day(),
                Err(e) => {
                  panic!("Unable to load newly created record with ID '{}': {}", id, e);
                }
              }
            },
            None => (),
          }
          continue;
        },
        "edit" | "e" => {
          self.choose_edit_note_days(display_all);
        },
        "templates" | "t" => {
          self.choose_note_templates();
        },
        "all" | "a" => {
          display_all = true;
        },
        "recent" | "r" => {
          display_all = false;
        },
        "print" | "p" => {
          self.print_most_recent_note_days();
        },
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_note_days()
              .iter()
              .any(|&nd| nd.id == num) {
                println_err!("Please choose from among the listed dates, or 'NEW / N' to begin a new record.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note_day(num) {
              Ok(_) => self.choose_note_day(),
              Err(e) => {
                println_err!("Unable to load records with ID {}: {}", num, e);
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn get_current_note_day_notes_by_category(&self) -> BTreeMap<NoteCategory, Vec<Note>> {
    let mut output: BTreeMap<NoteCategory, Vec<Note>> = BTreeMap::new();
    let mut ns = self.current_note_day_notes();
    ns.sort_by(|a, b| a.id.cmp(&b.id) );
    ns.sort_by(|a, b| a.category.cmp(&b.category) );
    for n in ns {
      output.entry(n.category).or_insert(vec![]);
      output.get_mut(&n.category).unwrap().push(n.clone());
    }
    output
  }
  fn get_note_day_notes_by_category(&self, nd: NoteDay) -> BTreeMap<NoteCategory, Vec<Note>> {
    let mut output: BTreeMap<NoteCategory, Vec<Note>> = BTreeMap::new();
    let mut ns = self.note_day_notes(nd);
    ns.sort_by(|a, b| a.id.cmp(&b.id) );
    ns.sort_by(|a, b| a.category.cmp(&b.category) );
    for n in ns {
      output.entry(n.category).or_insert(vec![]);
      output.get_mut(&n.category).unwrap().push(n.clone());
    }
    output
  }
  fn print_current_note_day(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let nd = self.current_note_day().to_owned();
    let name = self.get_client_by_id(nd.foreign_key["client_id"]).unwrap().full_name();
    let date: String = nd.heading_date();
    let heading = format!(" {} notes for {} ", date, name);
    println_suc!("{:-^150}", &heading);
    for (k, v) in &self.get_current_note_day_notes_by_category() {
      if v.len() > 0 {
        println_suc!("{}", k);
        for n in v {
          let (output, _) = n.generate_display_content_string_with_blanks(None, None, None, None, None);
          println!("{}\n", output);
        }
      }
    }
    println_inst!("{}", "Enter any input to return to the previous menu.");
    let mut s = String::new();
    let input_attempt = io::stdin().read_line(&mut s);
    match input_attempt {
      _ => (),
    }
  }
  fn print_note_day(&self, nd: NoteDay) {
    let name = self.get_client_by_id(nd.foreign_key["client_id"]).unwrap().full_name();
    let date: String = nd.heading_date();
    let heading = format!(" {} notes for {} ", date, name);
    println_suc!("{:-^150}", &heading);
    for (k, v) in &self.get_note_day_notes_by_category(nd) {
      if v.len() > 0 {
        println_suc!("{: <150}", k);
        for n in v {
          let (output, _) = n.generate_display_content_string_with_blanks(None, None, None, None, None);
          println!("{}\n", output);
        }
      }
    }
  }
  fn display_note_day(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^162}", "-");
    
    let notes = self.current_note_day_notes();
    
    let nd = self.current_note_day();
    let c = self.get_client_by_note_day_id(nd.id).unwrap();
    let heading = format!(" Notes for {} for {} ", c.full_name(), nd.fmt_date());
    println_on_bg!("{:-^162}", heading);
    println_on_bg!("{:-^162}", "-");
    println_on_bg!("{:-^6} | {:-^35} | {:-^30} | {:-^7} | {:-^70}", " ID ", " Category ", " Topic/structure ", " Words ", " Content sample " );
    println_on_bg!("{:-^162}", "-");
    for n in notes {
      let (s, _) = n.generate_display_content_string_with_blanks(None, None, None, None, None);
      let words: Vec<String> = s.split(" ").map(|word| word.to_string() ).collect();
      let sample = if s.len() > 65 {
        format!("{}{}", String::from(&s[..60]), String::from("..."))
      } else {
        s
      };

      let cat = match n.category {
        ICCNote(c) => c.to_string(),
        FPNote(c) => c.to_string(),
      };
      let n_structure = n.structure.to_string();
      
      println_on_bg!("{: ^6} | {: ^35} | {: ^30} | {: ^7} | {: ^70}", n.id, cat, n_structure, words.len(), sample);
    }
    println_on_bg!("{:-^162}", "-");
  }
  fn display_delete_note_day(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^178}", "-");
    
    let notes = self.current_note_day_notes();
    
    let nd = self.current_note_day();
    let c = self.get_client_by_note_day_id(nd.id).unwrap();
    let heading = format!(" Delete notes for {} for {} ", c.full_name(), nd.fmt_date());
    println_on_bg!("{:-^178}", heading);
    println_on_bg!("{:-^178}", "-");
    println_on_bg!("{:-^6} | {:-^35} | {:-^30} | {:-^14} | {:-^79}", " ID ", " Category ", " Topic/structure ", " Word count ", " Content sample " );
    println_on_bg!("{:-^178}", "-");
    for n in notes {
      let (s, _) = n.generate_display_content_string_with_blanks(None, None, None, None, None);
      let words: Vec<String> = s.split(" ").map(|word| word.to_string() ).collect();
      let sample = if s.chars().count() > 75 {
        format!("{}{}", String::from(&s[..73]), String::from("..."))
      } else {
        s
      };

      let cat = match n.category {
        ICCNote(c) => c.to_string(),
        FPNote(c) => c.to_string(),
      };
      let n_structure = n.structure.to_string();
      
      println_on_bg!("{: ^6} | {: ^35} | {: ^30} | {: ^14} | {: ^79}", n.id, cat, n_structure, words.len(), sample);
    }
    println_on_bg!("{:-^178}", "-");
  }
  fn choose_delete_notes(&mut self) {
    loop {
      self.display_delete_note_day();
      println_inst!("Select note ID to delete.");
      println_inst!("CANCEL / C: Cancel");

      let mut choice = String::new();
      let read_attempt = io::stdin().read_line(&mut choice);
      let input = match read_attempt {
        Ok(_) => choice,
        Err(e) => {
          println_err!("Could not read input; try again ({}).", e);
          continue;
        }
      };
      let input = input.trim();
      match input {

        "CANCEL" | "cancel" | "Cancel" | "C" | "c" => {
          break;
        }
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_notes()
              .iter()
              .any(|&n| n.id == num) {
                println_err!("Please choose from among the listed note IDs.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note(num) {
              Ok(_) => {
                self.choose_delete_note();
                break;
              }
              Err(e) => {
                println_err!("Unable to load records with ID {}: {}", num, e);
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn choose_note_day(&mut self) {
    loop {
      self.sort_data_by_dates();
      self.display_note_day();

      println_inst!(
        "| {} | {} | {}",
        "NEW / N: new note",
        "PRINT / P: display all client notes for this day",
        "DELETE: delete individual records",
      );
      println_inst!(
        "| {} | {}",
        "DELETE ALL: delete all",
        "QUIT / Q: quit menu"
      );
      let mut choice = String::new();
      let read_attempt = io::stdin().read_line(&mut choice);
      let input = match read_attempt {
        Ok(_) => choice.to_ascii_lowercase(),
        Err(e) => {
          println_err!("Could not read input; try again ({}).", e);
          continue;
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        }
        "delete" | "d" => {
          self.choose_delete_notes();
        }
        "delete all" | "da" | "d a" => {
          self.choose_delete_note_day();
          break;
        }
        "edit" | "e" => {
          println_inst!("Choose note by ID to edit its content.");
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
        "new" | "n" => {
          self.create_note_get_id(None);
        }
        "print" | "p" => {
          self.print_current_note_day();
        }
        _ => {
          match input.to_string().parse() {
            Ok(num) => {
              if !self.current_user_notes().iter().any(|n| n.id == num ) {
                println_err!("Please choose from among the listed note IDs.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              } else {
                match self.get_note_option_by_id(num) {
                  None => {
                    println_err!("Invalid note ID.");
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  },
                  Some(_) => {
                    match self.load_note(num) {
                      Ok(_) => self.choose_note(),
                      Err(e) => {
                        println_err!("Unable to load template with ID {}: {}", num, e);
                        thread::sleep(time::Duration::from_secs(1));
                        continue;
                      }
                    }                    
                  }
                }
              }
            },
            Err(_) => {
              println_err!("Invalid command.");
              thread::sleep(time::Duration::from_secs(2));
              continue;
            },
          }
        }
      }
    }
  }
  fn create_note_day_get_id(&mut self) -> Option<u32> {
    let note_day = loop {
      let mut today_choice = String::new();
      print_inst!("Enter 'CANCEL' at any time to cancel.");
      print!("\n");
      print_inst!("Note for today? (Y/N)");
      print!("\n");
      let today_attempt = io::stdin().read_line(&mut today_choice);
      match today_attempt {
        Ok(_) => (),
        Err(e) => {
          println_err!("Invalid repsonse: {}", e);
          continue;
        }
      }
      let today = Local::now().naive_local().date();
      let date = match &today_choice.trim()[..] {
        "Y" | "y" | "Yes" | "YES" | "yes" => {
          today
        }
        "NO" | "no" | "No" | "N" | "n" => {
          let year = loop {
            let mut this_year_choice = String::new();
            println_inst!("This year ({})? (Y/N)", today.year());
            let this_year_attempt = io::stdin().read_line(&mut this_year_choice);
            match this_year_attempt {
              Ok(_) => match &this_year_choice.trim()[..] {
                "YES" | "yes" | "Yes" | "Y" | "y" => break today.year(),
                "NO" | "no" | "No" | "N" | "n" => {
                  let mut year_choice = String::new();
                  println_inst!("What year?");
                  let year_attempt = io::stdin().read_line(&mut year_choice);
                  match year_attempt {
                    Ok(_) => {
                      if year_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                        return None;
                      }
                      match year_choice.trim().parse() {
                        Ok(val) => {
                          if val > 9999 || val < 1000 {
                            println_err!("Please enter a valid year.");
                            continue;
                          } else {
                            break val;
                          }
                        }
                        Err(e) => {
                          println_err!("Invalid repsonse: {}", e);
                          continue;
                        }
                      }
                    }
                    Err(e) => {
                      println_err!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                },
                "Cancel" | "CANCEL" | "cancel" => return None,
                _ => {
                  println_err!("Please choose 'yes' or 'no.'");
                  continue;
                }
              },
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
          };
          let month = loop {
            let mut this_month_choice = String::new();
            println_inst!("This month ({})? (Y/N)", today.month());
            let this_month_attempt = io::stdin().read_line(&mut this_month_choice);
            match this_month_attempt {
              Ok(_) => match &this_month_choice.trim()[..] {
                "YES" | "yes" | "Yes" | "Y" | "y" => break today.month(),
                "NO" | "no" | "No" | "N" | "n" => {
                  let mut month_choice = String::new();
                  println_inst!("What month?");
                  let month_attempt = io::stdin().read_line(&mut month_choice);
                  match month_attempt {
                    Ok(_) => {
                      if month_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                        return None;
                      }
                      match month_choice.trim().parse() {
                        Ok(val) => {
                          if val > 12 || val < 1 {
                            println_err!("Please enter a valid month.");
                            continue;
                          } else {
                            break val;
                          }
                        }
                        Err(e) => {
                          println_err!("Invalid repsonse: {}", e);
                          continue;
                        }
                      }
                    }
                    Err(e) => {
                      println_err!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                },
                "CANCEL" | "cancel" | "Cancel" => return None,
                _ => {
                  println_err!("Please choose 'yes' or 'no.'");
                  continue;
                }
              },
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
          };
          let day = loop {
            let mut day_choice = String::new();
            println_inst!("What day?");
            let day_attempt = io::stdin().read_line(&mut day_choice);
            match day_attempt {
              Ok(_) => {
                if day_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                  return None;
                }
                match day_choice.trim().parse() {
                  Ok(val) => {
                    if val > 31 || val < 1 {
                      println_err!("Please enter a valid day.");
                      continue;
                    } else {
                      break val;
                    }
                  }
                  Err(e) => {
                    println_err!("Invalid repsonse: {}", e);
                    continue;
                  }
                }
              }
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
          };
          match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => date,
            None => {
              println_err!(
                "{}-{}-{} does not appear to be a valid date. Please try again.",
                year, month, day
              );
              continue;
            }
          }
        },
        "CANCEL" | "cancel" | "Cancel" => return None,
        _ => {
          println_err!("Invalid command.");
          continue;
        }
      };


      let client_id = match self.foreign_key.get("current_client_id") {
        Some(c) => *c,
        None => {
          loop {
            match self.specify_client(String::from("note")) {
              Some(id) => break id,
              None => {
                println_yel!("A note must be connected with a client. Cancel creating note ( Y / N )?");
                let mut answer = String::new();
                let answer_attempt = io::stdin().read_line(&mut answer);
                let final_answer = match answer_attempt {
                  Ok(_) => answer.trim().to_ascii_lowercase(),
                  Err(e) => {
                    println_err!("Failed to read line: {}", e);
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  }
                };
                match &final_answer[..] {
                  "yes" | "y" => return None,
                  _ => continue,
                }
              }
            }
          }
        }
      };
      let user_id = self.current_user().id;

      match self.generate_unique_new_note_day(date, user_id, client_id) {
        Ok(note_day) => break note_day,
        Err(e) => {
          println_err!("Failed to generate record for notes: {}", e);
          thread::sleep(time::Duration::from_secs(2));
          return None;
        }
      }

    };

    let id = note_day.id;
    self.save_note_day(note_day);
    Some(id)
  }
  fn note_day_dup_id_option(&self, date: &NaiveDate, user_id: u32, client_id: u32) -> Option<u32> {
    let dates_and_ids: Vec<(&NaiveDate, u32, u32, u32)> = self
      .note_days
      .iter()
      .map(|nd| (&nd.date, nd.foreign_key["user_id"], nd.foreign_key["client_id"], nd.id))
      .collect();

    match dates_and_ids
      .iter()
      .find(|(d, u, c, _)| d == &date && u == &user_id && c == &client_id) {
        Some(dates_and_ids_tup) => Some(dates_and_ids_tup.3),
        None => None,
      }
  }
  fn generate_unique_new_note_day(
    &mut self,
    date: NaiveDate,
    user_id: u32,
    client_id: u32,
  ) -> Result<NoteDay, String> {
    let id: u32 = self.note_days.len() as u32 + 1;

    match self.note_day_dup_id_option(&date, user_id, client_id) {
      Some(_) => Err(String::from("A note record already exists for that client on the given date.")),
      None => Ok(NoteDay::new(id, date, user_id, client_id, vec![]))
    }
  }
  pub fn read_note_days(filepath: &str) -> Result<Vec<NoteDay>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
    }

    let mut note_days: Vec<NoteDay> = vec![];

    for line in lines {
      let line_string = line?;

      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let id: u32 = values[0].parse().unwrap();
      
      let date_vec: Vec<i32> = match &values[1][..] {
        "" => vec![],
        _ => values[1]
        .split("-")
          .map(|val| val.parse().unwrap())
          .collect(),
      };

      let (year, month, day): (i32, u32, u32) = (date_vec[0], date_vec[1] as u32, date_vec[2] as u32);
      let date = NaiveDate::from_ymd(year, month, day);

      let user_id: u32 = values[2].parse().unwrap();
      let client_id: u32 = values[3].parse().unwrap();

      let notes: Vec<u32> = match &values[4][..] {
        "" => vec![],
        _ => values[4]
          .split("#")
          .map(|val| val.parse().unwrap())
          .collect(),
      };

      let nd = NoteDay::new(id, date, user_id, client_id, notes);
      note_days.push(nd);
    }
    note_days.sort_by(|a, b| a.foreign_key["client_id"].cmp(&b.foreign_key["client_id"]));
    note_days.sort_by(|a, b| a.foreign_key["user_id"].cmp(&b.foreign_key["user_id"]));
    note_days.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(note_days)
  }
  pub fn write_note_days(&self) -> std::io::Result<()> {
    let mut lines = String::from("##### note_days #####\n");
    for c in &self.note_days {
      lines.push_str(&c.to_string()[..]);
    }
    lines.push_str("##### note_days #####");
    let mut file = File::create(self.filepaths["note_day_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  fn save_note_day(&mut self, note_day: NoteDay) {

    let pos = self.note_days.binary_search_by(|nd| note_day.date.cmp(&nd.date)
      .then_with(|| nd.foreign_key["user_id"].cmp(&note_day.foreign_key["user_id"]))
      .then_with(|| nd.foreign_key["client_id"].cmp(&note_day.foreign_key["client_id"]))
    ).unwrap_or_else(|e| e);

    self.note_days.insert(pos, note_day);
    self.write_to_files();
  }
  fn choose_delete_note_day(&mut self) {
    loop {
      self.display_delete_note_day();
      println_yel!("Are you sure you want to delete all records for {} from {}?", self.current_client().full_name(), self.current_note_day().fmt_date_long());
      println_inst!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
      let mut confirm = String::new();
      let input_attempt = io::stdin().read_line(&mut confirm);
      let command = match input_attempt {
        Ok(_) => confirm.trim().to_string(),
        Err(e) => {
          println_err!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &command[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          self.delete_current_note_day();
          let new_note_day_id = self.reindex_note_days(None);
          match new_note_day_id {
            _ => (),
          }
          self.write_to_files();
          break;
        }
        _ => {
          break;
        }
      }
    }
  }
  fn delete_current_note_day(&mut self) {
    let current_note_day_notes = self.current_note_day_notes().iter().map(|co| co.id ).collect::<Vec<u32>>();
    for n_id in current_note_day_notes {
      match self.load_note(n_id) {
        Err(_) => panic!("Failed to delete note for current note day: {}. Available notes: {}.", n_id, self.notes.iter().map(|n| format!("{}", n.id) ).collect::<Vec<String>>().join(" ") ),
        Ok(_) => self.delete_current_note(),
      }
    }
    let new_note_id = self.reindex_notes(None);
    match new_note_id {
      _ => (),
    }
    let id = self.foreign_key.get("current_note_day_id").unwrap().to_owned();
    self.delete_from_blanks(String::from("note_day"), id);
    self.note_days.retain(|nd| nd.id != id);
    self.foreign_key.remove("current_note_day_id");
  }
  fn reindex_note_days(&mut self, current_id: Option<u32>) -> Option<u32> {
    let mut i: u32 = 1;
    let mut new_current_id: Option<u32> = None;
    for mut nd in &mut self.note_days {
      match current_id {
        Some(id) => {
          if nd.id == id {
            new_current_id = Some(i);
          }
        },
        None => (),
      }
      nd.id = i;
      i += 1;
    }
    new_current_id
  }
  fn get_note_day_by_id(&self, id: u32) -> Option<&NoteDay> {
    self.note_days.iter().find(|nd| nd.id == id)
  }
  /// assumes that the given note_day_id is valid
  fn get_client_by_note_day_id(&self, id: u32) -> Option<&Client> {
    self.clients.iter().find(|c| self.get_note_day_by_id(id).unwrap().foreign_key["client_id"] == c.id )
  }

// note templates

  fn current_note_template_mut(&mut self) -> &mut NoteTemplate {
    let nd_id = match self.foreign_key.get("current_note_template_id") {
      Some(id) => id,
      None => panic!("There is no current template selected."),
    };
    let maybe_current: Option<&mut NoteTemplate> = self.note_templates.iter_mut().find(|nt| nt.id == *nd_id);
    match maybe_current {
      Some(nd) => nd,
      None => panic!("The loaded template ID does not match any saved templates."),
    }
  }
  fn current_note_template(&self) -> &NoteTemplate {
    let nd_id = match self.foreign_key.get("current_note_template_id") {
      Some(id) => id,
      None => panic!("There is no current template selected."),
    };
    let maybe_current: Option<&NoteTemplate> = self.note_templates.iter().find(|nt| nt.id == *nd_id);
    match maybe_current {
      Some(nd) => nd,
      None => panic!("The loaded template ID does not match any saved templates."),
    }
  }
  fn current_user_note_templates(&self) -> Vec<&NoteTemplate> {
    self.note_templates.iter().filter(|nt|
      match nt.foreign_keys.get("user_ids") {
        Some(_) => {
          nt.foreign_keys["user_ids"].iter().any(|id| id  == &self.current_user().id )
        },
        None => false,
      }
    ).collect()
  }
  fn display_user_note_templates(&self) {
    let heading = format!(" All note templates for {} ", &self.current_user().full_name()[..]);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^156}", "-");
    println_on_bg!("{:-^156}", heading);
    println_on_bg!("{:-^156}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^100}", " ID ", " Type ", " Preview ");
    for nt in self.current_user_note_templates() {
      let mut type_string = format!("{}", nt.structure);
      if nt.custom {
        type_string.push_str(" (custom)");
      }
      println_on_bg!(
        "{: ^10} | {: ^40} | {: ^100}",
        nt.id,
        type_string,
        &nt.preview(),
      );
    }
    println_on_bg!("{:-^156}", "-");
    let current_templates = self.current_user_note_templates().clone();
    if current_templates.iter().filter(|nt| nt.custom ).count() > 0 {
      println_inst!(
        "{} | {}",
        "Choose template by ID.",
        "NEW / N: New template",
      );
      println_inst!(
        "{} | {}",
        "EDIT / E: Edit custom note templates",
        "COPY / C: Copy default/other template"
      );
    } else {
      println_inst!(
        "{} | {} | {}",
        "Choose template by ID.",
        "NEW / N: New template",
        "COPY / C: Copy template"
      );
    }
  }
  fn display_copy_user_note_templates(&self) {
    let heading = format!(" Copy note template to generate a new template ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^156}", "-");
    println_on_bg!("{:-^156}", heading);
    println_on_bg!("{:-^156}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^100}", " ID ", " Type ", " Preview ");

    let defaults: Vec<NoteTemplate> = self.note_templates.clone().iter().filter(|nt| !nt.custom ).map(|nt| nt.clone() ).collect();
    let customs: Vec<NoteTemplate> = self.note_templates.clone().iter().filter(|nt| nt.custom ).map(|nt| nt.clone() ).collect();
    let nondup_customs: Vec<NoteTemplate> = customs.iter().filter(|c_nt| !defaults.iter().any(|d_nt| d_nt.content == c_nt.content  ) ).map(|nt| nt.clone() ).collect();

    let mut display_nts: Vec<NoteTemplate> = vec![];
    for nt in defaults {
      display_nts.push(nt.clone());
    }
    for nt in nondup_customs {
      display_nts.push(nt.clone());
    }

    for nt in display_nts {
      let mut type_string = format!("{}", nt.structure);
      if nt.custom {
        type_string.push_str(" (custom)");
      }
      println_on_bg!(
        "{: ^10} | {: ^40} | {: ^100}",
        nt.id,
        type_string,
        &nt.preview(),
      );
    }
    println_on_bg!("{:-^156}", "-");
    println_inst!("| {} | {}", "Choose template to copy by ID.", "QUIT / Q: Return to my note templates");
  }
  fn display_edit_note_templates(&self) {
    let heading = format!(" Edit note templates for {} ", &self.current_user().full_name()[..]);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^156}", "-");
    println_on_bg!("{:-^156}", heading);
    println_on_bg!("{:-^156}", "-");
    println_on_bg!("{:-^10} | {:-^40} | {:-^100}", " ID ", " Type ", " Preview ");
    for nt in self.current_user_note_templates() {
      if nt.custom {
        let type_string = format!("{} (custom)", nt.structure);
        println_on_bg!(
          "{: ^10} | {: ^40} | {: ^100}",
          nt.id,
          &type_string,
          &nt.preview(),
        );
      }
    }
    println_on_bg!("{:-^156}", "-");
    println_inst!("Choose template to edit by ID.");
  }
  fn choose_edit_note_templates(&mut self) {
    loop {
      self.display_edit_note_templates();
      println_inst!("QUIT / Q: Cancel editing");
      let mut field_to_edit = String::new();
      let input_attempt = io::stdin().read_line(&mut field_to_edit);
      match input_attempt {
        Ok(_) => (),
        Err(_) => {
          println_err!("Failed to read input. Please try again.");
          continue;
        }
      }
      field_to_edit = field_to_edit.to_ascii_lowercase().trim().to_string();
      match &field_to_edit[..] {
        "quit" | "q" => {
          break ();
        }
        _ => match field_to_edit.parse() {
          Ok(num) => match self.get_note_template_option_by_id(num) {
            Some(_) => {
              if self.current_user_note_templates().iter().any(|no_t| no_t.id == num ) {
                match self.load_note_template(num) {
                  Ok(_) => {
                    if self.current_note_template().custom {
                      self.choose_edit_note_template(String::from("edit"));
                    } else {
                      panic!("Failed to load a note template by ID listed in the current user's note templates.");
                    }
                  }
                  Err(_) => panic!("Failed to load a note template by ID listed in the current user's note templates."),
                }
              } else {
                println_err!("Please choose from among the listed note templates.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
            },
            None => {
              println_err!("No note template for id '{}'", field_to_edit);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          },
          Err(e) => {
            println_err!("Unable to parse {} to int: {}", field_to_edit, e);
            thread::sleep(time::Duration::from_secs(2));
            continue;
          },
        },
      }
    }
  }
  fn display_edit_note_template(&self) {
    self.current_note_template().display_edit_content(None, None);
  }
  fn choose_edit_note_template(&mut self, s: String) {
    loop {
      self.display_edit_note_template();
      match &s[..] {
        "copy" => { // not currently being used
          println_inst!("Press ENTER to save this template to your personal list.");
          println_inst!(
            "{} | {}",
            "STRUCTURE / S: Edit structure type",
            "BLANKS / B: Edit blanks",
          );
          println_inst!(
            "{} | {}",
            "CONTENT / C: Edit default content",
            "SAVE: Save copy to your notes",
          );
        },
        "edit" => {
          println_inst!(
            "{} | {}",
            "STRUCTURE / S: Edit structure type",
            "BLANKS / B: Edit blanks",
          );
          println_inst!(
            "{} | {}",
            "CONTENT / C: Edit default content",
            "QUIT / Q: Quit menu",
          );
        },
        _ => panic!("Unsupported display purpose variable 's' passed to fn 'display_edit_note_template': {}", s),
      }
      println_inst!("Choose blank by ID to delete or change it to a different type of blank.");
      let mut field_to_edit = String::new();
      let input_attempt = io::stdin().read_line(&mut field_to_edit);
      match input_attempt {
        Ok(_) => (),
        Err(_) => {
          println_err!("Failed to read input. Please try again.");
          continue;
        }
      }
      field_to_edit = field_to_edit.trim().to_string();
      match &field_to_edit.to_ascii_lowercase()[..] {
        "quit" | "q" | "save" | "" => {
          let current_nt = self.current_note_template();
          match self.note_template_dup_already_saved(current_nt.id) {
            Some(nt_ids) => {
              let num_dups = nt_ids.len();
              for nt_id in nt_ids {
                match self.load_note_template(nt_id) {
                  Ok(_) => self.delete_current_note_template(),
                  Err(e) => panic!("Detected duplicate note template with ID '{}' after copy, loaded successfully but failed to delete: {}", nt_id, e),
                }
              }
              self.reindex_note_templates();
              self.write_to_files();
              print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
              println_yel!("Copies discarded: {}", num_dups);
              thread::sleep(time::Duration::from_secs(1));
            },
            None => (),
          }
          break;
        },
        "structure" | "s" => {
          let structure = loop {
            self.display_structure_types();
            print_inst!("Choose a new structure for the selected note template from the menu.");
            print!("\n");
            print_inst!("'CANCEL/QUIT' at any time to cancel.");
            print!("\n");
            let mut structure_choice = String::new();
            let structure_attempt = io::stdin().read_line(&mut structure_choice);
            match structure_attempt {
              Ok(_) => (),
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
            let structure = structure_choice.trim();
            let chosen_id: usize = match structure.parse() {
              Ok(num) => num,
              Err(e) => {
                if &structure[..] == "cancel" {
                  break None;
                }
                println_err!("Failed to parse '{}' as int: {}", structure, e);
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
            };
            break match StructureType::iterator().nth(chosen_id - 1) {
              None => {
                println_err!("Invalid choice.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              },
              Some(s) => Some(s.clone()),
            };
          };
          match structure {
            Some(s) => self.current_note_template_mut().structure = s,
            None => continue,
          }
        },
        "content" | "c" => {
          let mut content_focus_id: Option<u32> = Some(1);
          let mut blank_focus_id: Option<u32> = None;
          loop {
            self.current_note_template().display_edit_content(blank_focus_id, content_focus_id);
            println_inst!(
              "{} | {}",
              "EDIT / E: Edit selected content",
              "ADD / A: Add content to end of template",
            );
            println_inst!(
              "{} | {}",
              "INSERT / I: Insert additional content",
              "DELETE / D: Delete content",
            );
            println_inst!("Choose content section by ID.");
            println_inst!("Enter 'QUIT / Q' at any time to return to the editing menu.");
            let mut content_choice = String::new();
            let content_attempt = io::stdin().read_line(&mut content_choice);
            match content_attempt {
              Ok(_) => (),
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
            let content = content_choice.trim();
            match &content.to_ascii_lowercase()[..] {
              "quit" | "q" => {
                break;
              },
              "append" | "add" | "a" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                self.current_note_template().display_edit_content(None, None);
                println_inst!("Enter new content to append to the template, including spaces.");
                let mut new_section = String::new();
                let new_section_choice = loop {
                  let section_result = io::stdin().read_line(&mut new_section);
                  break match section_result {
                    Ok(_) => String::from(&new_section[..new_section.len()-2]),
                    Err(e) => {
                      println_err!("Failed to read line: {}", e);
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  };
                };

                let c_content = self.current_note_template().content.clone();
                let last_char = c_content.chars().last();
                let new_content = match last_char {
                  None => format!("{}{}", c_content, new_section_choice),
                  Some(c) => {
                    if String::from("?!.").contains(c) && new_section_choice.chars().next() != Some(' ') {
                      format!("{}{}{}", c_content, " ", new_section_choice)
                    } else {
                      format!("{}{}", c_content, new_section_choice)
                    }
                  }
                };
                self.current_note_template_mut().content = new_content;
                self.current_note_template_mut().clean_spacing();
                self.write_note_templates().unwrap()
              },
              "edit" | "e" => {
                self.current_note_template().display_edit_content(blank_focus_id, content_focus_id);
                println_inst!("Enter exact content to replace the selected text, including any spaces on the left and right.");
                let mut new_section = String::new();
                let new_section_choice = loop {
                  let section_result = io::stdin().read_line(&mut new_section);
                  break match section_result {
                    Ok(_) => String::from(&new_section[..new_section.len()-2]),
                    Err(e) => {
                      println_err!("Failed to read line: {}", e);
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  };
                };

                let nt_content = self.current_note_template().content.clone();

                let mut new_content = String::new();

                for (i, idxs) in self.current_note_template().get_content_section_indices().iter().enumerate() {
                  if i + 1 == content_focus_id.unwrap() as usize {
                    new_content = format!("{}{}{}", &nt_content[..idxs.0], &new_section_choice[..], &nt_content[idxs.1..]);
                  }
                }

                self.current_note_template_mut().content = new_content;
                self.current_note_template_mut().clean_spacing();
                self.write_note_templates().unwrap()
              },
              "delete" | "d" => {
                loop {
                  println_yel!("Are you sure you want to delete the currently selected section of content? (Y/N)");
                  let mut delete_choice = String::new();
                  let delete_attempt = io::stdin().read_line(&mut delete_choice);
                  match delete_attempt {
                    Ok(_) => (),
                    Err(e) => {
                      println_err!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                  let delete = delete_choice.trim();
                  match &delete[..] {
                    "YES" | "yes" | "Yes" | "Y" | "y" => {
                      let nt_content = self.current_note_template().content.clone();

                      let mut new_content = String::new();
                      for (i, idxs) in self.current_note_template().get_content_section_indices().iter().enumerate() {
                        if i + 1 == content_focus_id.unwrap() as usize {
                          new_content = format!("{}{}", &nt_content[..idxs.0], &nt_content[idxs.1..]);
                        }
                      }
                      self.current_note_template_mut().content = new_content;
                      self.current_note_template_mut().clean_spacing();
                      self.write_note_templates().unwrap();
                      break;
                    },
                    "NO" | "no" | "No" | "N" | "n" => {
                      break;
                    },
                    _ => {
                      println_err!("Please enter either 'YES'/'Y' or 'NO'/'N'.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              },
              "insert" | "i" => { // in content choice
                let mut chosen_text = String::new();
                'insert_content: loop {
                  
                  self.current_note_template().display_edit_content(blank_focus_id, content_focus_id);
                  
                  if chosen_text == String::new() {
                    println_inst!("Enter new content to insert, without spaces on the ends.");
                    let mut selected_text = String::new();
                    let enter_result = io::stdin().read_line(&mut selected_text);
                    chosen_text = match enter_result {
                      Ok(_) => String::from(&selected_text[..selected_text.len()-2]),
                      Err(e) => {
                        println_err!("Failed to read string: {}", e);
                        thread::sleep(time::Duration::from_secs(2));
                        continue;
                      }
                    };
                  }

                  let content_indices = self.current_note_template().get_content_section_indices();
                  println_inst!("Select a section of text in the template by ID, then ENTER to insert new content before, after, or in that section.");
                  println_inst!("Enter CANCEL at any time to cancel.");

                  let insert = loop {
                    let mut insert_choice = String::new();
                    let insert_attempt = io::stdin().read_line(&mut insert_choice);
                    match insert_attempt {
                      Ok(_) => (),
                      Err(e) => {
                        println_err!("Invalid input: {}", e);
                        thread::sleep(time::Duration::from_secs(2));
                        continue 'insert_content;
                      }
                    }
                    let insert = insert_choice.trim();
                    if content_focus_id.unwrap() == 0 {
                      if &insert[..] == "" {
                        continue;
                      }
                    }
                    break insert.to_string();
                  };
                  match &insert[..] {
                    "CANCEL" | "cancel" | "C" | "c" => {
                      break 'insert_content;
                    },
                    "" => {
                      let mut current_location = 1;
                      'insert_location_content: loop {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        let indices = content_indices[content_focus_id.unwrap() as usize - 1];
                        let idx1 = indices.0 as usize;
                        let idx2 = indices.1 as usize;
                        let display_string = format!("{}", self.current_note_template().content[idx1..idx2].to_string());
                        let num_chars = display_string.chars().count();
                        if num_chars <= 163 {
                          println_on_bg!("{}", &display_string);
                          let mut pointer_string = String::new();
                          for _ in 0..current_location {
                            pointer_string.push_str("-");
                          }
                          pointer_string.push_str("^");
                          println_on_bg!("{}", &pointer_string);
                        } else {
                          let mut outer_vec: Vec<(String, String)> = vec![];

                          let mut display_content = display_string.clone();
                          while display_content.chars().count() > 163 {
                            let (p1, p2) = display_content.split_at(163);
                            let mut pointer_string = String::new();
                            for _ in 0..163 {
                              pointer_string.push_str(" ");
                            }
                            outer_vec.push((format!("{}", p1), pointer_string));
                            display_content = format!("{}", p2);
                          }
                          if display_content == String::new() {
                            ()
                          } else {
                            let mut pointer_string = String::new();
                            for _ in 1..pointer_string.chars().count() {
                              pointer_string.push_str("-");
                            }
                            pointer_string.push_str("^");
                            outer_vec.push((format!("{}", display_content), pointer_string));
                          }
                          for row_and_pointer in outer_vec {
                            println_on_bg!("{}", row_and_pointer.0);
                            println_on_bg!("{}", row_and_pointer.1);
                          }
                        }
                        println_inst!("New content: {}", &chosen_text);
                        println_inst!("Enter an integer to navigate to a point at which to insert the new content.");
                        println_inst!("Press ENTER to insert new content in the current location.");
                        println_inst!(
                          "{} | {} | {}",
                          "BEFORE / B: Insert at start of section",
                          "AFTER / A: Insert at end of section",
                          "CANCEL / C: Cancel inserting new content",
                        );

                        let mut insert_string = String::new();
                        let insert_attempt = io::stdin().read_line(&mut insert_string);
                        match insert_attempt {
                          Ok(_) => (),
                          Err(e) => {
                            println_err!("Invalid input: {}", e);
                            thread::sleep(time::Duration::from_secs(2));
                            continue;
                          },
                        }
                        match &insert_string.trim().to_ascii_lowercase()[..] {
                          // integer to change current location
                          // ENTER to insert it there, then show what it will look like and confirm
                          "cancel" => {
                            break 'insert_location_content;
                          }
                          "before" | "b" => {
                            loop {
                              let last_content_char = self.current_note_template()
                                .generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1+1]
                                .chars()
                                .last();
                              let next_content_char = self.current_note_template()
                                .generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..]
                                .chars()
                                .next();
                              let bfrs = get_spacing_buffers(last_content_char, next_content_char, chosen_text.clone());

                              let new_content_section = format!("{}{}{}{}", &bfrs.0, &chosen_text, &bfrs.1, &display_string[..]);
                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              println_inst!("{}", &new_content_section);

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              match &confirm.to_ascii_lowercase()[..] {
                                "yes" | "y" => {
                                  let nt_content = self.current_note_template().content.clone();

                                  let mut new_content = String::new();

                                  for (i, idxs) in self.current_note_template().get_content_section_indices().iter().enumerate() {
                                    if i + 1 == content_focus_id.unwrap() as usize {
                                      new_content = format!("{}{}{}", &nt_content[..idxs.0], &new_content_section[..], &nt_content[idxs.1..]);
                                    }
                                  }
                                  self.current_note_template_mut().content = new_content;
                                  self.current_note_template_mut().clean_spacing();
                                  self.write_note_templates().unwrap();
                                  break 'insert_content;
                                },
                                "no" | "n" => {
                                  continue 'insert_location_content;
                                },
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          "after" | "a" => {
                            loop {
                              let last_content_char = display_string[..]
                                .chars()
                                .last();
                              let next_content_char = self.current_note_template()
                                .generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..]
                                .chars()
                                .next();
                              let bfrs = get_spacing_buffers(last_content_char.clone(), next_content_char, chosen_text.clone());

                              let last_char = match last_content_char {
                                Some(c) => c.to_string(),
                                None => String::new(),
                              };

                              let new_content_section = format!("{}{}{}{}", &display_string[..], &bfrs.0, &chosen_text, &bfrs.1);
                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              println_inst!("{}{}", &last_char, &new_content_section);

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              match &confirm.to_ascii_lowercase()[..] {
                                "yes" | "y" => {
                                  let nt_content = self.current_note_template().content.clone();
                                  let mut new_content = String::new();
                                  for (i, idxs) in self.current_note_template().get_content_section_indices().iter().enumerate() {
                                    if i + 1 == content_focus_id.unwrap() as usize {
                                      new_content = format!("{}{}{}", &nt_content[..idxs.0], &new_content_section[..], &nt_content[idxs.1..]);
                                    }
                                  }
                                  self.current_note_template_mut().content = new_content;
                                  self.current_note_template_mut().clean_spacing();
                                  self.write_note_templates().unwrap();
                                  break 'insert_content;
                                },
                                "no" | "n" => {
                                  continue 'insert_location_content;
                                },
                                "cancel" | "c" => {
                                  break 'insert_content;
                                },
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          "" => {
                            loop {
                              let last_content_char = if current_location == 0 {
                                self.current_note_template().generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1].chars().last()
                              } else {
                                display_string[..current_location].chars().last()
                              };
                              let next_content_char = if current_location >= &display_string.chars().count()-1 {
                                self.current_note_template().generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..].chars().next()
                              } else {
                                display_string[current_location..].chars().next()
                              };
                              let bfrs = get_spacing_buffers(last_content_char, next_content_char, chosen_text.to_string());

                              let new_content = format!(
                                "{}{}{}{}{}",
                                &display_string[..current_location],
                                &bfrs.0,
                                &chosen_text,
                                &bfrs.1,
                                &display_string[current_location..]
                              );
                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              print_on_bg!("{}{}",
                                &display_string[..current_location],
                                &bfrs.0,
                              );
                              print_highlighted_content!("{}",
                                &chosen_text,
                              );
                              print_on_bg!("{}{}\n",
                                &bfrs.1,
                                &display_string[current_location..]
                              );

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              let current_content = self.current_note_template().content.clone();

                              match &confirm[..] {
                                "YES" | "yes" | "Yes" | "Y" | "y" => {
                                  self.current_note_template_mut().content = format!(
                                    "{}{}{}",
                                    &current_content[..idx1],
                                    &new_content,
                                    &current_content[idx2..],
                                  );
                                  self.current_note_template_mut().clean_spacing();
                                  self.write_note_templates().unwrap();

                                  break 'insert_content;
                                },
                                "NO" | "no" | "No" | "N" | "n" => {
                                  continue 'insert_location_content;
                                },
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          _ => {
                            match insert_string.trim().parse() {
                              Ok(num) => {
                                if num <= chosen_text.len() {
                                  current_location = num;
                                  continue;
                                } else {
                                  println_err!("Value too high.");
                                  thread::sleep(time::Duration::from_secs(1));
                                  continue;
                                }
                              },
                              Err(_) => {
                                println_err!("Invalid command.");
                                thread::sleep(time::Duration::from_secs(2));
                                continue;
                              }
                            }
                          }
                        }
                      }
                    },
                    _ => {
                      match insert.parse::<u32>() {
                        Err(_) => {
                          println_err!("Invalid command.");
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        },
                        Ok(num) => {
                          if num > 0 && (num as usize) < content_indices.len() {
                            content_focus_id = Some(num);
                            blank_focus_id = None;
                          } else {
                            println_err!("Please enter an ID in the range shown.");
                          }
                        },
                      }
                    }
                  }
                }
                self.write_to_files();
              },
              _ => {
                match content.parse::<u32>() {
                  Err(_) => {
                    let content_indices = self.current_note_template().get_content_section_indices();
                    let (idx1, idx2) = content_indices[(content_focus_id.unwrap() - 1) as usize];
                    let last_content_char = self.current_note_template()
                      .generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1+1]
                      .chars()
                      .last();
                    let next_content_char = self.current_note_template()
                      .generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..]
                      .chars()
                      .next();
                    let bfrs = get_spacing_buffers(last_content_char, next_content_char, content.to_string());

                    let new_content_section = format!("{}{}{}", &bfrs.0, &content, &bfrs.1);
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    self.current_note_template().display_content(blank_focus_id, content_focus_id);
                    println_inst!("Press ENTER to confirm replacing selected content with '{}'.", &new_content_section);
                    println_inst!("Any other key to cancel.");
                    let mut new_section = String::new();
                    let new_section_choice = loop {
                      let section_result = io::stdin().read_line(&mut new_section);
                      break match section_result {
                        Ok(_) => new_section.trim().to_ascii_lowercase(),
                        Err(e) => {
                          println_err!("Failed to read line: {}", e);
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        }
                      };
                    };

                    match &new_section_choice[..] {
                      "" => {
                        let n_content = self.current_note_template().content.clone();
    
                        let mut new_content = String::new();
    
                        for (i, idxs) in self.current_note_template().get_content_section_indices().iter().enumerate() {
                          if i + 1 == content_focus_id.unwrap() as usize {
                            new_content = format!("{}{}{}", &n_content[..idxs.0], &new_content_section[..], &n_content[idxs.1..]);
                          }
                        }
    
                        self.current_note_template_mut().content = new_content;
                        self.current_note_template_mut().clean_spacing();
                        self.write_note_templates().unwrap()
                      },
                      _ => {
                        continue;
                      }
                    }
                  },
                  Ok(num) => {
                    let num_sections = self.current_note_template().get_num_content_sections();
                    if num as usize > num_sections || num == 0 {
                      println_err!("Please choose from among the content IDs shown above.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    } else {
                      content_focus_id = Some(num);
                    }
                  },
                }
              },
            }
          }
        },
        _ => { // THIS IS "blanks" | "b" option - because parsing to int will also lead to choosing blank condition
          let mut blank_focus_id: Option<u32> = Some(1);
          let mut content_focus_id: Option<u32> = None;
          if field_to_edit.clone() == String::from("blanks") || field_to_edit.clone() == String::from("b") {
            ()
          } else {
            match field_to_edit.parse::<u32>() {
              Ok(num) => {
                lazy_static! {
                  static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                }
                let num_blanks = RE_BLANK.find_iter(&self.current_note_template().content).count() as u32;
                if num > num_blanks || num == 0 {
                  blank_focus_id = Some(num);
                  content_focus_id = None;
                } else {
                  println_err!("Please choose from among the blank IDs shown above.");
                  thread::sleep(time::Duration::from_secs(2));
                }
              },
              Err(_) => {
                println_err!("Invalid command.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
            }
          }
          loop {
            self.current_note_template().display_edit_content(blank_focus_id, content_focus_id);
            println_inst!(
              "{} | {} | {}",
              "EDIT / E: Edit selected blank type",
              "INSERT / I: Insert new blank",
              "DELETE / D: Delete blank",
            );
            println_inst!("Choose blank by blank ID.");
            println_inst!("Enter 'QUIT / Q' at any time to return to the editing menu.");
            let mut blank_choice = String::new();
            let blank_attempt = io::stdin().read_line(&mut blank_choice);
            match blank_attempt {
              Ok(_) => (),
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
            let blank = blank_choice.trim();
            match &blank.to_ascii_lowercase()[..] {
              "quit" | "q" => {
                break;
              },
              "edit" | "e" => {
                display_blanks_empty();
                let b_idx_opt = choose_blanks_option();
                let b = match b_idx_opt {
                  None => break,
                  Some(b_idx) => {
                    let new_b = Blank::vector_of_variants()[b_idx];
                    let num_blanks = RE_BLANK.find_iter(&self.current_note_template().content).count() as u32;
                    let connected_blank_id: Option<u32> = match new_b {
                      Pronoun1ForBlank(_) | Pronoun2ForBlank(_) | Pronoun3ForBlank(_) | Pronoun4ForBlank(_) => {
                        loop {
                          self.current_note_template().display_edit_content(Some(0), None);
                          println_inst!("Please enter the ID of the blank to add pronouns for. Enter CANCEL / C to cancel.");
                          let mut input_string = String::new();
                          let input_result = io::stdin().read_line(&mut input_string);
                          let input = match input_result {
                            Ok(_) => input_string.trim(),
                            Err(e) => {
                              println_err!("Failed to read input: {}.", e);
                              thread::sleep(time::Duration::from_secs(1));
                              continue;
                            }
                          };
                          if &input.to_ascii_lowercase()[..] == "cancel" {
                            break None;
                          }
                          match input.parse() {
                            Ok(num) => {
                              if num > num_blanks {
                                println_err!("Please choose an integer in the range of the number of current blanks already added to this template.");
                                thread::sleep(time::Duration::from_secs(1));
                                continue;
                              } else {
                                break Some(num);
                              }
                            },
                            Err(_) => {
                              println_err!("Invalid command.");
                              thread::sleep(time::Duration::from_secs(1));
                              continue;
                            }
                          }
                        }
                      },
                      _ => None,
                    };
                    match new_b {
                      Pronoun1ForBlank(_) | Pronoun2ForBlank(_) | Pronoun3ForBlank(_) | Pronoun4ForBlank(_) => {
                        if let None = connected_blank_id {
                          println_yel!("Blank discarded.");
                          thread::sleep(time::Duration::from_secs(1));
                          continue;
                        }
                        match new_b {
                          Pronoun1ForBlank(_) => {
                            Pronoun1ForBlank(connected_blank_id)
                          },
                          Pronoun2ForBlank(_) => {
                            Pronoun2ForBlank(connected_blank_id)
                          },
                          Pronoun3ForBlank(_) => {
                            Pronoun3ForBlank(connected_blank_id)
                          },
                          Pronoun4ForBlank(_) => {
                            Pronoun4ForBlank(connected_blank_id)
                          },
                          _ => panic!("There was another blank type after a blank ID was already established as being needed for a blank ID connection."),
                        }
                      },
                      _ => new_b,
                    }
                  }
                };
                
                let nt_content = self.current_note_template().content.clone();

                lazy_static! {
                  static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                }

                let mut new_content = String::new();
                for (i, m) in RE_BLANK.find_iter(&nt_content).enumerate() {
                  if i + 1 == blank_focus_id.unwrap() as usize {
                    new_content = format!("{}{}{}", &nt_content[..m.start()], &b.to_string()[..], &nt_content[m.end()..]);
                  }
                }

                self.current_note_template_mut().content = new_content;
                self.current_note_template_mut().clean_spacing();
                self.write_note_templates().unwrap();
              },
              "delete" | "d" => {
                loop {
                  println_yel!("Are you sure you want to delete the selected blank? (Y/N)");
                  let mut delete_choice = String::new();
                  let delete_attempt = io::stdin().read_line(&mut delete_choice);
                  match delete_attempt {
                    Ok(_) => (),
                    Err(e) => {
                      println_err!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                  let delete = delete_choice.trim();
                  match &delete[..] {
                    "YES" | "yes" | "Yes" | "Y" | "y" => {
                      let nt_content = self.current_note_template().content.clone();

                      lazy_static! {
                        static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                      }

                      let mut new_content = String::new();
                      for (i, m) in RE_BLANK.find_iter(&nt_content).enumerate() {
                        if i + 1 == blank_focus_id.unwrap() as usize {
                          new_content = format!("{}{}", &nt_content[..m.start()], &nt_content[m.end()..]);
                        }
                      }
                      self.current_note_template_mut().content = new_content;
                      self.current_note_template_mut().clean_spacing();
                      self.write_note_templates().unwrap();
                      break;
                    },
                    "NO" | "no" | "No" | "N" | "n" => {
                      break;
                    },
                    _ => {
                      println_err!("Please enter either 'YES'/'Y' or 'NO'/'N'.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              },
              "insert" | "i" => { // in blanks choice
                let new_blank = match choose_blanks_option() {
                  Some(nb) => Blank::vector_of_variants()[nb],
                  None => continue,
                };
                blank_focus_id = None;
                content_focus_id = Some(1);
                'insert_loop: loop {
                  let current_note_template = self.current_note_template();
                  let typed_content_indices = current_note_template.get_content_section_indices();
                  current_note_template.display_edit_content(blank_focus_id, content_focus_id);
                  println_inst!("Select a section of text in the template by ID, then ENTER to insert a new blank before, after, or in that section.");
                  println_inst!("Enter CANCEL at any time to cancel.");
                  let mut insert_choice = String::new();
                  let insert_attempt = io::stdin().read_line(&mut insert_choice);
                  match insert_attempt {
                    Ok(_) => (),
                    Err(e) => {
                      println_err!("Invalid input: {}", e);
                      continue;
                    }
                  }
                  let insert = insert_choice.trim();
                  match &insert[..] {
                    "CANCEL" | "cancel" | "C" | "c" => {
                      break;
                    },
                    "" => {
                      let mut new_blanks = current_note_template.get_ordered_blanks();
                      let indices = typed_content_indices[content_focus_id.unwrap() as usize - 1];
                      let idx1 = indices.0 as usize;
                      let idx2 = indices.1 as usize;
                      let display_string = current_note_template.content[idx1..idx2].to_string();
                      let num_blanks_before_idx = current_note_template.num_blanks_before_idx(idx1);
                      new_blanks.insert(num_blanks_before_idx, new_blank.clone());
                      let mut current_location = 1;
                      'insert_location_blanks: loop {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        let num_chars = display_string.chars().count();
                        if num_chars <= 163 {
                          println_on_bg!("{}", &display_string);
                          let mut pointer_string = String::new();
                          for _ in 0..current_location {
                            pointer_string.push_str("-");
                          }
                          pointer_string.push_str("^");
                          println_on_bg!("{}", &pointer_string);
                        } else {
                          let mut outer_vec: Vec<(String, String)> = vec![];

                          let mut display_content = self.current_note_template().content[idx1..idx2].to_string();
                          while display_content.chars().count() > 163 {
                            let (p1, p2) = display_content.split_at(163);
                            let mut pointer_string = String::new();
                            for _ in 0..163 {
                              pointer_string.push_str(" ");
                            }
                            outer_vec.push((format!("{}", p1), pointer_string));
                            display_content = format!("{}", p2);
                          }
                          if display_content == String::new() {
                            ()
                          } else {
                            let mut pointer_string = String::new();
                            for _ in 1..pointer_string.chars().count() {
                              pointer_string.push_str("-");
                            }
                            pointer_string.push_str("^");
                            outer_vec.push((format!("{}", display_content), pointer_string));
                          }
                          for row_and_pointer in outer_vec {
                            println_on_bg!("{}", row_and_pointer.0);
                            println_on_bg!("{}", row_and_pointer.1);
                          }
                        }
                        println_inst!("Enter an integer to navigate to a point at which to insert the new blank.");
                        println_inst!("Press ENTER to insert the selected blank type in the current location.");
                        println_inst!("CANCEL: Cancel inserting blank");

                        let mut insert_string = String::new();
                        let insert_attempt = io::stdin().read_line(&mut insert_string);
                        match insert_attempt {
                          Ok(_) => (),
                          Err(e) => {
                            println_err!("Invalid input: {}", e);
                            thread::sleep(time::Duration::from_secs(2));
                            continue;
                          },
                        }
                        match &insert_string.trim()[..] {
                          // integer to change current location
                          // ENTER to insert it there, then show what it will look like and confirm
                          "cancel" => {
                            break 'insert_loop;
                          }
                          "" => {
                            loop {

                              let last_content_char = if current_location == 0 {
                                self.current_note_template().generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1].chars().last()
                              } else {
                                display_string[..current_location].chars().last()
                              };
                              let next_content_char = if current_location >= &display_string.chars().count()-1 {
                                self.current_note_template().generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..].chars().next()
                              } else {
                                display_string[current_location..].chars().next()
                              };
                              let bfrs = get_spacing_buffers(last_content_char, next_content_char, new_blank.to_string());

                              let new_content = format!(
                                "{}{}{}{}{}",
                                &display_string[..current_location],
                                &bfrs.0,
                                new_blank,
                                &bfrs.1,
                                &display_string[current_location..]
                              );
                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              print_inst!("{}{}",
                                &display_string[..current_location],
                                &bfrs.0,
                              );
                              print_highlighted_content!("{}",
                                &new_blank.display_to_user_empty()[..],
                              );
                              print_inst!("{}{}\n",
                                &bfrs.1,
                                &display_string[current_location..]
                              );

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              let current_content = self.current_note_template().content.clone();

                              match &confirm.to_ascii_lowercase()[..] {
                                "yes" | "y" => {
                                  let mut edited_content = format!(
                                    "{}{}{}",
                                    &current_content[..idx1],
                                    &new_content,
                                    &current_content[idx2..],
                                  );
                                  let formatted_blanks = NoteTemplate::get_blanks_with_new_ids(new_blanks.clone(), content_focus_id.unwrap() - 1);

                                  lazy_static! {
                                    static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                                  }

                                  for (i, m) in RE_BLANK.find_iter(&edited_content.clone()).enumerate() {
                                    if i + 1 == content_focus_id.unwrap() as usize {
                                      edited_content = format!("{}{}{}", &edited_content[..m.start()], &formatted_blanks[i].to_string()[..], &edited_content[m.end()..]);
                                    }
                                  }

                                  self.current_note_template_mut().content = edited_content;
                                  self.current_note_template_mut().clean_spacing();
                                  self.write_note_templates().unwrap();
                                  content_focus_id = None; // because after inserting, we will go back to focusing on blank
                                  break 'insert_loop;
                                },
                                "no" | "n" => {
                                  continue 'insert_location_blanks;
                                },
                                "cancel" | "c" => {
                                  content_focus_id = None; // because after inserting, we will go back to focusing on blank
                                  break 'insert_loop;
                                },
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          "before" | "b" => {
                            loop {
                              let last_content_char = if current_location == 0 {
                                self.current_note_template().generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1].chars().last()
                              } else {
                                display_string[..current_location].chars().last()
                              };
                              let next_content_char = if current_location >= &display_string.chars().count()-1 {
                                self.current_note_template().generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..].chars().next()
                              } else {
                                display_string[current_location..].chars().next()
                              };
                              let bfrs = get_spacing_buffers(last_content_char, next_content_char, new_blank.display_to_user_empty());

                              let new_display_content = format!(
                                "{}{}{}{}",
                                &bfrs.0,
                                &new_blank.display_to_user_empty()[..],
                                &bfrs.1,
                                &display_string[..]
                              );

                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              println_inst!("{}", &new_display_content);

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              let current_content = self.current_note_template().content.clone();

                              match &confirm[..] {
                                "yes" | "y" => {
                                  self.current_note_template_mut().content = format!(
                                    "{}{}",
                                    &new_blank,
                                    &current_content[..],
                                  );
                                  self.current_note_template_mut().clean_spacing();
                                  self.write_note_templates().unwrap();
                                  content_focus_id = None; // because after inserting, we will go back to focusing on blank
                                  break 'insert_location_blanks;
                                },
                                "no" | "n" => {
                                  continue 'insert_location_blanks;
                                },
                                "cancel" => {
                                  content_focus_id = None; // because after inserting, we will go back to focusing on blank
                                  break 'insert_loop;
                                }
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          "after" | "a" => {
                            loop {
                              let last_content_char = if current_location == 0 {
                                self.current_note_template().generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1].chars().last()
                              } else {
                                display_string[..current_location].chars().last()
                              };
                              let next_content_char = if current_location >= &display_string.chars().count()-1 {
                                self.current_note_template().generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..].chars().next()
                              } else {
                                display_string[current_location..].chars().next()
                              };
                              let bfrs = get_spacing_buffers(last_content_char, next_content_char, new_blank.to_string());

                              let new_content = format!(
                                "{}{}{}{}",
                                &display_string[..],
                                &bfrs.0,
                                new_blank,
                                &bfrs.1
                              );
                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              println_inst!("{}", &new_content);

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              let current_content = self.current_note_template().content.clone();

                              match &confirm[..] {
                                "yes" | "y" => {
                                  self.current_note_template_mut().content = format!(
                                    "{}{}",
                                    &current_content[..],
                                    &new_blank,
                                  );
                                  self.current_note_template_mut().clean_spacing();
                                  self.write_note_templates().unwrap();
                                  break 'insert_location_blanks;
                                },
                                "no" | "n" => {
                                  continue 'insert_location_blanks;
                                },
                                "cancel" => {
                                  content_focus_id = None; // because after inserting, we will go back to focusing on blank
                                  break 'insert_loop;
                                },
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          _ => {
                            match insert_string.trim().parse() {
                              Ok(num) => {
                                let num_chars = self.current_note_template().content.chars().count();
                                if num > num_chars {
                                  current_location = num_chars;
                                } else {
                                  current_location = num;
                                }
                                continue;
                              },
                              Err(_) => {
                                println_err!("Invalid command.");
                                thread::sleep(time::Duration::from_secs(2));
                                continue;
                              }
                            }
                          }
                        }
                      }
                    },
                    _ => {
                      match insert.parse::<u32>() {
                        Err(_) => {
                          println_err!("Invalid command.");
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        },
                        Ok(num) => {
                          if num as usize > typed_content_indices.len() || num as usize == 0 {
                            println_err!("Please choose from among the displayed content indices.");
                            thread::sleep(time::Duration::from_secs(2));
                            continue;
                          } else {
                            content_focus_id = Some(num);
                            blank_focus_id = None;
                            continue;
                          }
                        },
                      }
                    }
                  }
                }
                self.write_note_templates().unwrap();
              },
              _ => {
                let nt_content = self.current_note_template().content.clone();
                lazy_static! {
                  static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                }

                let num_matches = RE_BLANK.find_iter(&nt_content).count();

                let u32_num = blank.parse::<u32>();

                match blank.parse::<usize>() {
                  Ok(num) => if num <= num_matches {
                    blank_focus_id = Some(u32_num.unwrap());
                    content_focus_id = None; // because after inserting, we will go back to focusing on blank
                    continue;
                  } else {
                    println_err!("Please choose a blank by ID among those displayed in the note template's content.");
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  },
                  Err(_) => {
                    println_err!("Invalid command.");
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  }
                }                  
              }
            }
          }
        },
      }
    }
  }
  fn choose_note_templates(&mut self) {
    loop {
      let input = loop {
        self.display_user_note_templates();
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_note_template_get_id();
          match maybe_new_id {
            Some(_) => (),
            None => (),
          }
        },
        "edit" | "e" => {
          let current_templates = self.current_user_note_templates().clone();
          if current_templates.iter().filter(|nt| nt.custom ).count() > 0 {
            self.choose_edit_note_templates();
          } else {
            println_err!("There are no custom templates to edit.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
        },
        "copy" | "c" => {
          self.choose_copy_note_template();
        },
        "quit" | "q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_note_templates()
              .iter()
              .any(|&nt| nt.id == num) {
                println_err!("Please choose from among the listed templates, or 'NEW / N' to create a new template.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note_template(num) {
              Ok(_) => self.choose_note_template(),
              Err(e) => {
                println_err!("Unable to load template with ID {}: {}", num, e);
                thread::sleep(time::Duration::from_secs(1));
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
          }
        },
      }
    }
  }
  fn select_note_template(&mut self) -> Option<u32> {
    loop {
      let input = loop {
        self.display_user_note_templates();
        let mut choice = String::new();
        let read_attempt = io::stdin().read_line(&mut choice);
        match read_attempt {
          Ok(_) => break choice.to_ascii_lowercase(),
          Err(e) => {
            println_err!("Could not read input; try again ({}).", e);
            continue;
          }
        }
      };
      let input = input.trim();
      match input {
        "new" | "n" => {
          let maybe_new_id = self.create_note_template_get_id();
          match maybe_new_id {
            Some(_) => (),
            None => (),
          }
          continue;
        },
        "edit" | "e" => {
          self.choose_edit_note_templates();
          continue;
        },
        "copy" | "c" => {
          self.choose_copy_note_template();
        },
        "quit" | "q" => {
          break None;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_note_templates()
              .iter()
              .any(|&nt| nt.id == num) {
                println_err!("Please choose from among the listed templates, or 'NEW / N' to create a new template.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note_template(num) {
              Ok(_) => break Some(num),
              Err(e) => {
                println_err!("Unable to load template with ID {}: {}", num, e);
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
          },
          Err(e) => {
            println_err!("Could not read input as a number; try again ({}).", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          }
        },
      }
    }
  }
  fn display_note_template(&self) {
    self.current_note_template().display_content(None, None);
  }
  fn choose_note_template(&mut self) {
    loop {

      self.display_note_template();

      if self.current_note_template().custom {
        println_inst!(
          "| {} | {} | {}",
          "NOTE / N: use template for a new note",
          "COPY / C: make copy for new custom template",
          "EDIT / E: edit template",
        );
        println_inst!(
          "| {} | {}",
          "DELETE: delete template",
          "QUIT / Q: quit menu"
        );
      } else {
        println_inst!(
          "| {} | {} | {}",
          "NOTE / N: Use template for a new note",
          "COPY / C: create new custom template from this template",
          "QUIT / Q: quit menu"
        );
      }
      let mut choice = String::new();
      let read_attempt = io::stdin().read_line(&mut choice);
      let input = match read_attempt {
        Ok(_) => choice.to_ascii_lowercase(),
        Err(e) => {
          println_err!("Could not read input; try again ({}).", e);
          continue;
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        }
        "delete" | "d" => {
          if self.current_note_template().custom {
            self.choose_delete_note_template();
            self.write_note_templates().unwrap();
            break;
          } else {
            println_err!("Cannot delete default note.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
        }
        "note" | "n" => {
          let n_id = self.create_note_from_template_get_id(Some(self.current_note_template().id));
          match n_id {
            Some(id) => {
              println_suc!("Note #{} added to record.", id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            },
            None => {
              println_yel!("Note discarded.");
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        }
        "copy" | "c" => {
          self.copy_note_template(self.current_note_template().id);
          break;
        }
        "edit" | "e" => {
          if self.current_note_template().custom {
            self.choose_edit_note_template(String::from("edit"));
          } else {
            println_err!("Cannot edit default note templates. To create a custom version, use the 'COPY' command and edit the copy.");
            thread::sleep(time::Duration::from_secs(4));
            continue;
          }
        }
        _ => println_err!("Invalid command."),
      }
    }
  }
  fn display_structure_types(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^96}", "-");
    println_on_bg!("{: ^10} | {: ^40} | {: ^40}", " ID ", " Note/template type ", " Abbreviation ");
    println_on_bg!("{:-^96}", "-");
    for (i, st) in StructureType::iterator().enumerate() {
      println_on_bg!(
        "{: ^10} | {: ^40} | {: ^40}",
        i+1,
        &format!(" {} ", st),
        &format!(" {} ", st.abbreviate()),
      );
    }
    println_on_bg!("{:-^96}", "-");
    println_inst!("| {}", "Choose template type by name, ID, or abbreviation.");
  }
  fn load_note_template(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&NoteTemplate> = self.note_templates.iter().find(|nt| nt.id == id);
    match current {
      Some(nt) => {
        self.foreign_key.insert(String::from("current_note_template_id"), nt.id);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to read selected template from filepath.",
      )),
    }
  }
  fn create_note_template_get_id(&mut self) -> Option<u32> {
    let mut note_template = loop {
      let structure = loop {
        self.display_structure_types();
        print_inst!("Enter 'CANCEL' at any time to cancel.");
        print!("\n");
        print_inst!("Build a template for what kind of record?");
        print!("\n");
        let mut structure_choice = String::new();
        let structure_attempt = io::stdin().read_line(&mut structure_choice);
        match structure_attempt {
          Ok(_) => (),
          Err(e) => {
            println_err!("Invalid repsonse: {}", e);
            continue;
          }
        }
        let structure = structure_choice.trim();
        let chosen_id: usize = match structure.parse() {
          Ok(num) => num,
          Err(e) => {
            if &structure[..] == "cancel" || &structure[..] == "c" {
              return None;
            }
            println_err!("Failed to parse '{}' as int: {}", structure, e);
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
        };
        match StructureType::iterator().nth(chosen_id - 1) {
          None => {
            println_err!("Invalid choice.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          },
          Some(s) => break s.clone(),
        }
      };
      let mut content = String::new();
      let mut display_content = String::new();
      let mut nt = NoteTemplate::new(0, structure.clone(), true, String::new(), vec![]);
      loop {
        nt.content = content.clone();
        nt.clean_spacing();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        nt.display_content(None, None);
        println_inst!("Enter text to add text to the template, or choose from among the following options:");
        println_inst!(
          "{} | {}",
          "TEXT / T: add custom text",
          "BLANK / B: add custom blank",
        );
        println_inst!(
          "{} | {}",
          "SAVE / S: finish and save template",
          "BACK: Delete last sentence or blank",
        );
        let mut custom_choice = String::new();
        let custom_attempt = io::stdin().read_line(&mut custom_choice);
        let custom_choice = match custom_attempt {
          Ok(_) => custom_choice,
          Err(e) => {
            println_err!("Invalid repsonse: {}", e);
            continue;
          }
        };
        let custom = custom_choice.trim();
        match &custom[..] {
          "back" => {
            let current_content = content.split(".");
            let num_sentences = content.split(".").count() - 1;
            let new_content = current_content.enumerate()
              .filter(|(i, _)| i != &num_sentences )
              .map(|(_, ele)| ele.trim() )
              .collect::<Vec<&str>>()
              .join(". ");
            content = new_content;

            let current_display_content = display_content.split(".");
            let num_display_sentences = display_content.split(".").count() - 1;
            let new_display_content = current_display_content.enumerate()
              .filter(|(i, _)| i != &num_display_sentences )
              .map(|(_, ele)| ele.trim() )
              .collect::<Vec<&str>>()
              .join(". ");
            display_content = new_display_content;
          },
          "text" | "t" | "TEXT" | "Text" | "T" => {
            loop {
              println_inst!("Enter custom text as you would like it to appear.");
              let mut text_choice = String::new();
              let text_attempt = io::stdin().read_line(&mut text_choice);
              match text_attempt {
                Ok(_) => {
                  match content.chars().last() {
                    Some(c) => {
                      if c.is_whitespace() && c != '.' && c != '!' && c != '?' || &custom.trim()[..] == "." || &custom.trim()[..] == "!" {
                        content.push_str(&text_choice.trim()[..]);
                        display_content.push_str(&text_choice.trim()[..]);
                      } else {
                        content.push_str(&format!(" {}", &text_choice.trim()[..]));
                        display_content.push_str(&format!(" {}", &text_choice.trim()[..]));
                      }
                    },
                    None => {
                      content.push_str(&text_choice.trim()[..]);
                      display_content.push_str(&text_choice.trim()[..]);
                    }
                  }
                  break;
                },
                Err(e) => {
                  println_err!("Invalid repsonse: {}", e);
                  continue;
                }
              }
            }
          },
          "blank" | "b" | "BLANK" | "Blank" | "B" => {
            let blank_idx = choose_blanks();
            let chosen_blank = Blank::vector_of_variants()[blank_idx];
            
            lazy_static! {
              static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
            }

            let num_blanks = RE_BLANK.find_iter(&content).count();
            
            let connected_blank_id: Option<usize> = match chosen_blank {
              Pronoun1ForBlank(_) | Pronoun2ForBlank(_) | Pronoun3ForBlank(_) | Pronoun4ForBlank(_) => {
                loop {
                  nt.display_content(Some(0), None);
                  println_inst!("Please enter the ID of the blank to derive pronouns. Enter CANCEL / C to cancel.");
                  let mut input_string = String::new();
                  let input_result = io::stdin().read_line(&mut input_string);
                  let input = match input_result {
                    Ok(_) => input_string.trim(),
                    Err(e) => {
                      println_err!("Failed to read input: {}.", e);
                      thread::sleep(time::Duration::from_secs(1));
                      continue;
                    }
                  };
                  if &input.to_ascii_lowercase()[..] == "cancel" {
                    break None;
                  }
                  match input.parse() {
                    Ok(num) => {
                      if num > num_blanks {
                        println_err!("Please choose an integer in the range of the number of current blanks already added to this template.");
                        thread::sleep(time::Duration::from_secs(1));
                        continue;
                      } else {
                        break Some(num);
                      }
                    },
                    Err(_) => {
                      println_err!("Invalid command.");
                      thread::sleep(time::Duration::from_secs(1));
                      continue;
                    }
                  }
                }
              },
              _ => None,
            };
            let (blank_string, display_blank_string) = match connected_blank_id {
              Some(num) => {
                let new_blank: Blank = match chosen_blank {
                  Pronoun1ForBlank(_) => Pronoun1ForBlank(Some(num as u32)),
                  Pronoun2ForBlank(_) => Pronoun2ForBlank(Some(num as u32)),
                  Pronoun3ForBlank(_) => Pronoun3ForBlank(Some(num as u32)),
                  Pronoun4ForBlank(_) => Pronoun4ForBlank(Some(num as u32)),
                  _ => panic!("User was asked to select connected_blank_id even though blank type is not linked to another blank."),
                };
                (format!("{}", new_blank), new_blank.display_to_user())
              },
              None => (format!("{}", chosen_blank), chosen_blank.display_to_user()),
            };
            match content.chars().last() {
              Some(c) => {
                if c.is_whitespace() && c != '.' && c != '!' && c != '?' {
                  content.push_str(&format!("{}", blank_string));
                  display_content.push_str(&format!("[ {} ]", display_blank_string));
                } else {
                  content.push_str(&format!(" {}", blank_string));
                  display_content.push_str(&format!(" [ {} ]", display_blank_string));
                }
              },
              None => {
                content.push_str(&format!("{}", blank_string));
                display_content.push_str(&format!("[ {} ]", display_blank_string));
              }
            }
            continue;
          },
          "save" | "s" | "SAVE" | "Save" | "S" => {
            if display_content.len() == 0 {
              println_err!("You must add at least one field to the content of a template. Please add either some text or at least one blank.");
              thread::sleep(time::Duration::from_secs(4));
              continue;
            }
            break;
          },
          "cancel" | "Cancel" | "CANCEL" => return None,
          _ => {
            match content.chars().last() {
              Some(c) => {
                if c.is_whitespace() && c != '.' && c != '!' && c != '?' || &custom.trim()[..] == "." || &custom.trim()[..] == "!" {
                  content.push_str(&custom.trim()[..]);
                  display_content.push_str(&custom.trim()[..]);
                } else {
                  content.push_str(&format!(" {}", &custom.trim()[..]));
                  display_content.push_str(&format!(" {}", &custom.trim()[..]));
                }
              },
              None => {
                content.push_str(&custom.trim()[..]);
                display_content.push_str(&custom.trim()[..]);
              }
            }
          }
        }
      };

      match self.generate_unique_new_note_template(structure, content, self.current_user().id) {
        Ok(nt) => break nt,
        Err((_, e)) => {
          println_err!("Failed to generate template: {}", e);
          thread::sleep(time::Duration::from_secs(3));
          continue;
        }
      }
    };

    note_template.clean_spacing();
    let id = note_template.id;
    self.save_note_template(note_template);
    Some(id)
  }
  fn copy_note_template(&mut self, nt_id: u32) {
    let copied_nt = match self.get_note_template_option_by_id(nt_id) {
      Some(nt) => nt,
      None => panic!("Invalid note template ID passed to copy_note_template."),
    };
    let nt_structure = copied_nt.structure.clone();
    let nt_content = copied_nt.content.clone();
    let new_nt = match self.generate_unique_new_note_template(nt_structure, nt_content, self.current_user().id) {
      Ok(nt) => nt,
      Err((nt, es)) => {
        if es == String::from("match") {
          nt
        } else {
          panic!("Failed to generate new note template to create copy for reason other than 'match' with existing note template.");
        }
      }
    };
    let new_id = new_nt.id;
    self.save_note_template(new_nt);
    self.write_to_files();
    match self.load_note_template(new_id) {
      Ok(_) => (),
      Err(e) => panic!("Failed to load note template immediately following generation in copy_note_template: {}.", e),
    }
  }
  fn choose_copy_note_template(&mut self) {
    loop {
      self.display_copy_user_note_templates();
      let mut input_string = String::new();
      let input_result = io::stdin().read_line(&mut input_string);
      let user_choice = match input_result {
        Ok(_) => input_string.trim(),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        },
      };
      match &user_choice[..] {
        "cancel" | "CANCEL" | "Cancel" | "C" | "c" | "quit" | "q" | "" => {
          break;
        },
        _ => {
          match user_choice.parse() {
            Ok(num) => {
              let defaults: Vec<NoteTemplate> = self.note_templates.clone().iter().filter(|nt| !nt.custom ).map(|nt| nt.clone() ).collect();
              let customs: Vec<NoteTemplate> = self.note_templates.clone().iter().filter(|nt| nt.custom ).map(|nt| nt.clone() ).collect();
              let nondup_customs: Vec<NoteTemplate> = customs.iter().filter(|c_nt| !defaults.iter().any(|d_nt| d_nt.content == c_nt.content  ) ).map(|nt| nt.clone() ).collect();

              let mut display_nts: Vec<NoteTemplate> = vec![];
              for nt in defaults {
                display_nts.push(nt.clone());
              }
              for nt in nondup_customs {
                display_nts.push(nt.clone());
              }
              match self.load_note_template(num) {
                Ok(_) => {
                  if !display_nts.iter().any(|nt| nt.id == num ) {
                    println_err!("Please choose from among the listed templates.");
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  } else {
                    self.copy_note_template(num);
                  }
                },
                Err(_) => {
                  println_err!("Please choose from among the listed templates.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              }
            },
            Err(_) => {
              println_err!("Invalid command: '{}'", &user_choice);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        }
      }

    }
  }
  fn note_template_dup_already_saved(&self, nt_id: u32) -> Option<Vec<u32>> {
    let nt_match = match self.note_templates.iter().find(|nt| nt.id == nt_id ) {
      Some(nt) => nt.clone(),
      None => return None,
    };
    let matches: Vec<u32> = self.note_templates.iter().filter(|nt| nt.structure == nt_match.structure
      && nt.content == nt_match.content
      && nt.foreign_key.get("user_id").as_ref() == nt_match.foreign_key.get("user_id").as_ref()
      && nt.id != nt_match.id
    )
    .map(|nt| nt.id )
    .collect();

    if matches.len() > 0 {
      Some(matches)
    } else {
      None
    }

  }
  fn note_template_dup_id_option(&self, structure: &StructureType, content: String, user_id: u32) -> Option<u32> {
    let template_fields: Vec<(&StructureType, &str, Option<&u32>, u32)> = self
      .note_templates
      .iter()
      .map(|nt| (
        &nt.structure,
        &nt.content[..],
        nt.foreign_key.get("user_id"),
        nt.id)
      )
      .collect();

    match template_fields
      .iter()
      .find(|(s, c, u, _)| s == &structure && c == &&content[..] && u == &Some(&user_id)) {
        Some(field_tup) => Some(field_tup.3),
        None => None,
      }
  }
  fn generate_unique_new_note_template(
    &mut self,
    structure: StructureType,
    content: String,
    user_id: u32,
  ) -> Result<NoteTemplate, (NoteTemplate, String)> {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    let matches = RE_BLANK.find_iter(&content);
    let mut changed_content = content.clone();
    for m in matches {
      for idx in m.start()..m.end() {
        changed_content = format!(
          "{}{}{}",
          &changed_content[..idx],
          "X",
          &changed_content[idx+1..]
        );
      }
    }
    if changed_content.contains(" | ") || changed_content.contains("(---") || changed_content.contains("---)") {
      return Err(
        (
          NoteTemplate::new(0, structure, true, content, vec![user_id]),
          String::from("The following strings are not allowed: ' | ', '(---', and '---)'. ")
        )
      );
    }
    
    let id = (self.note_templates.len() + 1) as u32;

    match self.note_template_dup_id_option(&structure, content.clone(), user_id) {
      Some(_) => Err(
        (
          NoteTemplate::new(id, structure, true, content, vec![user_id]),
          String::from("match") // must remain "match" in order to be caught in error handling for when copying a note template intentionally
        )
      ),
      None => Ok(NoteTemplate::new(id, structure, true, content, vec![user_id]))
    }
  }
  pub fn read_note_templates(filepath: &str) -> Result<Vec<NoteTemplate>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
    }

    let mut note_templates: Vec<NoteTemplate> = vec![];

    for line in lines {
      let line_string = line?;

      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let id: u32 = values[0].parse().unwrap();

      let structure = match &values[1][..] {
        "Care Plan" => CarePlan,
        "Intake" => Intake,
        "Assessment" => Assessment,
        "SNCD" => Sncd,
        "Home Visit" => HomeVisit,
        "Agenda Prep" => AgendaPrep,
        "Debrief" => Debrief,
        "Phone Call" => PhoneCall,
        "Scheduling" => Scheduling,
        "Sent Email" => SentEmail,
        "Referral" => Referral,
        "Custom Structure" => CustomStructure,
        "Parent Support" => ParentSupport,
        "Sent Cancellation" => SentCancellation,
        "Parent Appearance" => ParentAppearance,
        "Parent Skills" => ParentSkills,
        "Failed Contact Attempt" => FailedContactAttempt,
        "Categorized Emails" => CategorizedEmails,
        "Authorization Requested" => AuthorizationRequested,
        "Authorization Issued" => AuthorizationIssued,
        "Collateral Outreach" => CollateralOutreach,
        "Update From Collateral" => UpdateFromCollateral,
        "Invited To Meeting" => InvitedToMeeting,
        "Sent Document" => SentDocument,
        "Updated Document" => UpdatedDocument,
        "Discuss Communication" => DiscussCommunication,
        "Received Verbal Consent" => ReceivedVerbalConsent,
        "Received Written Consent" => ReceivedWrittenConsent,
        "Documentation" => DocumentationStructure,
        "Brainstorm Contribution" => BrainstormContribution,
        _ => return Err(Error::new(
          ErrorKind::Other,
          &format!("Unsupported StructureType saved to file: {}.", &values[1])[..],
        )),
      };

      let content = values[2].clone();

      let user_ids: Vec<u32> = match &values[3][..] {
        "" => vec![],
        _ => values[3]
          .split("#")
          .map(|val| val.parse().unwrap())
          .collect()
      };

      // let note_ids: Vec<u32> = match &values[4][..] {
      //   "" => vec![],
      //   _ => values[4]
      //     .split("#")
      //     .map(|val| val.parse().unwrap())
      //     .collect(),
      // };

      // ^^ possibly use again to store which note came from which template

      let nt = NoteTemplate::new(id, structure, true, content, user_ids);
      note_templates.push(nt);
    }

    for (i, def) in DEFAULT_NOTE_TEMPLATES.iter().enumerate() {
      let i = i as u32;
      let structure = match def.0 {
        "Care Plan" => CarePlan,
        "Intake" => Intake,
        "Assessment" => Assessment,
        "SNCD" => Sncd,
        "Home Visit" => HomeVisit,
        "Agenda Prep" => AgendaPrep,
        "Debrief" => Debrief,
        "Phone Call" => PhoneCall,
        "Scheduling" => Scheduling,
        "Sent Email" => SentEmail,
        "Referral" => Referral,
        "Custom Structure" => CustomStructure,
        "Parent Support" => ParentSupport,
        "Sent Cancellation" => SentCancellation,
        "Parent Appearance" => ParentAppearance,
        "Parent Skills" => ParentSkills,
        "Failed Contact Attempt" => FailedContactAttempt,
        "Categorized Emails" => CategorizedEmails,
        "Authorization Requested" => AuthorizationRequested,
        "Authorization Issued" => AuthorizationIssued,
        "Collateral Outreach" => CollateralOutreach,
        "Update From Collateral" => UpdateFromCollateral,
        "Invited To Meeting" => InvitedToMeeting,
        "Sent Document" => SentDocument,
        "Updated Document" => UpdatedDocument,
        "Discuss Communication" => DiscussCommunication,
        "Received Verbal Consent" => ReceivedVerbalConsent,
        "Received Written Consent" => ReceivedWrittenConsent,
        "Documentation" => DocumentationStructure,
        "Brainstorm Contribution" => BrainstormContribution,
        _ => {
          panic!("Support not added for reading default Structure Type from constant: {}.", def.0);
        }
      };
      let content = String::from(def.1);
      let nt = NoteTemplate::new(i, structure, false, content, vec![]);
      note_templates.push(nt);
    }

    let mut nonduplicates: Vec<NoteTemplate> = vec![];
    for nt in note_templates {
      if !nonduplicates.iter().any(|nondup|
        nondup.structure == nt.structure
        && nondup.content == nt.content
        && !nondup.foreign_keys.get("user_ids").as_deref().iter().any(|u_id_a| !nt.foreign_keys.get("user_ids").as_deref().iter().any(|u_id_b| u_id_b == u_id_a ) )
        && !nt.foreign_keys.get("user_ids").as_deref().iter().any(|u_id_a| !nondup.foreign_keys.get("user_ids").as_deref().iter().any(|u_id_b| u_id_b == u_id_a ) )
      ) {
        nonduplicates.push(nt.clone());
      }
    }

    nonduplicates.sort_by(|a, b| a.id.cmp(&b.id));
    nonduplicates.sort_by(|a, b|
      match (&a.foreign_key.get("user_id"), &b.foreign_key.get("user_id")) {
        (Some(anum), Some(bnum)) => anum.cmp(&bnum),
        _ => b.foreign_key.get("user_id").cmp(&a.foreign_key.get("user_id")),
      }
    );
    nonduplicates.sort_by(|a, b| a.structure.cmp(&b.structure));
    nonduplicates.sort_by(|a, b| a.custom.cmp(&b.custom));
    Ok(nonduplicates)
  }
  pub fn write_note_templates(&self) -> std::io::Result<()> {
    let mut lines = String::from("##### note_templates #####\n");
    for nt in &self.note_templates {
      if nt.custom {
        lines.push_str(&nt.to_string()[..]);
      }
    }
    lines.push_str("##### note_templates #####");
    let mut file = File::create(self.filepaths["note_template_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  fn save_note_template(&mut self, note_template: NoteTemplate) {

    let pos = self.note_templates.binary_search_by(|nt| nt.id.cmp(&note_template.id)
      .then_with(||
        nt.structure.cmp(&note_template.structure))
      .then_with(|| match (&nt.foreign_key.get("user_id"), &note_template.foreign_key.get("user_id")) {
        (Some(anum), Some(bnum)) => anum.cmp(&bnum),
        (Some(_), None) => std::cmp::Ordering::Greater,
        (None, Some(_)) => std::cmp::Ordering::Less,
        _ => note_template.foreign_key["user_id"].cmp(&nt.foreign_key["user_id"]),
      } )
      .then_with(|| nt.id.cmp(&note_template.id))
    ).unwrap_or_else(|e| e);

    self.note_templates.insert(pos, note_template);
    self.write_to_files();
  }
  // fn current_user_custom_note_templates(&self) -> Vec<&NoteTemplate> {
  //   self.note_templates.iter().filter(|nt| nt.custom ).filter(|nt| nt.foreign_key["user_id"] == self.current_user().id).collect()
  // }
  // fn current_user_custom_note_templates_mut(&mut self) -> Vec<&mut NoteTemplate> {
  //   let current_id = self.current_user().id;
  //   self.note_templates.iter_mut().filter(|nt| nt.custom ).filter(|nt| nt.foreign_key["user_id"] == current_id).collect()
  // }
  fn choose_delete_note_template(&mut self) {
    loop {
      self.display_delete_note_template();
      println_yel!("Are you sure you want to delete this note template?");
      println_inst!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
      let mut confirm = String::new();
      let input_attempt = io::stdin().read_line(&mut confirm);
      let command = match input_attempt {
        Ok(_) => confirm.trim().to_string(),
        Err(e) => {
          println_err!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &command[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          self.delete_current_note_template();
          self.reindex_note_templates();
          self.write_to_files();
          break;
        }
        _ => {
          break;
        }
      }
    }
  }
  fn display_delete_note_template(&self) {
    let heading = String::from(" DELETE NOTE TEMPLATE ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^146}", "-");
    println_on_bg!("{:-^146}", heading);
    println_on_bg!("{:-^146}", "-");

    self.current_note_template().display_content(None, None);

    println_on_bg!("{:-^146}", "-");
  }
  fn delete_current_note_template(&mut self) {
    let id = self.foreign_key.get("current_note_template_id").unwrap();
    self.note_templates.retain(|nd| nd.id != *id);
    self.foreign_key.remove("current_note_template_id");
  }
  fn reindex_note_templates(&mut self) {
    let mut i: u32 = 1;
    for mut nt in &mut self.note_templates {
      nt.id = i;
      i += 1;
    }
  }
  fn get_note_template_option_by_id(&self, id: u32) -> Option<&NoteTemplate> {
    self.note_templates.iter().find(|nt| nt.id == id)
  }
  // fn get_note_template_option_by_id_mut(&mut self, id: u32) -> Option<&mut NoteTemplate> {
  //   self.note_templates.iter_mut().find(|nt| nt.id == id)
  // }

// notes

  fn current_note_mut(&mut self) -> &mut Note {
    let n_id = match self.foreign_key.get("current_note_id") {
      Some(id) => id,
      None => panic!("There is no current note selected."),
    };
    let maybe_current: Option<&mut Note> = self.notes.iter_mut().find(|nt| nt.id == *n_id);
    match maybe_current {
      Some(n) => n,
      None => panic!("The loaded ID does not match any saved notes."),
    }
  }
  fn current_note(&self) -> &Note {
    let n_id = match self.foreign_key.get("current_note_id") {
      Some(id) => id,
      None => panic!("There is no current note selected."),
    };
    let maybe_current: Option<&Note> = self.notes.iter().find(|nt| nt.id == *n_id);
    match maybe_current {
      Some(n) => n,
      None => panic!("The loaded ID does not match any saved notes."),
    }
  }
  fn current_note_copy(&self) -> Note {
    let n_id = match self.foreign_key.get("current_note_id") {
      Some(id) => id,
      None => panic!("There is no current note selected."),
    };
    let maybe_current: Option<Note> = self.notes.iter().cloned().find(|n| n.id == *n_id);
    match maybe_current {
      Some(n) => n,
      None => panic!("The loaded ID does not match any saved notes."),
    }
  }
  fn current_note_day_notes(&self) -> Vec<&Note> {
    self.notes.iter().filter(|n| self.current_note_day().foreign_keys["note_ids"].iter().any(|n_id| n_id == &n.id )).collect()
  }
  fn current_note_day_notes_mut(&mut self) -> Vec<&mut Note> {
    let current_note_day = self.current_note_day().clone();
    self.notes.iter_mut().filter(|n| current_note_day.foreign_keys["note_ids"].iter().any(|n_id| n_id == &n.id )).collect()
  }
  fn note_day_notes(&self, nd: NoteDay) -> Vec<&Note> {
    self.notes.iter().filter(|n| nd.foreign_keys["note_ids"].iter().any(|n_id| n_id == &n.id )).collect()
  }
  fn get_note_day_by_note_id(&self, id: u32) -> Option<&NoteDay> {
    self.note_days.iter().find(|nd| nd.foreign_keys["note_ids"].iter().any(|n_id| n_id == &id) )
  }
  // fn get_note_template_by_note_id(&self, id: u32) -> Option<&NoteTemplate> {
  //   self.note_templates.iter().find(|nd| nd.foreign_keys["note_ids"].iter().any(|n_id| n_id == &id) )
  // }
  fn display_note(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^163}", "-");
    
    let n = self.current_note();
    let nd = self.get_note_day_by_note_id(n.id).unwrap();
    let c = self.get_client_by_note_day_id(nd.id).unwrap();
    
    // let nt_string = match self.get_note_template_by_note_id(n.id) {
    //   Some(nt) => nt.display_short(),
    //   None => String::from("n/a"),
    // };

    // let cat_string = match n.category {
    //   ICCNote(ncat) => format!("'{}' by ICC", ncat),
    //   FPNote(ncat) => format!("'{}' by FP", ncat),
    // };

    let heading = format!(" {} {} note for {} ", nd.heading_date(), n.structure, c.full_name());
    println_on_bg!("{:-^163}", heading);
    let heading2 = format!(" ({}) ", n.category);
    println_on_bg!("{:-^163}", heading2);
    println_on_bg!("{:-^163}", "-");
    n.display_content(Some(0), None);
  }
  fn load_note(&mut self, id: u32) -> std::io::Result<()> {
    let current: Option<&Note> = self.notes.iter().find(|n| n.id == id);
    match current {
      Some(n) => {
        self.foreign_key.insert(String::from("current_note_id"), n.id);
        Ok(())
      }
      None => Err(Error::new(
        ErrorKind::Other,
        "Failed to read selected note from filepath.",
      )),
    }
  }
  fn choose_note(&mut self) {
    self.display_note();
    println_inst!("| {} | {} | {}", " EDIT / E: Edit entry ", " DELETE / D: Delete entry ", " QUIT / Q: Quit menu");
    loop {
      let mut choice = String::new();
      let read_attempt = io::stdin().read_line(&mut choice);
      let input = match read_attempt {
        Ok(_) => choice.to_ascii_lowercase(),
        Err(e) => {
          println_err!("Could not read input; try again ({}).", e);
          continue;
        }
      };
      let input = input.trim();
      match input {
        "quit" | "q" => {
          break;
        }
        "delete" | "d" => {
          self.choose_delete_note();
          break;
        }
        "edit" | "e" => {
          self.choose_edit_note();
          break;
        }
        _ => println_err!("Invalid command."),
      }
    }
  }
  fn choose_edit_note(&mut self) {
    'choose_edit: loop {
      let mut blank_focus_id: Option<u32> = None;
      let mut content_focus_id: Option<u32> = None;
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      self.current_note().display_content(blank_focus_id, content_focus_id);
      println_inst!(
        "{} | {}",
        "BLANKS / B: Edit blanks",
        "CONTENT / C: Edit other content",
      );
      println_inst!(
        "{} | {}",
        "TYPE / T: Edit note type/category",
        "QUIT / Q: Quit menu",
      );
      println_inst!("Choose blank by ID to edit its content.");
      println_inst!("Changes are saved automatically.");
      let mut field_to_edit = String::new();
      let input_attempt = io::stdin().read_line(&mut field_to_edit);
      match input_attempt {
        Ok(_) => (),
        Err(_) => {
          println_err!("Failed to read input. Please try again.");
          continue;
        }
      }
      field_to_edit = field_to_edit.trim().to_string();
      match &field_to_edit.to_ascii_lowercase()[..] {
        "quit" | "q" => break,
        "type" | "t" | "category" | "cat" => {
          let structure = loop {
            self.display_structure_types();
            print_inst!("Choose a new structure for the selected note from the menu.");
            print!("\n");
            print_inst!("Enter 'CANCEL' at any time to cancel.");
            print!("\n");
            let mut structure_choice = String::new();
            let structure_attempt = io::stdin().read_line(&mut structure_choice);
            match structure_attempt {
              Ok(_) => (),
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
            let structure = structure_choice.trim();
            let chosen_id: usize = match structure.parse() {
              Ok(num) => num,
              Err(e) => {
                println_err!("Failed to parse '{}' as int: {}", structure, e);
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
            };
            break match StructureType::iterator().nth(chosen_id - 1) {
              None => {
                println_err!("Invalid choice.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              },
              Some(s) => s.clone(),
            };
          };
          let ncat = match self.current_user().role {
            Icc => {
              match structure {
                CarePlan => ICCNote(FaceToFaceContactWithClient),
                Intake => ICCNote(FaceToFaceContactWithClient),
                Assessment => ICCNote(FaceToFaceContactWithClient),
                Sncd => ICCNote(FaceToFaceContactWithClient),
                HomeVisit => ICCNote(FaceToFaceContactWithClient),
                AgendaPrep => ICCNote(FaceToFaceContactWithClient),
                Debrief => ICCNote(FaceToFaceContactWithClient),
                PhoneCall => ICCNote(TelephoneContactWithClient),
                Scheduling => ICCNote(CareCoordination),
                SentEmail => ICCNote(CareCoordination),
                Referral => ICCNote(CareCoordination),
                ParentSupport => ICCNote(FaceToFaceContactWithClient),
                SentCancellation => ICCNote(CareCoordination),
                ParentAppearance => ICCNote(FaceToFaceContactWithClient),
                ParentSkills => ICCNote(FaceToFaceContactWithClient),
                FailedContactAttempt => ICCNote(MemberOutreachNoShow),
                CategorizedEmails => ICCNote(Documentation),
                AuthorizationRequested => ICCNote(CareCoordination),
                AuthorizationIssued => ICCNote(CareCoordination),
                CollateralOutreach => ICCNote(CareCoordination),
                UpdateFromCollateral => ICCNote(CareCoordination),
                InvitedToMeeting => ICCNote(CareCoordination),
                SentDocument => ICCNote(CareCoordination),
                UpdatedDocument => ICCNote(Documentation),
                DiscussCommunication => ICCNote(CareCoordination),
                ReceivedVerbalConsent => ICCNote(FaceToFaceContactWithClient),
                ReceivedWrittenConsent => ICCNote(FaceToFaceContactWithClient),
                DocumentationStructure => ICCNote(Documentation),
                BrainstormContribution => ICCNote(FaceToFaceContactWithClient),
                CustomStructure => {
                  loop {
                    match self.choose_note_category() {
                      Some(ncat) => break ncat,
                      None => {
                        let decision = loop {
                          let mut cancel_choice = String::new();
                          println_yel!("You must select a note category to fill in a custom note template.");
                          println_inst!("Cancel writing note? ( Y / N )");
                          let cancel_choice_attempt = io::stdin().read_line(&mut cancel_choice);
                          let cancel_choice_content = match cancel_choice_attempt {
                            Ok(_) => cancel_choice.trim().to_ascii_lowercase(),
                            Err(e) => {
                              println_err!("Failed to read line: {}", e);
                              thread::sleep(time::Duration::from_secs(2));
                              continue;
                            }
                          };
                          break cancel_choice_content;
                        };
                        match &decision[..] {
                          "yes" | "y" => {
                            continue 'choose_edit;
                          },
                          "no" | "n" => {
                            continue;
                          },
                          _ => {
                            println_err!("Invalid command.");
                            continue;
                          },
                        }
                      }
                    }

                  }
                }
              }
            }
            Fp => {
              match structure {
              CarePlan => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
              Intake => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
              Assessment => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
              Sncd => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
              HomeVisit => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
              AgendaPrep => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
              Debrief => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
              PhoneCall => FPNote(DescriptionOfIntervention(Some(TelephoneSupport))),
              Scheduling => FPNote(PlanAdditionalInformation),
              SentEmail => FPNote(PlanAdditionalInformation),
              Referral => FPNote(DescriptionOfIntervention(Some(CollateralContact))),
              ParentSupport => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
              SentCancellation => FPNote(DescriptionOfIntervention(Some(ProviderOutreachToPerson))),
              ParentAppearance => FPNote(Functioning),
              ParentSkills => FPNote(Functioning),
              FailedContactAttempt => FPNote(DescriptionOfIntervention(Some(ProviderOutreachToPerson))),
              CategorizedEmails => FPNote(DescriptionOfIntervention(Some(FPInterventionDocumentation))),
              AuthorizationRequested => FPNote(PlanAdditionalInformation),
              AuthorizationIssued => FPNote(PlanAdditionalInformation),
              CollateralOutreach => FPNote(DescriptionOfIntervention(Some(CollateralContact))),
              UpdateFromCollateral => FPNote(DescriptionOfIntervention(Some(CollateralContact))),
              InvitedToMeeting => FPNote(PlanAdditionalInformation),
              SentDocument => FPNote(PlanAdditionalInformation),
              UpdatedDocument => FPNote(DescriptionOfIntervention(Some(FPInterventionDocumentation))),
              DiscussCommunication => FPNote(DescriptionOfIntervention(Some(DirectTimeWithProviders))),
              ReceivedVerbalConsent => FPNote(PlanAdditionalInformation),
              ReceivedWrittenConsent => FPNote(PlanAdditionalInformation),
              DocumentationStructure=> FPNote(DescriptionOfIntervention(Some(FPInterventionDocumentation))),
              BrainstormContribution => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
                CustomStructure => {
                  loop {
                    match self.choose_note_category() {
                      Some(ncat) => break ncat,
                      None => {
                        let decision = loop {
                          let mut cancel_choice = String::new();
                          println_yel!("You must select a note category to fill in a custom note template.");
                          println_inst!("Cancel writing note? ( Y / N )");
                          let cancel_choice_attempt = io::stdin().read_line(&mut cancel_choice);
                          let cancel_choice_content = match cancel_choice_attempt {
                            Ok(_) => cancel_choice.trim().to_ascii_lowercase(),
                            Err(e) => {
                              println_err!("Failed to read line: {}", e);
                              thread::sleep(time::Duration::from_secs(2));
                              continue;
                            }
                          };
                          break cancel_choice_content;
                        };
                        match &decision[..] {
                          "yes" | "y" => {
                            continue 'choose_edit;
                          },
                          "no" | "n" => {
                            continue;
                          },
                          _ => {
                            println_err!("Invalid command.");
                            continue;
                          },
                        }
                      }
                    }

                  }
                },
              }
            }
          };
          self.current_note_mut().structure = structure;
          self.current_note_mut().category = ncat;
        },
        "content" | "c" => {
          content_focus_id = Some(1);
          blank_focus_id = None;
          loop {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            self.current_note().display_content(blank_focus_id, content_focus_id);
            println_inst!(
              "{} | {}",
              "EDIT / E: Edit selected content",
              "ADD / A: Add content to end of note",
            );
            println_inst!(
              "{} | {}",
              "INSERT / I: Insert additional content",
              "DELETE / D: Delete content",
            );
            println_inst!("Choose content section by ID.");
            println_inst!("Enter 'QUIT / Q' at any time to return to the editing menu.");
            let mut content_choice = String::new();
            let content_attempt = io::stdin().read_line(&mut content_choice);
            match content_attempt {
              Ok(_) => (),
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
            let content = content_choice.trim();
            match &content.to_ascii_lowercase()[..] {
              "quit" | "q" => {
                break;
              },
              "append" | "add" | "a" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                self.current_note().display_content(None, None);
                println_inst!("Enter new content to append to the note, including spaces.");
                let mut new_section = String::new();
                let new_section_choice = loop {
                  let section_result = io::stdin().read_line(&mut new_section);
                  break match section_result {
                    Ok(_) => String::from(&new_section[..new_section.len()-2]),
                    Err(e) => {
                      println_err!("Failed to read line: {}", e);
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  };
                };

                let c_content = self.current_note().content.clone();
                let last_char = c_content.chars().last();
                let new_content = match last_char {
                  None => format!("{}{}", c_content, new_section_choice),
                  Some(c) => {
                    if String::from("?!.").contains(c) && new_section_choice.chars().next() != Some(' ') {
                      format!("{}{}{}", c_content, " ", new_section_choice)
                    } else {
                      format!("{}{}", c_content, new_section_choice)
                    }
                  }
                };
                self.current_note_mut().content = new_content;
                self.current_note_mut().clean_spacing();
                self.write_to_files()
              },
              "edit" | "e" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                self.current_note().display_content(blank_focus_id, content_focus_id);
                println_inst!("Enter exact content to replace the selected text, including any spaces on the left and right.");
                let mut new_section = String::new();
                let new_section_choice = loop {
                  let section_result = io::stdin().read_line(&mut new_section);
                  break match section_result {
                    Ok(_) => String::from(&new_section[..new_section.len()-2]),
                    Err(e) => {
                      println_err!("Failed to read line: {}", e);
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  };
                };

                let nt_content = self.current_note().content.clone();

                let mut new_content = String::new();

                for (i, idxs) in self.current_note().get_content_section_indices().iter().enumerate() {
                  if i + 1 == content_focus_id.unwrap() as usize {
                    new_content = format!("{}{}{}", &nt_content[..idxs.0], &new_section_choice[..], &nt_content[idxs.1..]);
                  }
                }

                self.current_note_mut().content = new_content;
                self.current_note_mut().clean_spacing();
                self.write_to_files()
              },
              "delete" | "d" => {
                loop {
                  println_yel!("Are you sure you want to delete the currently selected section of content? (Y/N)");
                  let mut delete_choice = String::new();
                  let delete_attempt = io::stdin().read_line(&mut delete_choice);
                  match delete_attempt {
                    Ok(_) => (),
                    Err(e) => {
                      println_err!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                  let delete = delete_choice.trim();
                  match &delete[..] {
                    "YES" | "yes" | "Yes" | "Y" | "y" => {
                      let nt_content = self.current_note().content.clone();

                      let mut new_content = String::new();
                      for (i, idxs) in self.current_note().get_content_section_indices().iter().enumerate() {
                        if i == content_focus_id.unwrap() as usize {
                          new_content = format!("{}{}", &nt_content[..idxs.0], &nt_content[idxs.1..]);
                        }
                      }
                      self.current_note_mut().content = new_content;
                      self.current_note_mut().clean_spacing();
                      self.write_to_files();
                      break;
                    },
                    "NO" | "no" | "No" | "N" | "n" => {
                      break;
                    },
                    _ => {
                      println_err!("Please enter either 'YES'/'Y' or 'NO'/'N'.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              },
              "insert" | "i" => { // in content choice
                let mut chosen_text = String::new();
                content_focus_id = Some(1);
                blank_focus_id = None;
                'insert_content: loop {
                  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                  self.current_note().display_content(blank_focus_id, content_focus_id);
                  
                  if chosen_text == String::new() {
                    println_inst!("Enter new content to insert, without spaces on the ends.");
                    let mut selected_text = String::new();
                    let enter_result = io::stdin().read_line(&mut selected_text);
                    chosen_text = match enter_result {
                      Ok(_) => String::from(&selected_text[..selected_text.len()-2]),
                      Err(e) => {
                        println_err!("Failed to read string: {}", e);
                        thread::sleep(time::Duration::from_secs(2));
                        continue;
                      }
                    };
                  }

                  let content_indices = self.current_note().get_content_section_indices();
                  println_inst!("Select a section of text in the note by ID, then ENTER to insert new content before, after, or in that section.");
                  println_inst!("Enter CANCEL at any time to cancel.");

                  let insert = loop {
                    let mut insert_choice = String::new();
                    let insert_attempt = io::stdin().read_line(&mut insert_choice);
                    match insert_attempt {
                      Ok(_) => (),
                      Err(e) => {
                        println_err!("Invalid input: {}", e);
                        thread::sleep(time::Duration::from_secs(2));
                        continue 'insert_content;
                      }
                    }
                    let insert = insert_choice.trim();
                    if content_focus_id.unwrap() == 0 {
                      if &insert[..] == "" {
                        continue;
                      }
                    }
                    break insert.to_string();
                  };
                  match &insert[..] {
                    "CANCEL" | "cancel" | "C" | "c" => {
                      break 'insert_content;
                    },
                    "" => {
                      let mut current_location = 1;
                      'insert_location_content: loop {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        let indices = content_indices[content_focus_id.unwrap() as usize - 1];
                        let idx1 = indices.0 as usize;
                        let idx2 = indices.1 as usize;
                        let display_string = self.current_note().content[idx1..idx2].to_string();
                        let num_chars = display_string.chars().count();
                        if num_chars <= 163 {
                          println_on_bg!("{}", &display_string);
                          let mut pointer_string = String::new();
                          for _ in 0..current_location {
                            pointer_string.push_str("-");
                          }
                          pointer_string.push_str("^");
                          println_on_bg!("{}", &pointer_string);
                        } else {
                          let mut outer_vec: Vec<(String, String)> = vec![];

                          let mut display_content = self.current_note().generate_display_content_string_with_blanks(None, None, None, None, None).0.clone();
                          while display_content.chars().count() > 163 {
                            let (p1, p2) = display_content.split_at(163);
                            let mut pointer_string = String::new();
                            for _ in 0..163 {
                              pointer_string.push_str(" ");
                            }
                            outer_vec.push((format!("{}", p1), pointer_string));
                            display_content = format!("{}", p2);
                          }
                          if display_content == String::new() {
                            ()
                          } else {
                            let mut pointer_string = String::new();
                            for _ in 1..pointer_string.chars().count() {
                              pointer_string.push_str("-");
                            }
                            pointer_string.push_str("^");
                            outer_vec.push((format!("{}", display_content), pointer_string));
                          }
                          for row_and_pointer in outer_vec {
                            println_on_bg!("{}", row_and_pointer.0);
                            println_on_bg!("{}", row_and_pointer.1);
                          }
                        }
                        println_inst!("New content: {}", &chosen_text);
                        println_inst!("Enter an integer to navigate to a point at which to insert the new content.");
                        println_inst!("Press ENTER to insert new content in the current location.");
                        println_inst!(
                          "{} | {} | {}",
                          "BEFORE / B: Insert at start of section",
                          "AFTER / A: Insert at end of section",
                          "CANCEL / C: Cancel inserting new content",
                        );

                        let mut insert_string = String::new();
                        let insert_attempt = io::stdin().read_line(&mut insert_string);
                        match insert_attempt {
                          Ok(_) => (),
                          Err(e) => {
                            println_err!("Invalid input: {}", e);
                            thread::sleep(time::Duration::from_secs(2));
                            continue;
                          },
                        }
                        match &insert_string.trim().to_ascii_lowercase()[..] {
                          // integer to change current location
                          // ENTER to insert it there, then show what it will look like and confirm
                          "cancel" => {
                            break 'insert_location_content;
                          }
                          "before" | "b" => {
                            loop {
                              let last_content_char = self.current_note()
                                .generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1+1]
                                .chars()
                                .last();
                              let next_content_char = self.current_note()
                                .generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..]
                                .chars()
                                .next();
                              let bfrs = get_spacing_buffers(last_content_char, next_content_char, chosen_text.clone());

                              let new_content_section = format!("{}{}{}{}", &bfrs.0, &chosen_text, &bfrs.1, &display_string[..]);
                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              println_inst!("{}", &new_content_section);

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              match &confirm.to_ascii_lowercase()[..] {
                                "yes" | "y" => {
                                  let nt_content = self.current_note().content.clone();

                                  let mut new_content = String::new();

                                  for (i, idxs) in self.current_note().get_content_section_indices().iter().enumerate() {
                                    if i + 1 == content_focus_id.unwrap() as usize {
                                      new_content = format!("{}{}{}", &nt_content[..idxs.0], &new_content_section[..], &nt_content[idxs.1..]);
                                    }
                                  }
                                  self.current_note_mut().content = new_content;
                                  self.current_note_mut().clean_spacing();
                                  self.write_to_files();
                                  break 'insert_content;
                                },
                                "no" | "n" => {
                                  continue 'insert_location_content;
                                },
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          "after" | "a" => {
                            loop {
                              let last_content_char = display_string[..]
                                .chars()
                                .last();
                              let next_content_char = self.current_note()
                                .generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..]
                                .chars()
                                .next();
                              let bfrs = get_spacing_buffers(last_content_char.clone(), next_content_char, chosen_text.clone());

                              let last_char = match last_content_char {
                                Some(c) => c.to_string(),
                                None => String::new(),
                              };

                              let new_content_section = format!("{}{}{}{}", &display_string[..], &bfrs.0, &chosen_text, &bfrs.1);
                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              println_inst!("{}{}", &last_char, &new_content_section);

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              match &confirm.to_ascii_lowercase()[..] {
                                "yes" | "y" => {
                                  let nt_content = self.current_note().content.clone();

                                  let mut new_content = String::new();

                                  for (i, idxs) in self.current_note().get_content_section_indices().iter().enumerate() {
                                    if i + 1 == content_focus_id.unwrap() as usize {
                                      new_content = format!("{}{}{}", &nt_content[..idxs.0], &new_content_section[..], &nt_content[idxs.1..]);
                                    }
                                  }
                                  self.current_note_mut().content = new_content;
                                  self.current_note_mut().clean_spacing();
                                  self.write_to_files();
                                  break 'insert_content;
                                },
                                "no" | "n" => {
                                  continue 'insert_location_content;
                                },
                                "cancel" | "c" => {
                                  break 'insert_content;
                                },
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          "" => {
                            loop {
                              let last_content_char = if current_location == 0 {
                                self.current_note().generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1].chars().last()
                              } else {
                                display_string[..current_location].chars().last()
                              };
                              let next_content_char = if current_location >= &display_string.chars().count()-1 {
                                self.current_note().generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..].chars().next()
                              } else {
                                display_string[current_location..].chars().next()
                              };
                              let bfrs = get_spacing_buffers(last_content_char, next_content_char, chosen_text.clone());

                              let new_content = format!(
                                "{}{}{}{}{}",
                                &display_string[..current_location],
                                &bfrs.0,
                                &chosen_text,
                                &bfrs.1,
                                &display_string[current_location..]
                              );
                              println_inst!("Confirm editing the selection to the following? (Y/N)");
                              print_on_bg!("{}{}",
                                &display_string[..current_location],
                                &bfrs.0,
                              );
                              print_highlighted_content!("{}",
                                &chosen_text,
                              );
                              print_on_bg!("{}{}\n",
                                &bfrs.1,
                                &display_string[current_location..]
                              );

                              let mut confirm_insert = String::new();
                              let confirm_attempt = io::stdin().read_line(&mut confirm_insert);
                              let confirm = match confirm_attempt {
                                Ok(_) => confirm_insert.trim(),
                                Err(e) => {
                                  println_err!("Failed to read input: {}.", e);
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              };

                              let current_content = self.current_note().content.clone();

                              match &confirm[..] {
                                "YES" | "yes" | "Yes" | "Y" | "y" => {
                                  self.current_note_mut().content = format!(
                                    "{}{}{}",
                                    &current_content[..idx1],
                                    &new_content,
                                    &current_content[idx2..],
                                  );
                                  self.current_note_mut().clean_spacing();
                                  self.write_to_files();
                                  break 'insert_content;
                                },
                                "NO" | "no" | "No" | "N" | "n" => {
                                  continue 'insert_location_content;
                                },
                                _ => {
                                  println_err!("Invalid command.");
                                  thread::sleep(time::Duration::from_secs(2));
                                  continue;
                                }
                              }
                            }
                          },
                          _ => {
                            match insert_string.trim().parse() {
                              Ok(num) => {
                                current_location = num;
                                continue;
                              },
                              Err(_) => {
                                println_err!("Invalid command.");
                                thread::sleep(time::Duration::from_secs(2));
                                continue;
                              }
                            }
                          }
                        }
                      }
                    },
                    _ => {
                      match insert.parse::<u32>() {
                        Err(_) => {
                          println_err!("Invalid command.");
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        },
                        Ok(num) => {
                          if num > 0 && (num as usize) < content_indices.len() {
                            content_focus_id = Some(num);
                          } else {
                            println_err!("Please enter an ID in the range shown.");
                          }
                        },
                      }
                    }
                  }
                }
                self.write_to_files();
              },
              _ => {
                match content.parse::<u32>() {
                  Err(_) => {
                    let content_indices = self.current_note().get_content_section_indices();
                    let (idx1, idx2) = content_indices[(content_focus_id.unwrap() - 1) as usize];
                    let last_content_char = self.current_note()
                      .generate_display_content_string_with_blanks(None, None, None, None, None).0[..idx1+1]
                      .chars()
                      .last();
                    let next_content_char = self.current_note()
                      .generate_display_content_string_with_blanks(None, None, None, None, None).0[idx2..]
                      .chars()
                      .next();
                    let bfrs = get_spacing_buffers(last_content_char, next_content_char, content.to_string());

                    let new_content_section = format!("{}{}{}", &bfrs.0, &content, &bfrs.1);
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    self.current_note().display_content(blank_focus_id, content_focus_id);
                    println_inst!("Press ENTER to confirm replacing selected content with '{}'.", &new_content_section);
                    println_inst!("Any other key to cancel.");
                    let mut new_section = String::new();
                    let new_section_choice = loop {
                      let section_result = io::stdin().read_line(&mut new_section);
                      break match section_result {
                        Ok(_) => new_section.trim().to_ascii_lowercase(),
                        Err(e) => {
                          println_err!("Failed to read line: {}", e);
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        }
                      };
                    };

                    match &new_section_choice[..] {
                      "" => {
                        let n_content = self.current_note().content.clone();
    
                        let mut new_content = String::new();
    
                        for (i, idxs) in self.current_note().get_content_section_indices().iter().enumerate() {
                          if i + 1 == content_focus_id.unwrap() as usize {
                            new_content = format!("{}{}{}", &n_content[..idxs.0], &new_content_section[..], &n_content[idxs.1..]);
                          }
                        }
    
                        self.current_note_mut().content = new_content;
                        self.current_note_mut().clean_spacing();
                        self.write_to_files()
                      },
                      _ => {
                        continue;
                      }
                    }
                  },
                  Ok(num) => {
                    let num_sections = self.current_note().get_num_content_sections();
                    if num as usize > num_sections || num == 0 {
                      println_err!("Please choose from among the content IDs shown above.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    } else {
                      content_focus_id = Some(num);
                    }
                  },
                }
              },
            }
          }
        },
        _ => { // THIS IS "blanks" | "b" option - because parsing to int will also lead to choosing blank condition
          lazy_static! {
            static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
          }
          let num_blanks = RE_BLANK.find_iter(&self.current_note().content).count() as u32;
          if num_blanks == 0 {
            println_err!("There are no blanks to edit.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          blank_focus_id = Some(1);
          content_focus_id = None;
          if field_to_edit.clone() == String::from("blanks") || field_to_edit.clone() == String::from("b") {
            ()
          } else {
            match field_to_edit.parse::<u32>() {
              Ok(num) => {
                if num > num_blanks || num == 0 {
                  blank_focus_id = Some(num);
                  content_focus_id = None;
                } else {
                  println_err!("Please choose from among the blank IDs shown above.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              },
              Err(_) => {
                println_err!("Invalid command.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
            }
          }
          loop {
            self.current_note_mut().blanks = self.autofill_note_blanks(self.current_note_copy()).blanks.clone();
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            self.current_note().display_content(blank_focus_id, content_focus_id);
            println_inst!(
              "{} | {}",
              "DELETE / D: Delete selected blank",
              "QUIT / Q: Return to editing menu",
            );
            println_inst!("Choose blank by blank ID.");
            println_inst!("Press ENTER to edit content of the selected blank.");
            let mut blank_choice = String::new();
            let blank_attempt = io::stdin().read_line(&mut blank_choice);
            match blank_attempt {
              Ok(_) => (),
              Err(e) => {
                println_err!("Invalid repsonse: {}", e);
                continue;
              }
            }
            let blank = blank_choice.trim();
            match &blank.to_ascii_lowercase()[..] {
              "quit" | "q" => {
                break;
              },
              "" | "edit" | "e" => {
                let i = blank_focus_id.unwrap();
                let b = match self.current_note().blanks.clone().get(&i) {
                  None => {
                    lazy_static! {
                      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                    }

                    let mut b = CustomBlank;

                    for (idx, m) in RE_BLANK.find_iter(&self.current_note().content).enumerate() {
                      if idx + 1 == i as usize {
                        b = Blank::get_blank_from_str(&self.current_note().content[m.start()..m.end()]);
                      }
                    }

                    b
                  },
                  Some(b_tup) => b_tup.0,
                };
                match b {
                  Collaterals => {
                    let mut collats: Vec<Collateral> = vec![];
                    let mut general_collats: Vec<Collateral> = vec![];
                    loop {
                      let initial_input = loop {
                        let collat_ids = collats.iter().map(|co| co.id ).collect::<Vec<u32>>();
                        self.display_client_collaterals(Some(collat_ids));
                        println_inst!("ALL: Select all");
                        let mut choice = String::new();
                        let read_attempt = io::stdin().read_line(&mut choice);
                        match read_attempt {
                          Ok(_) => break choice.trim().to_string(),
                          Err(e) => {
                            println_err!("Could not read input; try again ({}).", e);
                            continue;
                          }
                        }
                      };
                      match &initial_input.to_ascii_lowercase()[..] {
                        "new" | "n" => {
                          let maybe_new_id = self.create_collateral_get_id();
                          match maybe_new_id {
                            _ => (),
                          }
                          continue;
                        },
                        "add" | "a" => {
                          self.add_collateral();
                          continue;
                        },
                        "general" | "g" => {
                          loop {
                            let general_collat_ids = general_collats.clone().iter().map(|co| co.id ).collect::<Vec<u32>>();
                            let general_input = loop {
                              self.display_select_general_collaterals(Some(general_collat_ids.clone()));
                              println_inst!("ALL: Select all");
                              let mut choice = String::new();
                              let read_attempt = io::stdin().read_line(&mut choice);
                              match read_attempt {
                                Ok(_) => break choice.trim().to_string(),
                                Err(e) => {
                                  println_err!("Could not read input; try again ({}).", e);
                                  continue;
                                }
                              }
                            };
                            match &general_input.to_ascii_lowercase()[..] {
                              "new" | "n" => {
                                let maybe_new_id = self.create_general_collateral_get_id();
                                match maybe_new_id {
                                  Some(_) => (),
                                  None => (),
                                }
                                continue;
                              },
                              "edit" | "e" => {
                                self.choose_edit_general_collaterals();
                                continue;
                              },
                              "all" | "a" => {
                                general_collats = self.general_collaterals.clone();
                                break;
                              },
                              "quit" | "q" => {
                                break;
                              },
                              "" => {
                                break;
                              }
                              _ => match general_input.parse() {
                                Ok(num) => {
                                  let collat = match self.general_collaterals.iter().find(|co| co.id == num) {
                                    Some(co) => co.clone(),
                                    None => continue,
                                  };
                                  if !general_collats.iter().any(|co| co == &collat ) {
                                    general_collats.push(collat);
                                    if collats.len() == self.current_client_collaterals().len() {
                                      break;
                                    }
                                  } else {
                                    general_collats.retain(|co| co != &collat );
                                  }
                                },
                                Err(e) => {
                                  println_err!("Invalid input: {}; error: {}", initial_input, e);
                                  thread::sleep(time::Duration::from_secs(3));
                                  continue;
                                }
                              }
                            }
                          }
                          for collat in general_collats.clone() {
                            collats.push(collat.clone());
                          }
                          continue;
                        }
                        "edit" | "e" => {
                          self.choose_edit_client_collaterals();
                          continue;
                        },
                        "quit" | "q" => {
                          break;
                        },
                        "all" => {
                          let mut old_collats = collats.clone();
                          for collat in self.get_current_collaterals().iter().cloned().cloned().collect::<Vec<Collateral>>() {
                            if !old_collats.clone().iter().any(|co| co.id == collat.id ) {
                              old_collats.push(collat.clone());
                            }
                          }
                          break;
                        },
                        "" => {
                          if collats.len() > 0 {
                            let new_note = self.autofill_note_blanks(self.current_note_copy());
                            self.current_note_mut().blanks = new_note.blanks.clone();
                            break;
                          } else {
                            println_err!("Please choose at least one collateral to add to the current blank.");
                            thread::sleep(time::Duration::from_secs(2));
                            continue;
                          }
                        },
                        _ => {
                          let selected_id_res: Result<u32, _> = initial_input.parse();
                          match selected_id_res {
                            Ok(num) => {
                              if !self.current_client_collaterals().iter().any(|co| co.id == num ) {
                                println_err!("Please choose from among the options shown.");
                                thread::sleep(time::Duration::from_secs(2));
                                continue;
                              } else {
                                let collat = match self.get_collateral_by_id(num as u32) {
                                  Some(co) => co.clone(),
                                  None => continue,
                                };
                                if !collats.iter().any(|co| co == &collat ) {
                                  collats.push(collat);
                                  if collats.len() == self.current_client_collaterals().len() {
                                    break;
                                  }
                                } else {
                                  collats.retain(|co| co != &collat );
                                }
                              }
                            },
                            Err(e) => {
                              println_err!("Invalid input: {}; error: {}", initial_input, e);
                              thread::sleep(time::Duration::from_secs(3));
                              continue;
                            }
                          }
                        }
                      }
                    }
                    let blank_string = if collats.len() > 1 {
                      format!(
                        "{} {} {}",
                        collats[..collats.len()-1].iter().map(|co| co.full_name_and_title()).collect::<Vec<String>>().join(", "),
                        "and",
                        collats[collats.len()-1].full_name_and_title()
                      )
                    } else if collats.len() > 0 {
                      collats[0].full_name_and_title()
                    } else {
                      String::new()
                    };
                    let id_vec: Vec<u32> = collats.iter().map(|co| co.id ).collect();
                    self.current_note_mut().blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
                    let mut new_ids = self.current_note().foreign_keys["collateral_ids"].clone();
                    for co_id in id_vec {
                      if !new_ids.clone().iter().any(|old_id| old_id == &co_id ) {
                        new_ids.push(co_id);
                      }
                    }
                    self.current_note_mut().foreign_keys.insert(String::from("collateral_ids"), new_ids);
                  },
                  ClientGoal => {
                    let mut fill_ins: Vec<u32> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match self.select_client_goals(Some(fill_ins.clone())) {
                        Some(s) => s,
                        None => {
                          let fill_in_strings = fill_ins.iter().map(|g_id| self.get_goal_by_id(*g_id).unwrap().goal.clone() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, fill_ins));
                    }
                  },
                  InternalDocument => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(InternalDocument, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| InternalDocumentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  ExternalDocument => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(ExternalDocument, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| ExternalDocumentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  InternalMeeting => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(InternalMeeting, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| InternalMeetingFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  ExternalMeeting => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(ExternalMeeting, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| ExternalMeetingFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  Appearance => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(Appearance, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| AppearanceFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  SupportedParent => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(SupportedParent, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| SupportedParentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  ParentingSkill => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(ParentingSkill, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| ParentingSkillFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  CarePlanningTopic => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(CarePlanningTopic, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| CarePlanningTopicFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  YouthTopic => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(YouthTopic, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| YouthTopicFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  ContactMethod => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(ContactMethod, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| ContactMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  ContactPurpose => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(ContactPurpose, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| ContactPurposeFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  FulfilledContactPurpose => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(FulfilledContactPurpose, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| FulfilledContactPurposeFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  Service => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(Service, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| ServiceFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  MeetingMethod => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(MeetingMethod, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| MeetingMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  SignatureMethod => {
                    let mut fill_ins: Vec<usize> = vec![];
                    let final_blank_string = loop {
                      let blank_id = match Self::select_blank_fill_in(SignatureMethod, fill_ins.clone()) {
                        Some(s) => s,
                        None => {
                          let fill_in_objects = fill_ins.iter().map(|fi| SignatureMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                          let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                          break if fill_in_strings.len() > 1 {
                            format!(
                              "{}{}{}",
                              fill_in_strings[..fill_in_strings.len()-1].join(", "),
                              " and ",
                              fill_in_strings[fill_in_strings.len()-1],
                            )
                          } else if fill_in_strings.len() > 0 {
                            fill_in_strings[0].clone()
                          } else {
                            String::new()
                          };
                        }
                      };
                      if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                        fill_ins.push(blank_id);
                      } else {
                        fill_ins.retain(|fi| fi != &blank_id )
                      }
                    };
                    if fill_ins.len() > 0 {
                      self.current_note_mut().blanks.insert(i, (b.clone(), final_blank_string, vec![]));
                    }
                  },
                  CustomBlank => {
                    loop {
                      let mut custom_choice = String::new();
                      println_inst!("Enter new custom content for the current blank.");
                      let custom_attempt = io::stdin().read_line(&mut custom_choice);
                      let custom_content = match custom_attempt {
                        Ok(_) => custom_choice.trim(),
                        Err(e) => {
                          println_err!("Failed to read line: {}", e);
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        }
                      };
                      let blank_string = custom_content.to_string();
                      let id_vec = vec![];
                      self.current_note_mut().blanks.insert(i, (b.clone(), blank_string, id_vec));
                      break;
                    }
                  },
                  _ => {
                    println_err!("The selected blank type cannot be edited: {}.", b);
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  },
                }
                self.write_to_files();
              },
              "delete" | "d" => {
                loop {
                  println_yel!("Are you sure you want to delete the currently selected blank? (Y/N)");
                  let mut delete_choice = String::new();
                  let delete_attempt = io::stdin().read_line(&mut delete_choice);
                  match delete_attempt {
                    Ok(_) => (),
                    Err(e) => {
                      println_err!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                  let delete = delete_choice.trim().to_ascii_lowercase();
                  match &delete[..] {
                    "yes" | "y" => {
                      let n_content = self.current_note().content.clone();

                      lazy_static! {
                        static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                      }

                      let mut new_content = String::new();
                      for (i, m) in RE_BLANK.find_iter(&n_content).enumerate() {
                        if i + 1 == blank_focus_id.unwrap() as usize {
                          new_content = format!("{}{}", &n_content[..m.start()], &n_content[m.end()..]);
                          self.current_note_mut().blanks.remove(&((i+1) as u32));
                        }
                      }
                      self.current_note_mut().content = new_content;
                      self.current_note_mut().clean_spacing();
                      self.write_to_files();
                      break;
                    },
                    "no" | "n" => {
                      break;
                    },
                    _ => {
                      println_err!("Please enter either 'YES'/'Y' or 'NO'/'N'.");
                      thread::sleep(time::Duration::from_secs(2));
                      continue;
                    }
                  }
                }
              },
              _ => {
                let n_content = self.current_note().content.clone();
                lazy_static! {
                  static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                }

                let num_matches = RE_BLANK.find_iter(&n_content).count();

                let u32_num = blank.parse::<u32>();

                match blank.parse::<usize>() {
                  Ok(num) => if num <= num_matches {
                    blank_focus_id = Some(u32_num.unwrap());
                    continue;
                  } else {
                    println_err!("Please choose a blank by ID among those displayed in the note's content.");
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  },
                  Err(_) => {
                    println_err!("Invalid command.");
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  }
                }                  
              }
            }
          }
        },
      }
    }
  }
  fn display_blank_fill_in(blank_type: Blank, selected: Option<Vec<usize>>) {
    let display_category = format!("{}", blank_type.clone().display_to_user_empty());
    let fill_ins: Vec<String> = match blank_type {
      InternalDocument => {
        InternalDocumentFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      ExternalDocument => {
        ExternalDocumentFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      InternalMeeting => {
        InternalMeetingFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      ExternalMeeting => {
        ExternalMeetingFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      Appearance => {
        AppearanceFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      SupportedParent => {
        SupportedParentFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      ParentingSkill => {
        ParentingSkillFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      CarePlanningTopic => {
        CarePlanningTopicFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      YouthTopic => {
        YouthTopicFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      ContactMethod => {
        ContactMethodFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      ContactPurpose => {
        ContactPurposeFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      FulfilledContactPurpose => {
        FulfilledContactPurposeFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      Service => {
        ServiceFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      MeetingMethod => {
        MeetingMethodFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      SignatureMethod => {
        SignatureMethodFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      _ => panic!("Incompatible blank fill in string passed to fn 'display_blank_fill_in'")
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^113}", format!(" Fill in blank with {} ", display_category));
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^10} | {:-^100}", " ID ", " Content ");
    for (i, fi) in fill_ins.iter().enumerate() {
      match selected.clone() {
        Some(ids) => {
          if ids.iter().any(|id| id == &i ) {
            println_suc!("{: ^10} | {: <100}", i, fi);
          } else {
            println_on_bg!("{: ^10} | {: <100}", i, fi);
          }
        },
        None => {
          println_on_bg!("{: ^10} | {: <100}", i, fi);
        }
      }
    }
    println_on_bg!("{:-^113}", "-");
    match selected {
      None=> {
        println_inst!("| {} | {}", " Select content to add to blank by ID. ", " CANCEL / C: Cancel and return to editing note");
      },
      Some(_) => {
        println_inst!("| {} | {}", "Select content to add to blank by ID.", "ENTER: Add selected content to blank.");
      }
    }
  }
  fn select_blank_fill_in(blank_type: Blank, selected: Vec<usize>) -> Option<usize> {
    loop {
      NoteArchive::display_blank_fill_in(blank_type, Some(selected.clone()));
      let mut fill_in_choice = String::new();
      let fill_in_attempt = io::stdin().read_line(&mut fill_in_choice);
      let selected_option = match fill_in_attempt {
        Ok(_) => fill_in_choice.trim().to_ascii_lowercase(),
        Err(e) => {
          println_err!("Failed to read line: {}", e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      };
      match &selected_option[..] {
        "" => {
          if selected.len() > 0 {
            return None;
          } else {
            println_err!("Please choose at least one fill-in for the current blank.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
        }
        _ => (),
      }
      let chosen_id: usize = match selected_option.parse() {
        Ok(num) => num,
        Err(e) => {
          println_err!("Failed to parse '{}' as int: {}", selected_option, e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      };
      let selected_content: usize = match blank_type {
        InternalMeeting => {
          match InternalMeetingFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        ExternalMeeting => {
          match ExternalMeetingFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        InternalDocument => {
          match InternalDocumentFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        ExternalDocument => {
          match ExternalDocumentFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        Appearance => {
          match AppearanceFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        SupportedParent => {
          match SupportedParentFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        ParentingSkill => {
          match ParentingSkillFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        CarePlanningTopic => {
          match CarePlanningTopicFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        YouthTopic => {
          match YouthTopicFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        ContactMethod => {
          match ContactMethodFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        ContactPurpose => {
          match ContactPurposeFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        FulfilledContactPurpose => {
          match FulfilledContactPurposeFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        Service => {
          match ServiceFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        MeetingMethod => {
          match MeetingMethodFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        SignatureMethod => {
          match SignatureMethodFillIn::iterator_of_blanks().nth(chosen_id) {
            Some(_) => chosen_id,
            None => {
              println_err!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        _ => panic!("Incompatible fill in type passed to fn 'select_blank_flll_in'"),
      };
      return Some(selected_content)
    }
  }
  fn display_icc_note_categories() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{:-^58}", " ICC note categories ");
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{: ^5} | {:-^50}", " ID ", " Note category ");
    for (i, cat) in ICCNoteCategory::iterator().enumerate() {
      let num_remaining = 50 - format!("{}", cat.clone()).chars().count();
      let mut spacer = String::new();
      for _ in 0..num_remaining {
        spacer.push_str(" ");
      }
      println_on_bg!("{: ^5} | {:-<50}{}", i, cat, spacer);
    }
    println_on_bg!("{:-^58}", "-");
    println_inst!(
      "| {} | {}",
      "Enter ID to select a note category.",
      "QUIT / Q: Quit menu",
    );
  }
  fn display_fp_note_categories() {
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{:-^58}", " FP note categories ");
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{:-^5} | {:-^50}", " ID ", " Note category ");
    for (i, cat) in FPNoteCategory::iterator().enumerate() {
      let num_remaining = 50 - format!("{}", cat.clone()).chars().count();
      let mut spacer = String::new();
      for _ in 0..num_remaining {
        spacer.push_str(" ");
      }
      println_on_bg!("{:-^5} | {:-<50}{}", i, cat, spacer);
    }
    println_on_bg!("{:-^58}", "-");
    println_inst!(
      "| {} | {}",
      "Enter ID to select a note category.",
      "QUIT / Q: Quit menu",
    );
  }
  fn display_fp_intervention_descriptions() {
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{:-^58}", " Intervention descriptions ");
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{:-^5} | {:-^50}", " ID ", " Description category ");
    for (i, cat) in FPNoteCategory::iterator_of_descriptions().enumerate() {
      let num_remaining = 50 - format!("{}", cat.clone()).chars().count();
      let mut spacer = String::new();
      for _ in 0..num_remaining {
        spacer.push_str(" ");
      }
      println_on_bg!("{:-^5} | {:-<50}{}", i, cat, spacer);
    }
    println_on_bg!("{:-^58}", "-");
    println_inst!(
      "| {} | {}",
      "Enter ID to select an intervention category.",
      "QUIT / Q: Quit menu",
    );
  }
  fn display_fp_intervention_responses() {
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{:-^58}", " Response descriptions ");
    println_on_bg!("{:-^58}", "-");
    println_on_bg!("{:-^5} | {:-^50}", " ID ", " Response category ");
    for (i, cat) in FPNoteCategory::iterator_of_responses().enumerate() {
      let num_remaining = 50 - format!("{}", cat.clone()).chars().count();
      let mut spacer = String::new();
      for _ in 0..num_remaining {
        spacer.push_str(" ");
      }
      println_on_bg!("{:-^5} | {:-<50}{}", i, cat, spacer);
    }
    println_on_bg!("{:-^58}", "-");
    println_inst!(
      "| {} | {}",
      "Enter ID to select an intervention category.",
      "QUIT / Q: Quit menu",
    );
  }
  fn choose_note_category(&self) -> Option<NoteCategory> {
    let current_role = match self.current_user().role {
      Icc => Icc,
      Fp => Fp,
    };
    loop {
      match current_role {
        Icc => {
          NoteArchive::display_icc_note_categories();
        },
        Fp => {
          NoteArchive::display_fp_note_categories();
        },
      }

      let mut ncat_choice = String::new();
      let ncat_attempt = io::stdin().read_line(&mut ncat_choice);
      let ncat_input = match ncat_attempt {
        Ok(_) => ncat_choice.trim().to_ascii_lowercase(),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &ncat_input[..] {
        "quit" | "q" | "cancel" | "c" => return None,
        _ => match ncat_input.parse::<usize>() {
          Err(e) => {
            println_err!("Failed to parse input as a number: {}. Try again.", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          },
          Ok(num) => {
            match current_role {
              Icc => {
                let iccncat = ICCNoteCategory::iterator().nth(num);
                match iccncat {
                  Some(icccat) => return Some(ICCNote(icccat)),
                  None => {
                    println_err!("Index out of bounds for available note categories. Please identify a note category by valid ID.");
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                  }
                }
              },
              Fp => {
                let fpncat = FPNoteCategory::iterator().nth(num);
                match fpncat {
                  Some(fpcat) => {
                    match fpcat {
                      Functioning | PlanAdditionalInformation => return Some(FPNote(fpcat)),
                      _ => {
                        let subcat = self.choose_fp_intervention(fpcat);
                        match subcat.clone() {
                          None => return None,
                          Some(_) => match fpcat {
                            DescriptionOfIntervention(_) => return Some(FPNote(DescriptionOfIntervention(subcat))),
                            ResponseToIntervention(_) => return Some(FPNote(ResponseToIntervention(subcat))),
                            Functioning | PlanAdditionalInformation => {
                              panic!("Pattern 'Functioning' and 'PlanAdditionalInformation' should have already been handled.");
                            },
                          }
                        }
                      },
                    }
                  },
                  None => {
                    println_err!("Index out of bounds for available note categories. Please identify a note category by valid ID.");
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                  },
                }
              },
            }
          }
        }
      }
    }
  }
  fn choose_fp_intervention(&self, ncat: FPNoteCategory) -> Option<FPIntervention> {
    loop {
      match ncat {
        DescriptionOfIntervention(_) => NoteArchive::display_fp_intervention_descriptions(),
        ResponseToIntervention(_) => NoteArchive::display_fp_intervention_responses(),
        _ => panic!("Invalid FP note category sent to fn 'choose_fp_intervention': {}", ncat),
      }

      let mut ncat_choice = String::new();
      let ncat_attempt = io::stdin().read_line(&mut ncat_choice);
      let ncat_input = match ncat_attempt {
        Ok(_) => ncat_choice.trim().to_ascii_lowercase(),
        Err(e) => {
          println_err!("Failed to read input: {}.", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &ncat_input[..] {
        "quit" | "q" => return None,
        _ => match ncat_input.parse::<usize>() {
          Err(e) => {
            println_err!("Failed to parse input as a number: {}. Try again.", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          },
          Ok(num) => {
            let intervention = match ncat {
              DescriptionOfIntervention(_) | ResponseToIntervention(_) => FPIntervention::iterator().nth(num),
              _ => panic!("Invalid FP note category sent to fn 'choose_fp_intervention': {}", ncat),
            };
            match intervention {
              Some(interv) => return Some(interv),
              None => {
                println_err!("Index out of bounds for available interventions. Please identify an intervention by valid ID.");
                thread::sleep(time::Duration::from_secs(1));
                continue;
              }
            }
          }
        }
      }
    }
  }
  fn autofill_note_blanks(&self, mut n: Note) -> Note {
    let current_client = self.current_client().clone();
    let current_collaterals = self.get_owned_current_collaterals();
    let primary_contacts = current_collaterals.iter().filter(|co| co.primary_contact ).map(|co_ref| co_ref.clone() ).collect::<Vec<Collateral>>();
    let primary_contacts_len = primary_contacts.len();
    let guardians = current_collaterals.iter().filter(|co| co.guardian ).map(|co_ref| co_ref.clone() ).collect::<Vec<Collateral>>();
    let guardians_len = guardians.len();
    let cpt = current_collaterals.iter().filter(|co| co.care_plan_team ).map(|co_ref| co_ref.clone() ).collect::<Vec<Collateral>>();
    let cpt_len = cpt.len();
    let current_collaterals_len = current_collaterals.len();
    let u = self.current_user().clone();
    let nd = self.current_note_day().clone();
    let up = self.get_pronouns_by_id(u.pronouns).unwrap().clone();
    let cp = self.get_pronouns_by_id(current_client.pronouns).unwrap().clone();
    let empty_blanks = n.get_empty_blanks_and_indexes();
    for (i, b) in empty_blanks {
      let i = i as u32;
      match b {
        CurrentUser => {
          n.blanks.insert(i, (b.clone(), format!("{}", u.role), vec![u.id]));
        }
        PartnerICCOrFP => {
          let (maybe_id, output_string) = match u.role {
            Icc => {
              match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "fp" || &co.title.to_ascii_lowercase()[..] == "family partner" ) {
                Some(co) => (Some(co.id), co.full_name_and_title()),
                None => (None, String::from("Family Partner")),
              }
            },
            Fp => {
              match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "icc" || &co.title.to_ascii_lowercase()[..] == "intensive care coordinator" ) {
                Some(co) => (Some(co.id), co.full_name_and_title()),
                None => (None, String::from("Intensive Care Coordinator")),
              }
            },
          };
          // the generic versions "Family Partner" and "Intensive..." are not being used
          // because only inserting if the role exists. 
          match maybe_id {
            Some(id) => {
              n.blanks.insert(i, (b.clone(), output_string, vec![id]));
            }
            None => (),
          }
        }
        CurrentClientName => {
          let blank_string = current_client.full_name_with_label();
          let id_vec = vec![current_client.id];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        }
        AllCollaterals => {
          if current_collaterals_len > 0 {
            let blank_string = if current_collaterals_len > 1 {
              format!(
                "{} {} {}",
                current_collaterals[..current_collaterals.len()-1].iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", "),
                "and",
                current_collaterals[current_collaterals.len()-1].full_name_and_title(),
              )
            } else {
              current_collaterals[0].full_name_and_title()
            };
            let id_vec = current_client.foreign_keys["collateral_ids"].to_owned();
            n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
            n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
          }
        },
        PrimaryContact => {
          if primary_contacts_len > 0 {
            let blank_string = if primary_contacts_len > 1 {
              format!(
                "{} {} {}",
                primary_contacts[..primary_contacts_len-1].iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", "),
                "and",
                primary_contacts[primary_contacts_len-1].full_name_and_title(),
              )
            } else {
              primary_contacts[0].full_name_and_title()
            };
            let id_vec = primary_contacts.iter().map(|co| co.id ).collect::<Vec<u32>>();
            n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
            let mut old_ids = n.foreign_keys["collateral_ids"].clone();
            for new_id in id_vec {
              if !old_ids.clone().iter().any(|o_id| o_id == &new_id ) {
                old_ids.push(new_id);
              }
            }
            n.foreign_keys.insert(String::from("collateral_ids"), old_ids);
          }
        },
        Guardian => {
          if guardians_len > 0 {
            let blank_string = if guardians_len > 1 {
              format!(
                "{} {} {}",
                guardians[..guardians_len-1].iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", "),
                "and",
                guardians[guardians_len-1].full_name_and_title(),
              )
            } else {
              guardians[0].full_name_and_title()
            };
            let id_vec = guardians.iter().map(|co| co.id ).collect::<Vec<u32>>();
            n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
            let mut old_ids = n.foreign_keys["collateral_ids"].clone();
            for new_id in id_vec {
              if !old_ids.clone().iter().any(|o_id| o_id == &new_id ) {
                old_ids.push(new_id);
              }
            }
            n.foreign_keys.insert(String::from("collateral_ids"), old_ids);
          }
        },
        CarePlanTeam => {
          if cpt_len > 0 {
            let blank_string = if cpt_len > 1 {
              format!(
                "{} {} {}",
                cpt[..cpt_len-1].iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", "),
                "and",
                cpt[cpt_len-1].full_name_and_title(),
              )
            } else {
              cpt[0].full_name_and_title()
            };
            let id_vec = cpt.iter().map(|co| co.id ).collect::<Vec<u32>>();
            n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
            let mut old_ids = n.foreign_keys["collateral_ids"].clone();
            for new_id in id_vec {
              if !old_ids.clone().iter().any(|o_id| o_id == &new_id ) {
                old_ids.push(new_id);
              }
            }
            n.foreign_keys.insert(String::from("collateral_ids"), old_ids);
          }
        },
        Pronoun1ForUser => {
          let blank_string = up.subject.clone();
          let id_vec = vec![u.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun2ForUser => {
          let blank_string = up.object.clone();
          let id_vec = vec![u.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun3ForUser => {
          let blank_string = up.possessive_determiner.clone();
          let id_vec = vec![u.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun4ForUser => {
          let blank_string = up.possessive.clone();
          let id_vec = vec![u.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun1ForClient => {
          let blank_string = cp.subject.clone();
          let id_vec = vec![current_client.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun2ForClient => {
          let blank_string = cp.object.clone();
          let id_vec = vec![current_client.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun3ForClient => {
          let blank_string = cp.possessive_determiner.clone();
          let id_vec = vec![current_client.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun4ForClient => {
          let blank_string = cp.possessive.clone();
          let id_vec = vec![current_client.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        TodayDate => {
          let today = Local::now().naive_local().date();
          let blank_string = format!("{}, {}-{}-{}", today.weekday(), today.year(), today.month(), today.day());
          let id_vec = vec![];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        NoteDayDate => {
          let nd_date = nd.date;
          let nd_date_string = format!("{}, {}-{}-{}", nd_date.weekday(), nd_date.year(), nd_date.month(), nd_date.day());
          let blank_string = nd_date_string.clone();
          let id_vec = vec![nd.id.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        // requires checking if the blank has been filled
        Pronoun1ForBlank(b_id_opt) => {
          let id_vec = vec![];
          match b_id_opt {
            Some(b_id) => {
              match n.blanks.get(&b_id) {
                Some(b_tup) => {
                  match b_tup.0 {
                    CurrentUser => {
                      let p = self.get_pronouns_by_id(self.current_user().pronouns).unwrap();
                      n.blanks.insert(i, (b.clone(), p.subject.clone(), id_vec));
                    },
                    CurrentClientName => {
                      let blank_string = match self.get_pronouns_by_id(self.current_client().pronouns) {
                        Some(p) => p.subject.clone(),
                        None => panic!("The current user's pronouns cannot be entered due to a missing record."),
                      };
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                    },
                    Collaterals => {
                      let collat_ids = b_tup.2.to_owned();
                      if collat_ids.len() > 0 {
                        let blank_string = if collat_ids.len() > 1 {
                          String::from("they")
                        } else {
                          match self.get_pronouns_by_id(collat_ids[0]) {
                            Some(p) => p.subject.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    AllCollaterals => {
                      let collats = self.current_client_collaterals();
                      if collats.len() > 0 {
                        let blank_string = if collats.len() > 1 {
                          String::from("they")
                        } else {
                          match self.get_pronouns_by_id(collats[0].id) {
                            Some(p) => p.subject.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    PrimaryContact => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.primary_contact )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("they")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.subject.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    Guardian => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.guardian )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("they")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.subject.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    CarePlanTeam => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.care_plan_team )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("they")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.subject.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    PartnerICCOrFP => {
                      let blank_string = match u.role {
                        Icc => {
                          match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "fp" || &co.title.to_ascii_lowercase()[..] == "family partner" ) {
                            Some(co) => self.get_pronouns_by_id(co.pronouns).unwrap().subject.clone(),
                            None => String::from("Family Partner"),
                          }
                        },
                        Fp => {
                          match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "icc" || &co.title.to_ascii_lowercase()[..] == "intensive care coordinator" ) {
                            Some(co) => self.get_pronouns_by_id(co.pronouns).unwrap().subject.clone(),
                            None => String::from("Intensive Care Coordinator"),
                          }
                        },
                      };
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                    },
                    _ => {
                      n.blanks.insert(i, (b.clone(), String::from("PRONOUNS"), vec![]));
                    }
                    // panic!("A pronoun blank was connected to a type of blank for which pronouns do not apply."),
                  }
                  continue;
                },
                None => (), 
              }
            }
            None => (),
          }
        },
        Pronoun2ForBlank(b_id_opt) => {
          let id_vec = vec![];
          match b_id_opt {
            Some(b_id) => {
              match n.blanks.get(&b_id) {
                Some(b_tup) => {
                  match b_tup.0 {
                    CurrentUser => {
                      let p = self.get_pronouns_by_id(self.current_user().pronouns).unwrap();
                      n.blanks.insert(i, (b.clone(), p.object.clone(), id_vec));
                    },
                    CurrentClientName => {
                      let blank_string = match self.get_pronouns_by_id(self.current_client().pronouns) {
                        Some(p) => p.object.clone(),
                        None => panic!("The current user's pronouns cannot be entered due to a missing record."),
                      };
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                    },
                    Collaterals => {
                      let collat_ids = b_tup.2.to_owned();
                      if collat_ids.len() > 0 {
                        let blank_string = if collat_ids.len() > 1 {
                          String::from("them")
                        } else {
                          match self.get_pronouns_by_id(collat_ids[0]) {
                            Some(p) => p.object.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    AllCollaterals => {
                      let collats = self.current_client_collaterals();
                      if collats.len() > 0 {
                        let blank_string = if collats.len() > 1 {
                          String::from("them")
                        } else {
                          match self.get_pronouns_by_id(collats[0].id) {
                            Some(p) => p.object.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    PrimaryContact => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.primary_contact )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("them")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.object.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    Guardian => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.guardian )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("them")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.object.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    CarePlanTeam => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.care_plan_team )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("them")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.object.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    PartnerICCOrFP => {
                      let blank_string = match u.role {
                        Icc => {
                          match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "fp" || &co.title.to_ascii_lowercase()[..] == "family partner" ) {
                            Some(co) => self.get_pronouns_by_id(co.pronouns).unwrap().object.clone(),
                            None => String::from("Family Partner"),
                          }
                        },
                        Fp => {
                          match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "icc" || &co.title.to_ascii_lowercase()[..] == "intensive care coordinator" ) {
                            Some(co) => self.get_pronouns_by_id(co.pronouns).unwrap().object.clone(),
                            None => String::from("Intensive Care Coordinator"),
                          }
                        },
                      };
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                    },
                    _ => {
                      n.blanks.insert(i, (b.clone(), String::from("PRONOUNS"), vec![]));
                    }
                    // panic!("A pronoun blank was connected to a type of blank for which pronouns do not apply."),
                  }
                },
                None => (), 
              }
            }
            None => (),
          }
        },
        Pronoun3ForBlank(b_id_opt) => {
          let id_vec = vec![];
          match b_id_opt {
            Some(b_id) => {
              match n.blanks.get(&b_id) {
                Some(b_tup) => {
                  match b_tup.0 {
                    CurrentUser => {
                      let p = self.get_pronouns_by_id(self.current_user().pronouns).unwrap();
                      n.blanks.insert(i, (b.clone(), p.possessive_determiner.clone(), id_vec));
                    },
                    CurrentClientName => {
                      let blank_string = match self.get_pronouns_by_id(self.current_client().pronouns) {
                        Some(p) => p.possessive_determiner.clone(),
                        None => panic!("The current user's pronouns cannot be entered due to a missing record."),
                      };
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                    },
                    Collaterals => {
                      let collat_ids = b_tup.2.to_owned();
                      if collat_ids.len() > 0 {
                        let blank_string = if collat_ids.len() > 1 {
                          String::from("their")
                        } else {
                          match self.get_pronouns_by_id(collat_ids[0]) {
                            Some(p) => p.possessive_determiner.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    AllCollaterals => {
                      let collats = self.current_client_collaterals();
                      if collats.len() > 0 {
                        let blank_string = if collats.len() > 1 {
                          String::from("their")
                        } else {
                          match self.get_pronouns_by_id(collats[0].id) {
                            Some(p) => p.possessive_determiner.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    PrimaryContact => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.primary_contact )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("their")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.possessive_determiner.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    Guardian => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.guardian )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("their")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.possessive_determiner.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    CarePlanTeam => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.care_plan_team )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("their")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.possessive_determiner.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    PartnerICCOrFP => {
                      let blank_string = match u.role {
                        Icc => {
                          match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "fp" || &co.title.to_ascii_lowercase()[..] == "family partner" ) {
                            Some(co) => self.get_pronouns_by_id(co.pronouns).unwrap().possessive_determiner.clone(),
                            None => String::from("Family Partner's"),
                          }
                        },
                        Fp => {
                          match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "icc" || &co.title.to_ascii_lowercase()[..] == "intensive care coordinator" ) {
                            Some(co) => self.get_pronouns_by_id(co.pronouns).unwrap().possessive_determiner.clone(),
                            None => String::from("Intensive Care Coordinator's"),
                          }
                        },
                      };
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                    },
                    _ => {
                      n.blanks.insert(i, (b.clone(), String::from("PRONOUNS"), vec![]));
                    }
                    // panic!("A pronoun blank was connected to a type of blank for which pronouns do not apply."),
                  }
                },
                None => (), 
              }
            }
            None => (),
          }
        },
        Pronoun4ForBlank(b_id_opt) => {
          let id_vec = vec![];
          match b_id_opt {
            Some(b_id) => {
              match n.blanks.get(&b_id) {
                Some(b_tup) => {
                  match b_tup.0 {
                    CurrentUser => {
                      let p = self.get_pronouns_by_id(self.current_user().pronouns).unwrap();
                      n.blanks.insert(i, (b.clone(), p.possessive.clone(), id_vec));
                    },
                    CurrentClientName => {
                      let blank_string = match self.get_pronouns_by_id(self.current_client().pronouns) {
                        Some(p) => p.possessive.clone(),
                        None => panic!("The current user's pronouns cannot be entered due to a missing record."),
                      };
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                    },
                    Collaterals => {
                      let collat_ids = b_tup.2.to_owned();
                      if collat_ids.len() > 0 {
                        let blank_string = if collat_ids.len() > 1 {
                          String::from("theirs")
                        } else {
                          match self.get_pronouns_by_id(collat_ids[0]) {
                            Some(p) => p.possessive.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    AllCollaterals => {
                      let collats = self.current_client_collaterals();
                      if collats.len() > 0 {
                        let blank_string = if collats.len() > 1 {
                          String::from("theirs")
                        } else {
                          match self.get_pronouns_by_id(collats[0].id) {
                            Some(p) => p.possessive.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    PrimaryContact => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.primary_contact )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("theirs")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.possessive.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    Guardian => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.guardian )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("theirs")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.possessive.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    CarePlanTeam => {
                      let pcs = self.current_client_collaterals().iter()
                        .filter(|co| co.care_plan_team )
                        .map(|co| co.id )
                        .collect::<Vec<u32>>();
                      if pcs.len() > 0 {
                        let blank_string = if pcs.len() > 1 {
                          String::from("theirs")
                        } else {
                          match self.get_pronouns_by_id(pcs[0]) {
                            Some(p) => p.possessive.clone(),
                            None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                          }
                        };
                        n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      }
                    },
                    PartnerICCOrFP => {
                      let blank_string = match u.role {
                        Icc => {
                          match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "fp" || &co.title.to_ascii_lowercase()[..] == "family partner" ) {
                            Some(co) => self.get_pronouns_by_id(co.pronouns).unwrap().possessive.clone(),
                            None => String::from("Family Partner's"),
                          }
                        },
                        Fp => {
                          match current_collaterals.iter().find(|co| &co.title.to_ascii_lowercase()[..] == "icc" || &co.title.to_ascii_lowercase()[..] == "intensive care coordinator" ) {
                            Some(co) => self.get_pronouns_by_id(co.pronouns).unwrap().possessive.clone(),
                            None => String::from("Intensive Care Coordinator's"),
                          }
                        },
                      };
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                    },
                    _ => {
                      n.blanks.insert(i, (b.clone(), String::from("PRONOUNS"), vec![]));
                    }
                    // panic!("A pronoun blank was connected to a type of blank for which pronouns do not apply."),
                  }
                },
                None => (), 
              }
            }
            None => (),
          }
        },
        _ => (), // all others need to be filled in based on user input
      }
    }
    n
  }
  fn create_note_from_template_get_id(&mut self, nt_id: Option<u32>) -> Option<u32> {
    let nt_id = match nt_id {
      Some(id) => id,
      None => {
        let id = loop {
          let nt_id_opt = self.select_note_template();
          break match nt_id_opt {
            Some(id) => id,
            None => {
              let decision = loop {
                let mut cancel_choice = String::new();
                println_yel!("You must select a note template to build a note from a template.");
                println_inst!("Cancel writing note? ( Y / N )");
                let cancel_choice_attempt = io::stdin().read_line(&mut cancel_choice);
                break match cancel_choice_attempt {
                  Ok(_) => cancel_choice.trim().to_ascii_lowercase(),
                  Err(e) => {
                    println_err!("Failed to read line: {}", e);
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  }
                };
              };
              match &decision[..] {
                "yes" | "y" => {
                  return None;
                },
                "no" | "n" => {
                  continue;
                },
                _ => {
                  println_err!("Invalid command.");
                  continue;
                },
              }
            }
          }
        };
        id
      }
    };
    
    let nt = match self.get_note_template_option_by_id(nt_id) {
      Some(nt) => nt,
      None => {
        panic!("Failed to load a note template with the ID passed from the function that creates new note templates.");
      }
    };
    let nst = nt.structure;
    let ncnt = nt.content.clone();

    let ncat = match self.current_user().role {
      Icc => {
        match nst {
          CarePlan => ICCNote(FaceToFaceContactWithClient),
          Intake => ICCNote(FaceToFaceContactWithClient),
          Assessment => ICCNote(FaceToFaceContactWithClient),
          Sncd => ICCNote(FaceToFaceContactWithClient),
          HomeVisit => ICCNote(FaceToFaceContactWithClient),
          AgendaPrep => ICCNote(FaceToFaceContactWithClient),
          Debrief => ICCNote(FaceToFaceContactWithClient),
          PhoneCall => ICCNote(TelephoneContactWithClient),
          Scheduling => ICCNote(CareCoordination),
          SentEmail => ICCNote(CareCoordination),
          Referral => ICCNote(CareCoordination),
          ParentSupport => ICCNote(FaceToFaceContactWithClient),
          SentCancellation => ICCNote(CareCoordination),
          ParentAppearance => ICCNote(FaceToFaceContactWithClient),
          ParentSkills => ICCNote(FaceToFaceContactWithClient),
          FailedContactAttempt => ICCNote(MemberOutreachNoShow),
          CategorizedEmails => ICCNote(Documentation),
          AuthorizationRequested => ICCNote(CareCoordination),
          AuthorizationIssued => ICCNote(CareCoordination),
          CollateralOutreach => ICCNote(CareCoordination),
          UpdateFromCollateral => ICCNote(CareCoordination),
          InvitedToMeeting => ICCNote(CareCoordination),
          SentDocument => ICCNote(CareCoordination),
          UpdatedDocument => ICCNote(Documentation),
          DiscussCommunication => ICCNote(CareCoordination),
          ReceivedVerbalConsent => ICCNote(FaceToFaceContactWithClient),
          ReceivedWrittenConsent => ICCNote(FaceToFaceContactWithClient),
          DocumentationStructure=> ICCNote(Documentation),
          BrainstormContribution => ICCNote(FaceToFaceContactWithClient),
          CustomStructure => {
            loop {
              match self.choose_note_category() {
                Some(ncat) => break ncat,
                None => {
                  let decision = loop {
                    let mut cancel_choice = String::new();
                    println_yel!("You must select a note category to fill in a custom note template.");
                    println_inst!("Cancel writing note? ( Y / N )");
                    let cancel_choice_attempt = io::stdin().read_line(&mut cancel_choice);
                    let cancel_choice_content = match cancel_choice_attempt {
                      Ok(_) => cancel_choice.trim().to_ascii_lowercase(),
                      Err(e) => {
                        println_err!("Failed to read line: {}", e);
                        thread::sleep(time::Duration::from_secs(2));
                        continue;
                      }
                    };
                    break cancel_choice_content;
                  };
                  match &decision[..] {
                    "yes" | "y" => {
                      return None;
                    },
                    "no" | "n" => {
                      continue;
                    },
                    _ => {
                      println_err!("Invalid command.");
                      continue;
                    },
                  }
                }
              }
            }
          }
        }
      }
      Fp => {
        match nst {
          CarePlan => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          Intake => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          Assessment => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          Sncd => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          HomeVisit => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          AgendaPrep => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          Debrief => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          PhoneCall => FPNote(DescriptionOfIntervention(Some(TelephoneSupport))),
          Scheduling => FPNote(PlanAdditionalInformation),
          SentEmail => FPNote(PlanAdditionalInformation),
          Referral => FPNote(DescriptionOfIntervention(Some(CollateralContact))),
          ParentSupport => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          SentCancellation => FPNote(DescriptionOfIntervention(Some(ProviderOutreachToPerson))),
          ParentAppearance => FPNote(Functioning),
          ParentSkills => FPNote(Functioning),
          FailedContactAttempt => FPNote(DescriptionOfIntervention(Some(ProviderOutreachToPerson))),
          CategorizedEmails => FPNote(DescriptionOfIntervention(Some(FPInterventionDocumentation))),
          AuthorizationRequested => FPNote(PlanAdditionalInformation),
          AuthorizationIssued => FPNote(PlanAdditionalInformation),
          CollateralOutreach => FPNote(DescriptionOfIntervention(Some(CollateralContact))),
          UpdateFromCollateral => FPNote(DescriptionOfIntervention(Some(CollateralContact))),
          InvitedToMeeting => FPNote(PlanAdditionalInformation),
          SentDocument => FPNote(PlanAdditionalInformation),
          UpdatedDocument => FPNote(DescriptionOfIntervention(Some(FPInterventionDocumentation))),
          DiscussCommunication => FPNote(DescriptionOfIntervention(Some(DirectTimeWithProviders))),
          ReceivedVerbalConsent => FPNote(PlanAdditionalInformation),
          ReceivedWrittenConsent => FPNote(PlanAdditionalInformation),
          DocumentationStructure => FPNote(DescriptionOfIntervention(Some(FPInterventionDocumentation))),
          BrainstormContribution => FPNote(DescriptionOfIntervention(Some(FaceToFaceContact))),
          CustomStructure => {
            loop {
              match self.choose_note_category() {
                Some(ncat) => break ncat,
                None => {
                  let decision = loop {
                    let mut cancel_choice = String::new();
                    println_yel!("You must select a note category to fill in a custom note template.");
                    println_inst!("Cancel writing note? ( Y / N )");
                    let cancel_choice_attempt = io::stdin().read_line(&mut cancel_choice);
                    let cancel_choice_content = match cancel_choice_attempt {
                      Ok(_) => cancel_choice.trim().to_ascii_lowercase(),
                      Err(e) => {
                        println_err!("Failed to read line: {}", e);
                        thread::sleep(time::Duration::from_secs(2));
                        continue;
                      }
                    };
                    break cancel_choice_content;
                  };
                  match &decision[..] {
                    "yes" | "y" => {
                      continue;
                    },
                    "no" | "n" => {
                      continue;
                    },
                    _ => {
                      println_err!("Invalid command.");
                      continue;
                    },
                  }
                }
              }

            }
          },
        }
      }
    };

    let client_id = match self.foreign_key.get("current_client_id") {
      Some(id) => *id,
      None => self.select_client(),
    };

    self.load_client(client_id).unwrap();

    let nd_id = match self.foreign_key.get("current_note_day_id").clone() {
      Some(id) => *id,
      None => {
        let nds = self.current_user_note_days();
        let max_id: Option<u32> = match nds.iter().max_by(|a, b| a.date.cmp(&b.date) ) {
          Some(nd) => Some(nd.id),
          None => None,
        };
        let max_date: Option<NaiveDate> = match nds.iter().max_by(|a, b| a.date.cmp(&b.date) ) {
          Some(nd) => Some(nd.date.clone()),
          None => None,
        };
        let today = Local::now().naive_local().date();
        match max_id {
          Some(m) => {
            if max_date.unwrap() == today {
              self.foreign_key.insert(String::from("current_note_day"), m);
              m
            } else {
              let maybe_new_nd = self.generate_unique_new_note_day(today, self.current_user().id, self.current_client().id);
              match maybe_new_nd {
                Err(_) => panic!("Failed to generate new note day."),
                Ok(nd) => {
                  self.save_note_day(nd.clone());
                  self.load_note_day(nd.id).unwrap();
                  self.write_to_files();
                  nd.id
                }
              }
            }
          },
          None => {
            let maybe_new_nd = self.generate_unique_new_note_day(today, self.current_user().id, self.current_client().id);
            match maybe_new_nd {
              Err(_) => panic!("Failed to generate new note day."),
              Ok(nd) => {
                self.save_note_day(nd.clone());
                self.load_note_day(nd.id).unwrap();
                self.write_to_files();
                nd.id
              }
            }
          }
        }
      },
    };

    self.load_note_day(nd_id).unwrap();

    let date = self.current_note_day().date.clone();
    let mut n = self.generate_note(date, ncat, nst, ncnt).unwrap();

    let mut focus_id_option: Option<u32> = None;
    'choice: loop {
      n = self.autofill_note_blanks(n);
      
      // increments the current blank by going to the next one only when the current is filled,
      // or alternatively when focus_id_option is set to Some(id) because the user selected it

      let mut empty_blanks = n.get_empty_blanks_and_indexes();
      if empty_blanks.len() > 0 {
        if let None = focus_id_option.clone() {
          focus_id_option = Some(empty_blanks[0].0);
        }
      } else {
        break;
      }

      let (i, b) = match focus_id_option {
        Some(f_id) => {
          let f_id_b = match empty_blanks.iter().find(|b_tup| b_tup.0 == f_id ) {
            Some(empty_blank_tup) => empty_blank_tup.1,
            None => n.blanks[&f_id].0,
          };
          (f_id, f_id_b)
        },
        None => (empty_blanks[0].0, empty_blanks[0].1),
      };
      let i = i as u32;

      let choice = loop {
        let mut note_choice = String::new();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        n.display_content(focus_id_option, None);
        println_on_bg!("{:-^58}", "-");
        println_inst!(
          "| {} | {}",
          "SKIP / S: Skip to next blank",
          "DELETE / D: Delete content of current blank",
        );
        println_inst!(
          "| {} | {}",
          "CLEAR / C: Clear content of all blanks",
          "QUIT / Q: Quit menu",
        );
        println_inst!("ENTER to fill in the current blank.");
        println_inst!("Choose blank by ID to switch current blank.");

        // display options here

        let note_attempt = io::stdin().read_line(&mut note_choice);
        match note_attempt {
          Ok(_) => break note_choice.trim().to_ascii_lowercase(),
          Err(e) => {
            println_err!("Failed to read line: {}", e);
            thread::sleep(time::Duration::from_secs(2));
            continue;
          },
        }
      };
      match &choice[..] {
        "skip" | "s" => {
          match focus_id_option {
            Some(focus_id) => {
              if focus_id + 1 < empty_blanks.len() as u32 {
                focus_id_option = Some(focus_id + 1);
              }
              continue;
            },
            None => {
              if empty_blanks.len() > 0 {
                focus_id_option = Some(empty_blanks[0].0);
                continue;
              }
            },
          }
        },
        "delete" | "d" => {
          match n.blanks.get(&i) {
            Some(b_tup) => {
              loop {
                let mut delete_choice = String::new();
                println_yel!("Delete blank currently filled in with '{}'?", b_tup.1);
                let delete_choice_attempt = io::stdin().read_line(&mut delete_choice);
                let delete_choice_content = match delete_choice_attempt {
                  Ok(_) => delete_choice.trim().to_ascii_lowercase(),
                  Err(e) => {
                    println_err!("Failed to read line: {}", e);
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  }
                };
                match &delete_choice_content[..] {
                  "yes" | "y" => {
                    let current_blank = n.blanks.get(&i).unwrap();
                    let current_blank_type = current_blank.0.clone();
                    let current_blank_ids = current_blank.2.clone();
                    n.blanks.remove(&i);
                    match current_blank_type {
                      Collaterals | AllCollaterals => {
                        let mut collat_ids_included_elsewhere: Vec<u32> = vec![];
                        for (_idx, blank_tup) in &n.blanks {
                          match blank_tup.0 {
                            Collaterals | AllCollaterals => {
                              for co_id in &blank_tup.2 {
                                if !collat_ids_included_elsewhere.clone().iter().any(|id| id == co_id ) {
                                  collat_ids_included_elsewhere.push(*co_id)
                                }
                              }
                            },
                            _ => (),
                          }
                        }
                        for co_id in current_blank_ids {
                          let mut new_collat_ids: Vec<u32> = vec![];
                          if collat_ids_included_elsewhere.iter().any(|id| id == &co_id ) {
                            new_collat_ids.push(co_id);
                          }
                          n.foreign_keys.insert(String::from("collateral_ids"), new_collat_ids);
                        }
                      },
                      _ => (),
                    }
                    if current_blank_type.has_pronouns() {
                      n.delete_associated_pronouns(&i);
                    }
                    break;
                  },
                  "no" | "n" => {
                    break;
                  },
                  _ => {
                    println_err!("Invalid command.");
                    continue;
                  },
                }
              }
            },
            None => {
              println_err!("Current blank is already empty.");
              thread::sleep(time::Duration::from_secs(2));
              continue;
            },
          }
        },
        "clear" | "c" => {
          if n.blanks.len() == 0 {
            println_err!("All blanks are already empty.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            loop {
              let mut clear_choice = String::new();
              println_yel!("Delete all blanks from the current template and start over?");
              let clear_choice_attempt = io::stdin().read_line(&mut clear_choice);
              let clear_choice_content = match clear_choice_attempt {
                Ok(_) => clear_choice.trim().to_ascii_lowercase(),
                Err(e) => {
                  println_err!("Failed to read line: {}", e);
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              };
              match &clear_choice_content[..] {
                "yes" | "y" => {
                  n.blanks.clear();
                  break;
                },
                "no" | "n" => {
                  break;
                },
                _ => {
                  println_err!("Invalid command.");
                  continue;
                },
              }
            }
          }
        },
        "quit" | "q" => {
          return None;
        },
        "" => {
          if empty_blanks.len() == 0 {
            break;
          }
          match b {
            PartnerICCOrFP => {
              match n.blanks.get(&i) {
                Some(_) => (),
                None => {
                  match self.current_user().role {
                    Icc => println_inst!("No FP found in client collaterals."),
                    Fp => println_inst!("No ICC found in client collaterals."),
                  }
                  println_inst!("Press ENTER to choose or edit collaterals on the next menu.");
                  let mut s = String::new();
                  let input_attempt = io::stdin().read_line(&mut s);
                  match input_attempt {
                    _ => (),
                  }
                  let (blank_string, id_vec) = self.select_collaterals();
                  n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
                  n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
                }
              }
            },
            ClientGoal => {
              let mut fill_ins: Vec<u32> = vec![];
              let final_blank_string = loop {
                let blank_id = match self.select_client_goals(Some(fill_ins.clone())) {
                  Some(s) => s,
                  None => {
                    let fill_in_strings = fill_ins.iter().map(|g_id| self.get_goal_by_id(*g_id).unwrap().goal.clone() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      String::new()
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              if fill_ins.len() > 0 {
                n.blanks.insert(i, (b.clone(), final_blank_string, fill_ins));
              }
            },
            Collaterals => {
              let (blank_string, id_vec) = self.select_collaterals();
              n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
              n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
            },
            AllCollaterals => {
              loop {
                match n.blanks.get(&i) {
                  Some(_) => break,
                  None => {
                    println_inst!("Client currently has no collaterals added.");
                    println_inst!("Please add at least one collateral to proceed.");
                    self.choose_edit_client_collaterals();
                    let current_client = self.current_client();
                    let current_collaterals = self.current_client_collaterals();
                    let current_collaterals_len = current_collaterals.len();
                    if current_collaterals_len > 0 {
                      let blank_string = if current_collaterals_len > 1 {
                        format!(
                          "{} {} {}",
                          current_collaterals[..current_collaterals.len()-1].iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", "),
                          "and",
                          current_collaterals[current_collaterals.len()-1].full_name_and_title(),
                        )
                      } else {
                        current_collaterals[0].full_name_and_title()
                      };
                      let id_vec = current_client.foreign_keys["collateral_ids"].to_owned();
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
                      n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
                    }
                  },
                }
              }
            }
            PrimaryContact => {
              match n.blanks.get(&i) {
                Some(_) => (),
                None => {
                  println_inst!("No primary contact found for current client.");
                  println_inst!("Press ENTER to choose or edit collaterals on the next menu.");
                  let mut s = String::new();
                  let input_attempt = io::stdin().read_line(&mut s);
                  match input_attempt {
                    _ => (),
                  }
                  let (blank_string, id_vec) = self.select_collaterals();
                  n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
                  n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
                }
              }
            }
            Guardian => {
              match n.blanks.get(&i) {
                Some(_) => (),
                None => {
                  println_inst!("No guardian found for current client.");
                  println_inst!("Press ENTER to choose or edit collaterals on the next menu.");
                  let mut s = String::new();
                  let input_attempt = io::stdin().read_line(&mut s);
                  match input_attempt {
                    _ => (),
                  }
                  let (blank_string, id_vec) = self.select_collaterals();
                  n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
                  n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
                }
              }
            }
            CarePlanTeam => {
              match n.blanks.get(&i) {
                Some(_) => (),
                None => {
                  println_inst!("No Care Plan Team members found for current client.");
                  println_inst!("Press ENTER to choose or edit collaterals on the next menu.");
                  let mut s = String::new();
                  let input_attempt = io::stdin().read_line(&mut s);
                  match input_attempt {
                    _ => (),
                  }
                  let (blank_string, id_vec) = self.select_collaterals();
                  n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
                  n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
                }
              }
            }
            InternalDocument => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(InternalDocument, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| InternalDocumentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            ExternalDocument => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(ExternalDocument, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| ExternalDocumentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            InternalMeeting => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(InternalMeeting, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| InternalMeetingFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            ExternalMeeting => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(ExternalMeeting, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| ExternalMeetingFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            Appearance => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(Appearance, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| AppearanceFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            SupportedParent => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(SupportedParent, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| SupportedParentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            ParentingSkill => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(ParentingSkill, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| ParentingSkillFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            CarePlanningTopic => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(CarePlanningTopic, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| CarePlanningTopicFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            YouthTopic => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(YouthTopic, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| YouthTopicFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            ContactMethod => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(ContactMethod, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| ContactMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            ContactPurpose => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(ContactPurpose, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| ContactPurposeFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            FulfilledContactPurpose => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(FulfilledContactPurpose, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| FulfilledContactPurposeFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            Service => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(Service, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| ServiceFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            MeetingMethod => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(MeetingMethod, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| MeetingMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            SignatureMethod => {
              let mut fill_ins: Vec<usize> = vec![];
              let final_blank_string = loop {
                let blank_id = match Self::select_blank_fill_in(SignatureMethod, fill_ins.clone()) {
                  Some(s) => s,
                  None => {
                    let fill_in_objects = fill_ins.iter().map(|fi| SignatureMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                    let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      continue;
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), final_blank_string, id_vec));
            },
            CustomBlank => {
              loop {
                let mut custom_choice = String::new();
                println_inst!("Enter custom content for the current blank. CANCEL to cancel.");
                let custom_attempt = io::stdin().read_line(&mut custom_choice);
                let custom_content = match custom_attempt {
                  Ok(_) => custom_choice.trim(),
                  Err(e) => {
                    println_err!("Failed to read line: {}", e);
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  }
                };
                let blank_string = custom_content.to_string();
                let id_vec = vec![];
                if &blank_string.to_ascii_lowercase()[..] == "cancel" {
                  break;
                }
                n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                break;
              }
            },
            _ => (),
          }
          empty_blanks = n.get_empty_blanks_and_indexes();
          if empty_blanks.len() > 0 {
            focus_id_option = Some(empty_blanks[0].0);
          }
        },
        _ => {
          let blank_id: usize = match choice.parse() {
            Ok(num) => num,
            Err(_) => {
              match b {
                CustomBlank => {
                  loop {
                    loop {
                      let mut fill_choice = String::new();
                      println_inst!("Fill custom blank with '{}'? ( Y / N )", choice);
                      let fill_choice_attempt = io::stdin().read_line(&mut fill_choice);
                      let fill_choice_content = match fill_choice_attempt {
                        Ok(_) => fill_choice.trim().to_ascii_lowercase(),
                        Err(e) => {
                          println_err!("Failed to read line: {}", e);
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        }
                      };
                      match &fill_choice_content[..] {
                        "yes" | "y" => {
                          let blank_string = choice.clone();
                          let id_vec = vec![];
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                          continue 'choice;
                        },
                        "no" | "n" => {
                          break;
                        },
                        _ => {
                          println_err!("Invalid command.");
                          continue;
                        },
                      }
                    }
                    loop {
                      let mut custom_choice = String::new();
                      println_inst!("Enter custom content for the current blank.");
                      let custom_attempt = io::stdin().read_line(&mut custom_choice);
                      let custom_content = match custom_attempt {
                        Ok(_) => custom_choice.trim(),
                        Err(e) => {
                          println_err!("Failed to read line: {}", e);
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        }
                      };
                      let blank_string = custom_content.to_string();
                      let id_vec = vec![];
                      if &blank_string.to_ascii_lowercase()[..] == "cancel" {
                        continue 'choice;
                      }
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      continue 'choice;
                    }
                  }
                },
                _ => {
                  println_err!("Invalid command.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              }
            },
          };
          if blank_id > n.number_of_blanks() {
            println_err!("Invalid blank ID.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          focus_id_option = Some(blank_id as u32)
        },
      }
    }

    let note_id = n.id;
    self.save_note(n);

    self.write_to_files();
    Some(note_id)
    
  }
  fn display_blank_menus() {

    let fillables = Blank::vec_of_fillables();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^113}", "-");
    println_on_bg!("{:-^113}", " All default fill-ins for blanks ");
    println_on_bg!("{:-^113}", "-");
    for (i, fillable) in fillables.iter().enumerate() {
      println_on_bg!(
        "{: ^10} | {: <100}",
        i+1,
        &fillable.display_to_user_empty(),
      );
    }
    println_on_bg!("{:-^113}", "-");
    
    println_inst!("Select type of blank by ID.");
    println_inst!(
      "| {}",
      "CANCEL / C: Cancel",
    );
  }
  fn get_fillable_blank() -> Option<Blank> {
    loop {
      NoteArchive::display_blank_menus();
      let mut buffer = String::new();
      let idx_attempt = io::stdin().read_line(&mut buffer);
      let idx = match idx_attempt {
        Ok(_) => buffer.trim().to_ascii_lowercase(),
        Err(e) => {
          println_err!("Failed to read line: {}", e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      };
      match &idx[..] {
        "cancel" | "c" => {
          return None;
        },
        _ => {
          match idx.parse::<usize>() {
            Err(e) => {
              println_err!("Invalid input: '{}' ({})", idx, e);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            },
            Ok(num) => {
              match Blank::vec_of_fillables().iter().nth(num-1) {
                None => {
                  println_err!("Please choose from among the displayed options.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                },
                Some(blank_ref) => return Some(blank_ref.to_owned()),
              }
            }
          }
        }
      }
    }
  }
  fn get_blank_from_menu() -> Option<(Blank, String)> {
    loop {
      let chosen_blank = NoteArchive::get_fillable_blank();
      let b = match chosen_blank {
        None => return None,
        Some(blank) => blank,
      };

      match b.clone() {
        InternalDocument => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(InternalDocument, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| InternalDocumentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string))
        },
        ExternalDocument => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(ExternalDocument, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| ExternalDocumentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        InternalMeeting => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(InternalMeeting, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| InternalMeetingFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        ExternalMeeting => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(ExternalMeeting, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| ExternalMeetingFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        Appearance => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(Appearance, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| AppearanceFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        SupportedParent => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(SupportedParent, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| SupportedParentFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        ParentingSkill => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(ParentingSkill, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| ParentingSkillFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        CarePlanningTopic => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(CarePlanningTopic, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| CarePlanningTopicFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        YouthTopic => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(YouthTopic, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| YouthTopicFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        ContactMethod => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(ContactMethod, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| ContactMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        ContactPurpose => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(ContactPurpose, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| ContactPurposeFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        FulfilledContactPurpose => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(FulfilledContactPurpose, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| FulfilledContactPurposeFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        Service => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(Service, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| ServiceFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        MeetingMethod => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(MeetingMethod, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| MeetingMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        SignatureMethod => {
          let mut fill_ins: Vec<usize> = vec![];
          let final_blank_string = loop {
            let blank_id = match Self::select_blank_fill_in(SignatureMethod, fill_ins.clone()) {
              Some(s) => s,
              None => {
                let fill_in_objects = fill_ins.iter().map(|fi| SignatureMethodFillIn::iterator_of_blanks().nth(*fi).unwrap() );
                let fill_in_strings = fill_in_objects.map(|fo| fo.selected_display() ).collect::<Vec<String>>();
                break if fill_in_strings.len() > 1 {
                  format!(
                    "{}{}{}",
                    fill_in_strings[..fill_in_strings.len()-1].join(", "),
                    " and ",
                    fill_in_strings[fill_in_strings.len()-1],
                  )
                } else if fill_in_strings.len() > 0 {
                  fill_in_strings[0].clone()
                } else {
                  return None;
                };
              }
            };
            if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
              fill_ins.push(blank_id);
            } else {
              fill_ins.retain(|fi| fi != &blank_id )
            }
          };
          return Some((b, final_blank_string));
        },
        CustomBlank => {
          let final_string = loop {
            println!("Enter text exactly as you would like it to appear.");
            let mut input = String::new();
            let input_attempt = io::stdin().read_line(&mut input);
            match input_attempt {
              Ok(_) => break String::from(&input[..input.len()-2]),
              Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
              }
            }
          };
          return Some((b, final_string));
        },
        _ => panic!("Blank passed as fillable blank to fn 'get_blank_from_menu' but blank not listed as options in function."),
      }
    }
  }
  fn create_note_manually_get_id(&mut self) -> Option<u32> {

    let ncat = loop {
      match self.choose_note_category() {
        Some(nc) => break nc,
        None => {
          let mut choice = String::new();
          println_yel!("In order to enter a note manually, you must select a category.");
          println_inst!("Do you wish to cancel writing this note?");
          let choice_att = io::stdin().read_line(&mut choice);
          match choice_att {
            Ok(_) => {
              match &choice.trim().to_ascii_lowercase()[..] {
                "y" | "yes" => return None,
                "no" | "n" => continue,
                _ => {
                  println_err!("Invalid entry. Please select a template or cancel to return to the previous menu.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              }
            },
            Err(e) => {
              println_err!("Failed to read line: {}", e);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        }
      }
    };

    let date = self.current_note_day().date.clone();
    
    let mut n = match self.generate_note(date, ncat, CustomStructure, String::new()) {
      Ok(nopt) => nopt,
      Err(e) => panic!("Failed to generate Note with custom structure and '{}' as note category: {}.", format!("{}", ncat), e),
    };
    
    // add blanks
    let mut current_blank = 1;
    let mut hide = false;
    let mut youth_added = false;
    let mut hide_inst = false;
    loop {
      // prepare current client collaterals to display for entering into Note
      // inside loop because it is possible to add a collateral
      let collats = self.current_client_collaterals().iter().cloned().cloned().collect::<Vec<Collateral>>();
      let mut collats_iter = collats.iter();
      let mut col_rows: Vec<Vec<Collateral>> = vec![];
      'adding_rows: loop {
        let mut new_vec: Vec<Collateral> = vec![];
        for _ in 1..=4 {
          match collats_iter.next() {
            Some(col) => new_vec.push(col.to_owned()),
            None => {
              col_rows.push(new_vec);
              break 'adding_rows;
            }
          }
        }

        col_rows.push(new_vec.clone());
      }
      n.blanks = self.autofill_note_blanks(n.clone()).blanks.clone();
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      n.display_content(Some(0), None);
      if !hide {
        println_on_bg!("{:-^163}", " Collaterals ");
        for c_row in col_rows.clone() {
          let val1: String = match c_row.get(0) {
            Some(v) => format!("[{}] {}", v.id, v.full_name()),
            None => String::new(),
          };
          let val2: String = match c_row.get(1) {
            Some(v) => format!("[{}] {}", v.id, v.full_name()),
            None => String::new(),
          };
          let val3: String = match c_row.get(2) {
            Some(v) => format!("[{}] {}", v.id, v.full_name()),
            None => String::new(),
          };
          let val4: String = match c_row.get(3) {
            Some(v) => format!("[{}] {}", v.id, v.full_name()),
            None => String::new(),
          };
          println_on_bg!(
            "{:-^38} | {:-^38} | {:-^38} | {:-^38}",
            val1,
            val2,
            val3,
            val4,
          );
        }
        println_on_bg!("{:-^163}", "-");

      }
      println_inst!("Enter text to add to note.");
      println_inst!("Press ENTER to choose a phrase from the saved menus.");
      if !hide_inst {
        println_inst!(
          "| {} | {} | {}",
          "YOUTH / Y: Youth's full name",
          "ALL / A: All collaterals",
          "CPT: All Care Plan Team members",
        );
        println_inst!(
          "| {} | {} | {}",
          "GOAL / G: Client goal",
          "TODAY / T: Today's date",
          "NOTEDAY / ND: The date of the current note",
        );
        println_inst!(
          "| {} | {} | {}",
          "SAVE / S: Finish writing and save to current record",
          "BACK / B: Delete last word",
          "CANCEL / C: Cancel and discard",
        );
        println_inst!(
          "| {}",
          "GENERAL: Choose from general/universal collaterals (E.g. intake coordinators, insurance contacts, office managers extraordinaire.)",
        );
        if hide {
          println_inst!(
            "| {} | {}",
            "SHOW / SH: Show collaterals list",
            "INST / I: Hide instructions",
          );
        } else {
          println_inst!(
            "| {} | {}",
            "HIDE / H: Hide collaterals list",
            "INST / I: Hide instructions",
          );
        }
      } else {
        println_inst!(
          "| {}",
          "INST / I: Show instructions",
        );
      }
      
      let mut choice = String::new();
      let choice_att = io::stdin().read_line(&mut choice);
      match choice_att {
        Ok(_) => {
          match &choice.trim().to_ascii_lowercase()[..] {
            "" => {
              let b_opt = NoteArchive::get_blank_from_menu();
              let (blank, blank_fill) = match b_opt {
                Some((b, bf)) => (b, bf),
                None => continue,
              };
              if n.content.chars().count() == 0 {
                n.blanks.insert(current_blank, (blank, make_ascii_titlecase(blank_fill), vec![]));
                n.content.push_str(&format!("{}", blank));
              } else if n.content.chars().count() == 1 {
                match &n.content[n.content.len()-1..] {
                  " " => {
                    n.blanks.insert(current_blank, (blank, make_ascii_titlecase(blank_fill), vec![]));
                    n.content = String::new();
                    n.content.push_str(&format!("{}", blank));
                  },
                  _ => {
                    n.blanks.insert(current_blank, (blank, make_ascii_titlecase(blank_fill), vec![]));
                    n.content.push_str(&format!(" {}", blank));
                  },
                }
              } else {
                match &n.content[n.content.len()-2..] {
                  ". " => {
                    n.blanks.insert(current_blank, (blank, make_ascii_titlecase(blank_fill), vec![]));
                    n.content.push_str(&format!("{}", blank));
                  },
                  _ => match &n.content[n.content.len()-1..] {
                    " " => {
                      n.blanks.insert(current_blank, (blank, blank_fill, vec![]));
                      n.content.push_str(&format!("{}", blank));
                    },
                    "." => {
                      n.blanks.insert(current_blank, (blank, make_ascii_titlecase(blank_fill), vec![]));
                      n.content.push_str(&format!(" {}", blank));
                    },
                    _ => {
                      n.blanks.insert(current_blank, (blank, blank_fill, vec![]));
                      n.content.push_str(&format!(" {}", blank));
                    }
                  }
                }
              }
              current_blank += 1;
            },
            "hide" | "h" => {
              hide = true;
              continue;
            }
            "show" | "sh" => {
              hide = false;
              continue;
            },
            "instructions" | "inst" | "i" => {
              if hide_inst {
                hide_inst = false;
              } else {
                hide_inst = true;
              }
            }
            "youth" | "y" | "YOUTH" | "Youth" | "Y" => {
              let current_client_string = if !youth_added {
                youth_added = true;
                self.current_client().full_name_with_label()
              } else {
                self.current_client().full_name()
              };
              n.add_blank(CurrentClientName);
              n.blanks.insert(current_blank, (CurrentClientName, current_client_string, vec![]));
              current_blank += 1;
              continue;
            },
            "goal" | "g" => {
              let mut fill_ins: Vec<u32> = vec![];
              let final_blank_string = loop {
                let blank_id = match self.select_client_goals(Some(fill_ins.clone())) {
                  Some(s) => s,
                  None => {
                    let fill_in_strings = fill_ins.iter().map(|g_id| self.get_goal_by_id(*g_id).unwrap().goal.clone() ).collect::<Vec<String>>();
                    break if fill_in_strings.len() > 1 {
                      format!(
                        "{}{}{}",
                        fill_in_strings[..fill_in_strings.len()-1].join(", "),
                        " and ",
                        fill_in_strings[fill_in_strings.len()-1],
                      )
                    } else if fill_in_strings.len() > 0 {
                      fill_in_strings[0].clone()
                    } else {
                      String::new()
                    };
                  }
                };
                if !fill_ins.clone().iter().any(|fi| fi == &blank_id ) {
                  fill_ins.push(blank_id);
                } else {
                  fill_ins.retain(|fi| fi != &blank_id )
                }
              };
              if fill_ins.len() > 0 {
                n.add_blank(ClientGoal);
                n.blanks.insert(current_blank, (ClientGoal, final_blank_string, fill_ins));
                current_blank += 1;
              }
              continue;
            }
            "noteday" | "nd" => {
              let nd = self.current_note_day();
              n.add_blank(NoteDayDate);
              n.blanks.insert(current_blank, (NoteDayDate, nd.fmt_date_long(), vec![nd.id]));
              current_blank += 1;
              continue;
            },
            "today" | "t" => {
              let t = Local::now().naive_local().date();
              let nd = NoteDay::new(1, t, 1, 1, vec![]); // just for the fmtdate method below
              n.add_blank(TodayDate);
              n.blanks.insert(current_blank, (TodayDate, nd.fmt_date_long(), vec![]));
              current_blank += 1;
              continue;
            }
            "all" | "a" => {
              let current_collaterals = self.current_client_collaterals();
              let num_collats = current_collaterals.len();
              let all_collaterals_string = if num_collats == 0 {
                println_err!("No collaterals are saved for the current client.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              } else if num_collats == 1 {
                current_collaterals[0].full_name_and_title()
              } else if num_collats > 1 {
                let part1 = current_collaterals[..num_collats-1].to_owned().iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", ");
                let part2 = current_collaterals[num_collats-1].full_name_and_title();
                format!("{} and {}", part1, part2)
              } else {
                // else condition is impossible because vec must have positive length
                String::new()
              };
              n.add_blank(AllCollaterals);
              n.blanks.insert(current_blank, (AllCollaterals, all_collaterals_string, vec![]));
              current_blank += 1;
              continue;
            },
            "cpt" | "care plan team" | "care plan" => {
              let cpt = self.current_client_collaterals().iter().filter(|co| co.care_plan_team ).map(|co_ref| co_ref.to_owned().to_owned() ).collect::<Vec<Collateral>>();
              let cpt_len = cpt.len();
              if cpt_len > 0 {
                let blank_string = if cpt_len > 1 {
                  format!(
                    "{} {} {}",
                    cpt[..cpt_len-1].iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", "),
                    "and",
                    cpt[cpt_len-1].full_name_and_title(),
                  )
                } else {
                  cpt[0].full_name_and_title()
                };
                let id_vec = cpt.iter().map(|co| co.id ).collect::<Vec<u32>>();
                n.add_blank(CarePlanTeam);
                n.blanks.insert(current_blank, (CarePlanTeam, blank_string, id_vec.clone()));
                current_blank += 1;
                let mut old_ids = n.foreign_keys["collateral_ids"].clone();
                for new_id in id_vec {
                  if !old_ids.clone().iter().any(|o_id| o_id == &new_id ) {
                    old_ids.push(new_id);
                  }
                }
                n.foreign_keys.insert(String::from("collateral_ids"), old_ids);
              } else {
                if self.current_client_collaterals().len() == 0 {
                  println_err!("No collaterals are saved for the current client.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
                println_inst!("No Care Plan Team members found for current client.");
                println_inst!("Press ENTER to choose or edit collaterals on the next menu.");
                let mut s = String::new();
                let input_attempt = io::stdin().read_line(&mut s);
                match input_attempt {
                  _ => (),
                }
                let (blank_string, id_vec) = self.select_collaterals();
                n.add_blank(CarePlanTeam);
                n.blanks.insert(current_blank, (CarePlanTeam, blank_string, id_vec.clone()));
                current_blank += 1;
                n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
              }
            }
            "general" => {
              let mut general_collats: Vec<Collateral> = vec![];
              loop {
                let general_collat_ids = general_collats.iter().map(|co| co.id ).collect::<Vec<u32>>();
                let general_input = loop {
                  self.display_select_general_collaterals(Some(general_collat_ids.clone()));
                  println_inst!("ALL: Select all");
                  let mut choice = String::new();
                  let read_attempt = io::stdin().read_line(&mut choice);
                  match read_attempt {
                    Ok(_) => break choice.trim().to_string(),
                    Err(e) => {
                      println_err!("Could not read input; try again ({}).", e);
                      continue;
                    }
                  }
                };
                match &general_input.to_ascii_lowercase()[..] {
                  "new" | "n" => {
                    let maybe_new_id = self.create_general_collateral_get_id();
                    match maybe_new_id {
                      Some(_) => (),
                      None => (),
                    }
                    continue;
                  },
                  "edit" | "e" => {
                    self.choose_edit_general_collaterals();
                    continue;
                  },
                  "all" | "a" => {
                    general_collats = self.general_collaterals.clone();
                    break;
                  },
                  "quit" | "q" => {
                    break;
                  },
                  "" => {
                    break;
                  }
                  _ => match general_input.parse() {
                    Ok(num) => {
                      let collat = match self.general_collaterals.iter().find(|co| co.id == num) {
                        Some(co) => co.clone(),
                        None => continue,
                      };
                      if !general_collats.iter().any(|co| co == &collat ) {
                        general_collats.push(collat);
                        if general_collats.len() == self.general_collaterals.len() {
                          break;
                        }
                      } else {
                        general_collats.retain(|co| co != &collat );
                      }
                    },
                    Err(e) => {
                      println_err!("Invalid input: {}; error: {}", general_input, e);
                      thread::sleep(time::Duration::from_secs(3));
                      continue;
                    }
                  }
                }
              }
              let num_collats = general_collats.len();
              let general_collaterals_string = if num_collats == 0 {
                println_err!("No collaterals are saved for the current client.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              } else if num_collats == 1 {
                general_collats[0].full_name_and_title()
              } else if num_collats > 1 {
                let part1 = general_collats[..num_collats-1].to_owned().iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", ");
                let part2 = general_collats[num_collats-1].full_name_and_title();
                format!("{} and {}", part1, part2)
              } else {
                // else condition is impossible because vec must have positive length
                String::new()
              };
              n.add_blank(Collaterals);
              n.blanks.insert(current_blank, (Collaterals, general_collaterals_string, vec![]));
              current_blank += 1;
              continue;
            }
            "back" | "b" => {
              n.content = n.content.trim().to_string();
              let last_space = n.content.rfind(' ');
              match last_space {
                None => {
                  if n.content.chars().count() > 5 {
                    lazy_static! {
                      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                    }
                    let last_m = RE_BLANK.find_iter(&n.content).last();
                    let end_index: usize = match last_m {
                      Some(m) => {
                        if m.end() == n.content.len() {
                          m.start()
                        } else {
                          m.end()
                        }
                      }
                      None => n.content.chars().count() - 5,
                    };
                    let num_blanks = RE_BLANK.find_iter(&n.content[end_index..]).count();
                    current_blank -= num_blanks as u32;
                    n.remove_blanks_after_content_index(end_index);
                    youth_added = n.blank_contains_string(self.current_client().full_name_with_label());
                    n.content = String::from(&n.content[..end_index]);
                  } else {
                    current_blank = 1;
                    youth_added = false;
                    n.blanks.clear();
                    n.content = String::new();
                  }
                },
                Some(idx) => {
                  lazy_static! {
                    static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
                  }
                  let num_blanks = RE_BLANK.find_iter(&n.content[idx..]).count();
                  current_blank -= num_blanks as u32;
                  n.remove_blanks_after_content_index(idx);
                  youth_added = n.blank_contains_string(self.current_client().full_name_with_label());
                  n.content = n.content[..idx].to_string();
                },
              }
              let mut new_ids = n.foreign_keys["collateral_ids"].clone();
              new_ids.retain(|co_id|
                n.blanks.values()
                  .filter(|(b, _s, _ids)| *b == AllCollaterals || *b == Collaterals )
                  .any(|(_, _, ids)| ids.iter().any(|id| id == co_id ) )
              );
              n.foreign_keys.insert(
                String::from("collateral_ids"),
                new_ids
              );
              continue;
            },
            "save" | "s" | "SAVE" | "Save" | "S" => {
              let note_id = n.id;
              self.save_note(n);
              return Some(note_id)
            },
            "cancel" | "c" | "CANCEL" | "Cancel" | "C" => return None,
            _ => {
              match &choice.trim().parse::<u32>() {
                Ok(num) => {
                  if !self.current_client_collaterals().iter().any(|col| col.id == *num ) {
                    ()
                  } else {
                    match self.get_collateral_by_id(*num) {
                      Some(collat) => {
                        let collateral_display_string = collat.full_name_and_title();
                        if n.content.trim_end() != n.content {
                          n.add_blank(Collaterals);
                          n.blanks.insert(current_blank, (Collaterals, collateral_display_string, vec![]));
                          current_blank += 1;
                        } else {
                          n.content.push_str(" ");
                          n.add_blank(Collaterals);
                          n.blanks.insert(current_blank, (Collaterals, collateral_display_string, vec![]));
                          current_blank += 1;
                        }
                        continue;
                      },
                      None => (),
                    }
                  }
                },
                Err(_) => {
                  ()
                }
              }
              if !choice.contains("(---") && !choice.contains("---)") {
                let mut apostrophe = false;
                if choice.trim().chars().count() >= 2 {
                  if choice.trim().chars().count() >= 3 {
                    if &choice[..3] == "'s " {
                      apostrophe = true;
                    }
                  } else {
                    if &choice[..2] == "'s" {
                      apostrophe = true;
                    }
                  }
                }
                let mut ends_with_punctuation = false;
                match n.content.chars().last() {
                  Some(c) => {
                    if String::from("'\"@#$^*`]})<-_?!/.").contains(c) {
                      ends_with_punctuation = true;
                    }
                  }
                  None => (),
                }
                let mut adding_punctuation = false;
                if choice.trim().len() >= 1 {
                  match &choice[..1] {
                    "." | "!" | "?" | "/" | ")" | "}" | "]" | ":" | ";" | "," => adding_punctuation = true,
                    _ => (),
                  }
                }
                if n.content.trim_end() != n.content
                || String::new() == n.content
                || !ends_with_punctuation
                || adding_punctuation
                || apostrophe {
                  n.content.push_str(&choice[..choice.len()-2]);
                } else {
                  n.content.push_str(&format!("{}{}", " ", choice.trim())[..]);
                }
              } else {
                println_err!("Invalid character series in string.");
                thread::sleep(time::Duration::from_secs(2));
              }
              continue;
            }
          }
        },
        Err(e) => {
          println_err!("Failed to read line: {}", e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      }
    }

  }
  fn create_note_get_id(&mut self, nt_id: Option<u32>) -> Option<u32> {
    // first check if there was a nt_id passed and create from that template
    match nt_id {
      None => (),
      Some(id) => return self.create_note_from_template_get_id(Some(id)),
    }
    loop {
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      println_inst!(
        "| {} | {} | {} ",
        "TEMPLATE / T: build from template",
        "SCRATCH / S: build from scratch",
        "CANCEL / C: cancel",
      );
      let mut input = String::new();
      let input_attempt = io::stdin().read_line(&mut input);
      let input_val = match input_attempt {
        Ok(_) => input.trim().to_ascii_lowercase(),
        Err(e) => {
          println_err!("Failed to read line: {}", e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      };
      match &input_val[..] {
        "template" | "t" => return self.create_note_from_template_get_id(None),
        "scratch" | "s" => return self.create_note_manually_get_id(),
        "cancel" => return None,
        _ => {
          println_err!("Invalid input: {}", input_val);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      }
    }
  }
  fn generate_note(
    &mut self,
    date: NaiveDate,
    category: NoteCategory,
    structure: StructureType,
    content: String,
  ) -> Result<Note, String> {
    let id: u32 = self.notes.len() as u32 + 1;
    let user_id = self.current_user().id;
    let client_id = match self.foreign_key.get("current_client_id") {
      None => self.select_client(),
      Some(n) => *n,
    };
    match self.load_client(client_id) {
      Err(_) => panic!("Failed to load client selected for note."),
      Ok(_) => (),
    }
    let collateral_ids: Vec<u32> = vec![];

    Ok(Note::new(id, date, category, structure, content, user_id, client_id, collateral_ids))
  }
  pub fn read_notes(filepath: &str) -> Result<Vec<Note>, Error> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .unwrap();

    let reader = BufReader::new(file);

    let mut lines: Vec<std::io::Result<String>> = reader.lines().collect();

    if lines.len() > 0 {
      lines.remove(0)?;
    }
    if lines.len() > 0 {
      lines.remove(lines.len() - 1)?;
    }

    let mut notes: Vec<Note> = vec![];

    for line in lines {
      let line_string = line?;

      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let id: u32 = values[0].parse().unwrap();

      let date_vec: Vec<i32> = match &values[1][..] {
        "" => vec![],
        _ => values[1]
        .split("-")
          .map(|val| val.parse().unwrap())
          .collect(),
      };

      let (year, month, day): (i32, u32, u32) = (date_vec[0], date_vec[1] as u32, date_vec[2] as u32);
      let date = NaiveDate::from_ymd(year, month, day);

      let category_strings: Vec<String> = values[2].split(" - ").map(|s| s.to_string()).collect();
      let (category_string, subcategory_string) = (category_strings[0].clone(), category_strings[1].clone());
      
      let category = match &category_string[..] {
        "ICC Note" => {
          let subcategory = match &subcategory_string[..] {
            "Face to face contact with client" => FaceToFaceContactWithClient,
            "Telephone contact with client" => TelephoneContactWithClient,
            "Care coordination" => CareCoordination,
            "Documentation" => Documentation,
            "Care planning team" => CarePlanningTeam,
            "Transport client" => TransportClient,
            "Member outreach/no-show" => MemberOutreachNoShow,
            _ => return Err(Error::new(
              ErrorKind::Other,
              "Unsupported ICC note subcategory saved to file.",
            )),
          };
          ICCNote(subcategory)
        },
        "FP Note" => {
          let subcategory = match &subcategory_string[..] {
            "Functioning" => Functioning,
            "Plan/additional information" => PlanAdditionalInformation,
            "Description of 'face to face contact'" => DescriptionOfIntervention(Some(FaceToFaceContact)),
            "Description of 'collateral contact'" => DescriptionOfIntervention(Some(CollateralContact)),
            "Description of 'crisis support'" => DescriptionOfIntervention(Some(CrisisSupport)),
            "Description of 'telephone support'" => DescriptionOfIntervention(Some(TelephoneSupport)),
            "Description of 'direct time with providers'" => DescriptionOfIntervention(Some(DirectTimeWithProviders)),
            "Description of 'educationg, coaching, modeling and guiding'" => DescriptionOfIntervention(Some(EducatingCoachingModelingAndGuiding)),
            "Description of 'engage parent/caregiver in addressing goals'" => DescriptionOfIntervention(Some(EngageParentCaregiverInAddressingGoals)),
            "Description of 'teach advocacy, guide linkage to resources'" => DescriptionOfIntervention(Some(TeachAdvocacyGuideLinkageToResources)),
            "Description of 'teach networking in community and with providers'" => DescriptionOfIntervention(Some(TeachNetworkingInCommunityAndWithProviders)),
            "Description of 'provider outreach to person'" => DescriptionOfIntervention(Some(ProviderOutreachToPerson)),
            "Description of 'member transportation by staff'" => DescriptionOfIntervention(Some(MemberTransportationByStaff)),
            "Description of 'no show/late cancellation'" => DescriptionOfIntervention(Some(NoShowLateCancellation)),
            "Description of 'documentation'" => DescriptionOfIntervention(Some(FPInterventionDocumentation)),
            "Description of 'other'" => DescriptionOfIntervention(Some(Other)),
            "Response to 'face to face contact'" => ResponseToIntervention(Some(FaceToFaceContact)),
            "Response to 'collateral contact'" => ResponseToIntervention(Some(CollateralContact)),
            "Response to 'crisis support'" => ResponseToIntervention(Some(CrisisSupport)),
            "Response to 'telephone support'" => ResponseToIntervention(Some(TelephoneSupport)),
            "Response to 'direct time with providers'" => ResponseToIntervention(Some(DirectTimeWithProviders)),
            "Response to 'educating, coaching, modeling and guiding'" => ResponseToIntervention(Some(EducatingCoachingModelingAndGuiding)),
            "Response to 'engage parent/caregiver in addressing goals'" => ResponseToIntervention(Some(EngageParentCaregiverInAddressingGoals)),
            "Response to 'teach advocacy, guide linkage to resources'" => ResponseToIntervention(Some(TeachAdvocacyGuideLinkageToResources)),
            "Response to 'teach networking in community and with providers'" => ResponseToIntervention(Some(TeachNetworkingInCommunityAndWithProviders)),
            "Response to 'provider outreach to person'" => ResponseToIntervention(Some(ProviderOutreachToPerson)),
            "Response to 'member transportation by staff'" => ResponseToIntervention(Some(MemberTransportationByStaff)),
            "Response to 'no show/late cancellation'" => ResponseToIntervention(Some(NoShowLateCancellation)),
            "Response to 'documentation" => ResponseToIntervention(Some(FPInterventionDocumentation)),
            "Response to 'other'" => ResponseToIntervention(Some(Other)),
            _ => return Err(Error::new(
              ErrorKind::Other,
              "Unsupported FP note subcategory saved to file.",
            )),
          };
          FPNote(subcategory)
        },
        _ => return Err(Error::new(
          ErrorKind::Other,
          "Unsupported note category saved to file.",
        )),
      };

      let structure = match &values[3][..] {
        "Care Plan" => CarePlan,
        "Intake" => Intake,
        "Assessment" => Assessment,
        "SNCD" => Sncd,
        "Home Visit" => HomeVisit,
        "Agenda Prep" => AgendaPrep,
        "Debrief" => Debrief,
        "Phone Call" => PhoneCall,
        "Scheduling" => Scheduling,
        "Sent Email" => SentEmail,
        "Referral" => Referral,
        "Custom Structure" => CustomStructure,
        "Parent Support" => ParentSupport,
        "Sent Cancellation" => SentCancellation,
        "Parent Appearance" => ParentAppearance,
        "Parent Skills" => ParentSkills,
        "Failed Contact Attempt" => FailedContactAttempt,
        "Categorized Emails" => CategorizedEmails,
        "Authorization Requested" => AuthorizationRequested,
        "Authorization Issued" => AuthorizationIssued,
        "Collateral Outreach" => CollateralOutreach,
        "Update From Collateral" => UpdateFromCollateral,
        "Invited To Meeting" => InvitedToMeeting,
        "Sent Document" => SentDocument,
        "Updated Document" => UpdatedDocument,
        "Discuss Communication" => DiscussCommunication,
        "Received Verbal Consent" => ReceivedVerbalConsent,
        "Received Written Consent" => ReceivedWrittenConsent,
        "Documentation" => DocumentationStructure,
        "Brainstorm Contribution" => BrainstormContribution,
        _ => return Err(Error::new(
          ErrorKind::Other,
          "Unsupported StructureType saved to file.",
        )),
      };

      let content = values[4].clone();

      let blanks_strings: Vec<String> = values[5].split("/#/").map(|s| s.to_string() ).collect();
      let mut blanks: HashMap<u32, (Blank, String, Vec<u32>)> = HashMap::new();

        // pub blanks: HashMap<u32, (Blank, String, Vec<u32>)> 

      for b_string in blanks_strings {
        if b_string != String::from("") {
          let blank_values: Vec<String> = b_string.split("/%/").map(|st| st.to_string() ).collect();
  
          let blank_position: u32 = blank_values[0].parse().unwrap();
          let blank = Blank::get_blank_from_str(&blank_values[1]);
          let blank_content = blank_values[2].clone();
          let blank_foreign_keys: Vec<u32> = blank_values[3]
            .split('-')
            .map(|b_id_string| b_id_string.parse() )
            .filter(|b_id_res| b_id_res.is_ok() )
            .map(|b_id_res| b_id_res.unwrap() )
            .collect();
  
          blanks.insert(blank_position, (blank, blank_content, blank_foreign_keys));
        }
      }

      let note_user_id: u32 = values[6].parse().unwrap();
      let note_client_id: u32 = values[7].parse().unwrap();

      let collateral_ids: Vec<u32> = match &values[8][..] {
        "" => vec![],
        _ => values[8]
          .split("#")
          .map(|val| val.parse().unwrap())
          .collect(),
      };

      let mut n = Note::new(id, date, category, structure, content, note_user_id, note_client_id, collateral_ids);
      n.blanks = blanks;

      notes.push(n);
    }
    notes.sort_by(|a, b| a.id.cmp(&b.id));
    notes.sort_by(|a, b|
      a.foreign_key["user_id"].cmp(&b.foreign_key["user_id"])
    );
    notes.sort_by(|a, b|
      a.foreign_key["client_id"].cmp(&b.foreign_key["client_id"])
    );
    notes.sort_by(|a, b| a.date.cmp(&b.date) );
    Ok(notes)
  }
  pub fn write_notes(&self) -> std::io::Result<()> {
    let mut lines = String::from("##### notes #####\n");
    for n in &self.notes {
      lines.push_str(&n.to_string()[..]);
    }
    lines.push_str("##### notes #####");
    let mut file = File::create(self.filepaths["note_filepath"].clone()).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
    Ok(())
  }
  fn save_note(&mut self, note: Note) {

    let pos = self.notes.binary_search_by(|n| n.id.cmp(&note.id) ).unwrap_or_else(|e| e);

    let mut saved_nd_ids: Vec<u32> = self.current_note_day().foreign_keys["note_ids"].clone();
    saved_nd_ids.push(note.id);
    
    self.current_note_day_mut().foreign_keys.insert(String::from("note_ids"), saved_nd_ids);
    
    self.notes.insert(pos, note);
    self.write_to_files();
  }
  fn current_user_notes(&self) -> Vec<&Note> {
    self.notes.iter().filter(|n| n.foreign_key["user_id"] == self.current_user().id).collect()
  }
  // fn current_user_notes_mut(&mut self) -> Vec<&mut Note> {
  //   let current_id = self.current_user().id;
  //   self.notes.iter_mut().filter(|n| n.foreign_key["user_id"] == current_id).collect()
  // }
  fn choose_delete_note(&mut self) {
    loop {
      self.display_delete_note();
      println_yel!("Are you sure you want to delete this note?");
      println_inst!("| {} | {}", "YES / Y: confirm", "Any other key to cancel");
      let mut confirm = String::new();
      let input_attempt = io::stdin().read_line(&mut confirm);
      let command = match input_attempt {
        Ok(_) => confirm.trim().to_string(),
        Err(e) => {
          println_err!("Failed to read input: {}", e);
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &command[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          self.delete_current_note();
          let new_note_id = self.reindex_notes(None);
          match new_note_id {
            _ => (),
          }
          self.write_to_files();
          break;
        }
        _ => {
          break;
        }
      }
    }
  }
  fn display_delete_note(&self) {
    let heading = String::from(" DELETE NOTE ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^163}", "-");
    println_on_bg!("{:-^163}", heading);
    println_on_bg!("{:-^163}", "-");

    // the length of each line is 163
    self.current_note().display_content(None, None);

    println_on_bg!("{:-^163}", "-");
  }
  fn delete_current_note(&mut self) {
    let id = self.foreign_key.get("current_note_id").unwrap();
    self.notes.retain(|n| n.id != *id);
    let nd = self.current_note_day();
    let mut new_ids = nd.foreign_keys["note_ids"].clone();
    new_ids.retain(|n_id| n_id != id );
    self.current_note_day_mut().foreign_keys.insert(String::from("note_ids"), new_ids);
    self.foreign_key.remove("current_note_id");
  }
  fn reindex_notes(&mut self, current_id: Option<u32>) -> Option<u32> {
    let mut i: u32 = 1;
    let mut new_current_id: Option<u32> = None;
    let mut changes: HashMap<u32, u32> = HashMap::new();
    for mut n in &mut self.notes {
      match current_id {
        Some(id) => {
          if n.id == id {
            new_current_id = Some(i);
          }
        },
        None => (),
      }
      if n.id == i {
        ()
      } else {
        changes.insert(n.id, i);
        n.id = i;
      }
      i += 1;
    }
    for nd in &mut self.note_days {
      let old_ids = nd.foreign_keys["note_ids"].clone();
      let new_ids: Vec<u32> = old_ids.iter()
        .map(|co_id| match changes.get(co_id) { Some(val) => *val, None => *co_id } )
        .collect();
      nd.foreign_keys.insert(String::from("note_ids"), new_ids);
    }
    new_current_id
  }
  fn get_note_option_by_id(&self, id: u32) -> Option<&Note> {
    self.notes.iter().find(|n| n.id == id)
  }
  // fn get_note_option_by_id_mut(&mut self, id: u32) -> Option<&mut Note> {
  //   self.notes.iter_mut().find(|n| n.id == id)
  // }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_open_blank_files() {
    {
      let filepaths: HashMap<String, String> = [
        (String::from("user_filepath"), String::from("some_random_blank_user_file_name.txt"),),
        (String::from("client_filepath"), String::from("some_random_blank_client_file_name.txt"),),
        (String::from("goal_filepath"), String::from("some_random_blank_goal_file_name.txt"),),
        (String::from("collateral_filepath"), String::from("some_random_blank_collateral_file_name.txt"),),
        (String::from("general_collateral_filepath"), String::from("some_random_blank_general_collateral_file_name.txt"),),
        (String::from("pronouns_filepath"), String::from("some_random_blank_pronouns_file_name.txt"),),
        (String::from("note_day_filepath"), String::from("some_random_blank_note_day_file_name.txt"),),
        (String::from("note_template_filepath"), String::from("some_random_blank_note_template_file_name.txt"),),
        (String::from("note_filepath"), String::from("some_random_blank_note_file_name.txt"),),
      ].iter().cloned().collect();
      let a = NoteArchive::new(filepaths);
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
    fs::remove_file("some_random_blank_collateral_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_general_collateral_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_goal_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_pronouns_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_note_day_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_note_template_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_note_file_name.txt").unwrap();
  }
  #[test]
  fn can_load_from_files() {
    {
      let test_user = User::new(
        1,
        String::from("Bob"),
        String::from("Smith"),
        Icc,
        1,
        vec![1, 2, 3],
        vec![],
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
        let filepaths: HashMap<String, String> = [
          (String::from("user_filepath"), String::from("test_load_user.txt"),),
          (String::from("client_filepath"), String::from("test_load_client.txt"),),
          (String::from("goal_filepath"), String::from("test_load_goal.txt"),),
          (String::from("collateral_filepath"), String::from("test_load_collateral.txt"),),
          (String::from("general_collateral_filepath"), String::from("test_load_general_collateral.txt"),),
          (String::from("pronouns_filepath"), String::from("test_load_pronouns.txt"),),
          (String::from("note_day_filepath"), String::from("test_load_note_days.txt"),),
          (String::from("note_template_filepath"), String::from("test_load_note_templates.txt"),),
          (String::from("note_filepath"), String::from("test_load_note.txt"),),
        ].iter().cloned().collect();
      let mut a1 = NoteArchive::new(filepaths);

      a1.users = vec![test_user];
      a1.clients = vec![test_client];
      a1.pronouns = vec![test_pronouns];

      a1.write_to_files();

      a1.load_user(1).unwrap();
      let some_id: &u32 = &1;
      assert_eq!(a1.foreign_key.get("current_user_id"), Some(some_id));
    }
    fs::remove_file("test_load_user.txt").unwrap();
    fs::remove_file("test_load_client.txt").unwrap();
    fs::remove_file("test_load_collateral.txt").unwrap();
    fs::remove_file("test_load_general_collateral.txt").unwrap();
    fs::remove_file("test_load_goal.txt").unwrap();
    fs::remove_file("test_load_pronouns.txt").unwrap();
    fs::remove_file("test_load_note_days.txt").unwrap();
    fs::remove_file("test_load_note_templates.txt").unwrap();
    fs::remove_file("test_load_note.txt").unwrap();
  }
  #[test]
  fn creates_unique_new_instances() {
    let filepaths: HashMap<String, String> = [
      (String::from("user_filepath"), String::from("test_user_new_instance.txt"),),
      (String::from("client_filepath"), String::from("test_client_new_instance.txt"),),
      (String::from("goal_filepath"), String::from("test_goal_new_instance.txt"),),
      (String::from("collateral_filepath"), String::from("test_collateral_new_instance.txt"),),
      (String::from("general_collateral_filepath"), String::from("test_general_collateral_new_instance.txt"),),
      (String::from("pronouns_filepath"), String::from("test_pronouns_new_instance.txt"),),
      (String::from("note_day_filepath"), String::from("test_note_days_new_instance.txt"),),
      (String::from("note_template_filepath"), String::from("test_note_templates_new_instance.txt"),),
      (String::from("note_filepath"), String::from("test_note_new_instance.txt"),),
    ].iter().cloned().collect();

    let mut notes = NoteArchive::new_test(filepaths.clone());

    let new_user_attempt =
      notes.generate_unique_new_user(String::from("Carl"), String::from("Carlson"), Icc, 1);
    let new_client_attempt = notes.generate_unique_new_client(
      String::from("Carl"),
      String::from("Carlson"),
      NaiveDate::from_ymd(2008, 3, 4),
      1,
    );

    let new_pronouns_attempt = notes.generate_unique_new_pronouns(
      String::from("zey"),
      String::from("zem"),
      String::from("zeir"),
      String::from("zeirs"),
    );

    let new_user = match new_user_attempt {
      Ok(user) => user,
      Err(_) => panic!("Failed to generate user."),
    };
    let new_client = match new_client_attempt {
      Ok(client) => client,
      Err(h) => {
        let mut msg = String::new();
        for (e, i) in &h {
          msg.push_str(&format!(" {}: {}.", e, i));
        } 
        panic!(format!("Failed to generate client: {}", msg));
      }
    };
    let new_pronouns = match new_pronouns_attempt {
      Ok(pronouns) => pronouns,
      Err(e) => panic!("Failed to generate pronouns: {}", e),
    };

    assert_eq!(
      new_user,
      User::new(
        3,
        String::from("Carl"),
        String::from("Carlson"),
        Icc,
        1,
        vec![],
        vec![],
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
        4,
        String::from("zey"),
        String::from("zem"),
        String::from("zeir"),
        String::from("zeirs")
      )
    );
    for (_, v) in filepaths {
      fs::remove_file(v).unwrap();
    }
  }

  // pronouns

  #[test]
  fn gets_current_pronouns() {
    let filepaths: HashMap<String, String> = [
      (String::from("user_filepath"), String::from("test_user_current_pronouns.txt"),),
      (String::from("client_filepath"), String::from("test_client_current_pronouns.txt"),),
      (String::from("goal_filepath"), String::from("test_goal_current_pronouns.txt"),),
      (String::from("collateral_filepath"), String::from("test_collateral_current_pronouns.txt"),),
      (String::from("general_collateral_filepath"), String::from("test_glonal_collateral_current_pronouns.txt"),),
      (String::from("pronouns_filepath"), String::from("test_pronouns_current_pronouns.txt"),),
      (String::from("note_day_filepath"), String::from("test_note_days_current_pronouns.txt"),),
      (String::from("note_template_filepath"), String::from("test_note_templates_current_pronouns.txt"),),
      (String::from("note_filepath"), String::from("test_note_current_pronouns.txt"),),
    ].iter().cloned().collect();
    let mut notes = NoteArchive::new_test(filepaths.clone());

    notes.load_user(1).unwrap();

    notes.update_current_pronouns(1);

    let current_pronouns_id = notes.current_user().pronouns;

    assert_eq!(notes.get_pronouns_by_id(current_pronouns_id).unwrap().id, 1);
    for (_, v) in filepaths {
      fs::remove_file(v).unwrap();
    }
  }

  #[test]
  fn updates_current_pronouns() {
    let filepaths: HashMap<String, String> = [
      (String::from("user_filepath"), String::from("test_user_updates_pronouns.txt"),),
      (String::from("client_filepath"), String::from("test_client_updates_pronouns.txt"),),
      (String::from("goal_filepath"), String::from("test_goal_updates_pronouns.txt"),),
      (String::from("collateral_filepath"), String::from("test_collateral_updates_pronouns.txt"),),
      (String::from("general_collateral_filepath"), String::from("test_general_collateral_updates_pronouns.txt"),),
      (String::from("pronouns_filepath"), String::from("test_pronouns_updates_pronouns.txt"),),
      (String::from("note_day_filepath"), String::from("test_note_days_updates_pronouns.txt"),),
      (String::from("note_template_filepath"), String::from("test_note_templates_updates_pronouns.txt"),),
      (String::from("note_filepath"), String::from("test_note_updates_pronouns.txt"),),
    ].iter().cloned().collect();
    let mut notes = NoteArchive::new_test(filepaths.clone());

    notes.load_user(1).unwrap();

    notes.update_current_pronouns(2);
    assert_eq!(notes.current_user().pronouns, 2);

    notes.update_current_pronouns(1);
    assert_eq!(notes.current_user().pronouns, 1);

    for (_, v) in filepaths {
      fs::remove_file(v).unwrap();
    }
  }
  #[test]
  fn note_template_accurate_display_string() {
    let nt1 = NoteTemplate::new(
      1,
      CarePlan,
      true,
      format!(
        "{} is a {}. There is also a {} in {} until a {} comes along and {}, with what have you and such and such. {}, therefore, {}, and also {} is {}. How can anyone {}? What way of doing {} is there unless one {} unto the highest and utmost? {} is not as {} as {} is. Neither can {} be a {} until {} is a thing that does {} and {}.",
        CurrentUser,
        CurrentClientName,
        Collaterals,
        AllCollaterals,
        Pronoun1ForUser,
        Pronoun2ForUser,
        Pronoun3ForUser,
        Pronoun4ForUser,
        Pronoun1ForClient,
        Pronoun2ForClient,
        Pronoun3ForClient,
        Pronoun4ForClient,
        TodayDate,
        NoteDayDate,
        InternalDocument,
        ExternalDocument,
        InternalMeeting,
        ExternalMeeting,
        Appearance,
        SupportedParent,
        CustomBlank,
      ),
      vec![],
    );

    let nt2 = NoteTemplate::new(
      2,
      HomeVisit,
      true,
      format!(
        "{}'s pronouns are {}. {}'s pronouns are {}. {}'s pronouns are {}. {}'s pronouns are {}.",
        CurrentUser,
        Pronoun1ForBlank(Some(1)),
        Collaterals,
        Pronoun2ForBlank(Some(3)),
        Collaterals,
        Pronoun3ForBlank(Some(5)),
        Collaterals,
        Pronoun4ForBlank(Some(7)),
      ),
      vec![1],
    );
    
    // (String, Vec<(String, usize, usize)>)
    
    let (display_content_string1, _) = nt1.generate_display_content_string_with_blanks(None, None, None, None, None);
    assert_eq!(
      display_content_string1,
      format!(
        "{} is a {}. There is also a {} in {} until a {} comes along and {}, with what have you and such and such. {}, therefore, {}, and also {} is {}. How can anyone {}? What way of doing {} is there unless one {} unto the highest and utmost? {} is not as {} as {} is. Neither can {} be a {} until {} is a thing that does {} and {}.",
        CurrentUser.display_to_user(),
        CurrentClientName.display_to_user(),
        Collaterals.display_to_user(),
        AllCollaterals.display_to_user(),
        Pronoun1ForUser.display_to_user(),
        Pronoun2ForUser.display_to_user(),
        Pronoun3ForUser.display_to_user(),
        Pronoun4ForUser.display_to_user(),
        Pronoun1ForClient.display_to_user(),
        Pronoun2ForClient.display_to_user(),
        Pronoun3ForClient.display_to_user(),
        Pronoun4ForClient.display_to_user(),
        TodayDate.display_to_user(),
        NoteDayDate.display_to_user(),
        InternalDocument.display_to_user(),
        ExternalDocument.display_to_user(),
        InternalMeeting.display_to_user(),
        ExternalMeeting.display_to_user(),
        Appearance.display_to_user(),
        SupportedParent.display_to_user(),
        CustomBlank.display_to_user(),
      ),
    );
    
    let (display_content_string2, _) = nt2.generate_display_content_string_with_blanks(None, None, None, None, None);
    assert_eq!(
      display_content_string2,
      format!(
        "{}'s pronouns are {}. {}'s pronouns are {}. {}'s pronouns are {}. {}'s pronouns are {}.",
        CurrentUser.display_to_user(),
        Pronoun1ForBlank(Some(1)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun2ForBlank(Some(3)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun3ForBlank(Some(5)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun4ForBlank(Some(7)).display_to_user(),
      ),
    );

    let (display_content_string3, _) = nt2.generate_display_content_string_with_blanks(None, None, None, Some(1), None);
    assert_eq!(
      display_content_string3,
      format!(
        "[1]: {}'s pronouns are [2]: {}. [3]: {}'s pronouns are [4]: {}. [5]: {}'s pronouns are [6]: {}. [7]: {}'s pronouns are [8]: {}.",
        CurrentUser.display_to_user(),
        Pronoun1ForBlank(Some(1)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun2ForBlank(Some(3)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun3ForBlank(Some(5)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun4ForBlank(Some(7)).display_to_user(),
      ),
    );

    let (display_content_string4, _) = nt2.generate_display_content_string_with_blanks(None, None, None, None, Some(1));
    assert_eq!(
      format!(
        "[1]: {}[2]: 's pronouns are {}[3]: . {}[4]: 's pronouns are {}[5]: . {}[6]: 's pronouns are {}[7]: . {}[8]: 's pronouns are {}[9]: .",
        CurrentUser.display_to_user(),
        Pronoun1ForBlank(Some(1)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun2ForBlank(Some(3)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun3ForBlank(Some(5)).display_to_user(),
        Collaterals.display_to_user(),
        Pronoun4ForBlank(Some(7)).display_to_user(),
      ),
      display_content_string4,
    );

    let nt2 = NoteTemplate::new(
      2,
      Sncd,
      true,
      format!(
        "Here is a sentence. Here is another one with a blank ({}) in it. Here is another sentence that has a {}, {}, and {}.",
        CurrentUser,
        Pronoun1ForUser,
        Pronoun2ForUser,
        AllCollaterals,
      ),
      vec![],
    );
    
    let s1a = String::from("[1]: Here is a sentence. ");
    let s1b = String::from("[2]: Here is another one with a blank (");
    let s1c = format!("{}", &CurrentUser.display_to_user());
    let s1d = String::from("[3]: ) in it. ");
    let s1e = String::from("[4]: Here is another sentence that has a ");
    let s1f = format!("{}", &Pronoun1ForUser.display_to_user());
    let s1g = String::from("[5]: , ");
    let s1h = format!("{}", &Pronoun2ForUser.display_to_user());
    let s1i = String::from("[6]: , and ");
    let s1j = format!("{}", &AllCollaterals.display_to_user());
    let s1k = String::from("[7]: .");

    let check_display_string_with_content_focus = format!("{}{}{}{}{}{}{}{}{}{}{}", s1a, s1b, s1c, s1d, s1e, s1f, s1g, s1h, s1i, s1j, s1k);

    let (display_string_with_content_focus, _) = nt2.generate_display_content_string_with_blanks(None, None, None, None, Some(1));

    assert_eq!(check_display_string_with_content_focus, display_string_with_content_focus);
  }
  #[test]
  fn note_template_accurate_formatting_vector_without_focus() {
    let nt1 = NoteTemplate::new(
      1,
      CarePlan,
      true,
      format!(
        "{} is a user with {}/{} for pronouns.",
        CurrentUser,
        Pronoun1ForUser,
        Pronoun2ForUser,
      ),
      vec![],
    );

    let s1a = CurrentUser.display_to_user().chars().count();
    let s1b = String::from(" is a user with ").chars().count() + s1a;
    let s1c = Pronoun1ForUser.display_to_user().chars().count() + s1b;
    let s1d = String::from("/").chars().count() + s1c;
    let s1e = Pronoun2ForUser.display_to_user().chars().count() + s1d;
    let s1f = String::from(" for pronouns.").chars().count() + s1e + 1;

    let formatting1: Vec<(String, usize, usize)> = vec![
      (String::from("BLANK"), 0, s1a),
      (String::from("CONTENT"), s1a, s1b),
      (String::from("BLANK"), s1b, s1c),
      (String::from("CONTENT"), s1c, s1d),
      (String::from("BLANK"), s1d, s1e),
      (String::from("CONTENT"), s1e, s1f),
    ];

    // (String, Vec<(String, usize, usize)>)
    
    let (_, formatting_vector1) = nt1.generate_display_content_string_with_blanks(None, None, None, None, None);

    assert_eq!(formatting1, formatting_vector1);
  }
  #[test]
  fn note_template_gets_accurate_formatting_vec() {
    let nt2 = NoteTemplate::new(
      2,
      Sncd,
      true,
      format!(
        "Here is a sentence. Here is another one with a blank ({}) in it. Here is another sentence that has a {}, {}, and {}.",
        CurrentUser,
        Pronoun1ForUser,
        Pronoun2ForUser,
        AllCollaterals,
      ),
      vec![],
    );
    
    let (nt_display_string, formatting_vector2) = nt2.generate_display_content_string_with_blanks(None, None, None, Some(1), None);

    let s1a = String::from("Here is a sentence. ").chars().count();
    let s1b = String::from("Here is another one with a blank (").chars().count() + s1a;
    let s1c = format!("[1]: {}", &CurrentUser.display_to_user()).chars().count() + s1b;
    let s1d = String::from(") in it. ").chars().count() + s1c;
    let s1e = String::from("Here is another sentence that has a ").chars().count() + s1d;
    let s1f = format!("[2]: {}", &Pronoun1ForUser.display_to_user()).chars().count() + s1e;
    let s1g = String::from(", ").chars().count() + s1f;
    let s1h = format!("[3]: {}", &Pronoun2ForUser.display_to_user()).chars().count() + s1g;
    let s1i = String::from(", and ").chars().count() + s1h;
    let s1j = format!("[4]: {}", &AllCollaterals.display_to_user()).chars().count() + s1i;
    let s1k = String::from(".").chars().count() + s1j;

    let formatting2: Vec<(String, usize, usize)> = vec![
      (String::from("CONTENT"), 0, s1a),
      (String::from("CONTENT"), s1a, s1b),
      (String::from("HIGHLIGHTED BLANK"), s1b, s1c),
      (String::from("CONTENT"), s1c, s1d),
      (String::from("CONTENT"), s1d, s1e),
      (String::from("UNHIGHLIGHTED BLANK"), s1e, s1f),
      (String::from("CONTENT"), s1f, s1g),
      (String::from("UNHIGHLIGHTED BLANK"), s1g, s1h),
      (String::from("CONTENT"), s1h, s1i),
      (String::from("UNHIGHLIGHTED BLANK"), s1i, s1j),
      (String::from("CONTENT"), s1j, s1k + 1),
    ];

    // let blank_vec: Vec<(String, usize, usize)> = vec![];

    assert_eq!(formatting2, formatting_vector2);

    let nt_output_vecs: Vec<(usize, String, Option<Vec<(String, usize, usize)>>)> = NoteTemplate::get_display_content_vec_from_string(nt_display_string, Some(formatting_vector2));

    let n1 = Pronoun1ForUser.display_to_user().len();
    let n2 = Pronoun2ForUser.display_to_user().len();
    let n3 = AllCollaterals.display_to_user().len();

    let display_vecs: Vec<(usize, String, Option<Vec<(String, usize, usize)>>)> = vec![
      (
        0,
        String::from("Here is a sentence. "),
        Some(vec![
          (String::from("CONTENT"), 0, 20),
        ])
      ),
      (
        1,
        format!("Here is another one with a blank ([1]: {}) in it. ", CurrentUser.display_to_user()),
        Some(vec![
          (String::from("CONTENT"), 0, 34),
          (String::from("HIGHLIGHTED BLANK"), 34, CurrentUser.display_to_user().len() + 5 + 34),
          (String::from("CONTENT"), CurrentUser.display_to_user().len() + 5 + 34, CurrentUser.display_to_user().len() + 5 + 34 + 9),
        ])
      ),
      // Here is another sentence that has a [2]: Subject pronoun of the current user, [3]: Object pronoun of the current user, and [4]: All collaterals for the current client.
      // [
      //   ("CONTENT", 0, 20),
      //   ("CONTENT", 20, 54),
      //   ("HIGHLIGHTED BLANK", 54, 71),
      //   ("CONTENT", 71, 80),
      //   ("CONTENT", 80, 116),                    <--- 0, 36
      //   ("UNHIGHLIGHTED BLANK", 116, 156),       <--- 36, 76
      //   ("CONTENT", 156, 158),                   <--- 76, 78
      //   ("UNHIGHLIGHTED BLANK", 158, 197),       <--- 78, 117
      //   ("CONTENT", 197, 203),                   <--- 117, 123
      //   ("UNHIGHLIGHTED BLANK", 203, 246),       <--- 0, 43
      //   ("CONTENT", 246, 247)                    <--- 43, 44
      // ]
      (
        2,
        format!("Here is another sentence that has a [2]: {}, [3]: {}, and ", Pronoun1ForUser.display_to_user(), Pronoun2ForUser.display_to_user()),
        Some(vec![
          (String::from("CONTENT"), 0, 36),
          (String::from("UNHIGHLIGHTED BLANK"), 36, 36+n1+5),
          (String::from("CONTENT"), 36+n1+5, 36+n1+7),
          (String::from("UNHIGHLIGHTED BLANK"), 36+n1+7, 36+n1+n2+12),
          (String::from("CONTENT"), 36+n1+n2+12, 36+n1+n2+18),
        ])
      ),
      (
        2,
        format!("[4]: {}.", AllCollaterals.display_to_user()),
        Some(vec![
          (String::from("UNHIGHLIGHTED BLANK"), 0, n3+5),
          (String::from("CONTENT"), n3+5, n3+6+1),
        ])
      ),
    ];

    assert_eq!(display_vecs, nt_output_vecs);

    let nt4 = NoteTemplate::new(
      4,
      Sncd,
      true,
      format!(
        "A bunch of stuff happened today. Some good, some not so good. For example, here is some this that {} I have to write some stuff about. Lame, right?",
        ExternalDocument,
      ),
      vec![],
    );
    
    let (nt_display_string, formatting_vector4) = nt4.generate_display_content_string_with_blanks(None, None, None, None, Some(1));

    let s1a = String::from("[1]: A bunch of stuff happened today. ").chars().count();
    let s1b = String::from("[2]: Some good, some not so good. ").chars().count() + s1a;
    let s1c = String::from("[3]: For example, here is some this that ").chars().count() + s1b;
    let s1d = format!("{}", &ExternalDocument.display_to_user()).chars().count() + s1c;
    let s1e = String::from("[4]:  I have to write some stuff about. ").chars().count() + s1d;
    let s1f = String::from("[5]:  Lame, right?").chars().count() + s1e;
    
    let formatting4: Vec<(String, usize, usize)> = vec![
      (String::from("HIGHLIGHTED CONTENT"), 0, s1a),
      (String::from("UNHIGHLIGHTED CONTENT"), s1a, s1b),
      (String::from("UNHIGHLIGHTED CONTENT"), s1b, s1c),
      (String::from("UNFOCUSED BLANK"), s1c, s1d),
      (String::from("UNHIGHLIGHTED CONTENT"), s1d, s1e),
      (String::from("UNHIGHLIGHTED CONTENT"), s1e, s1f),
    ];

    assert_eq!(formatting4, formatting_vector4);
    
    let nt_output_vecs: Vec<(usize, String, Option<Vec<(String, usize, usize)>>)> = NoteTemplate::get_display_content_vec_from_string(nt_display_string, Some(formatting_vector4));
    
    let display_vecs: Vec<(usize, String, Option<Vec<(String, usize, usize)>>)> = vec![
      (
        0,
        String::from("[1]: A bunch of stuff happened today. "),
        Some(vec![
          (String::from("HIGHLIGHTED CONTENT"), 0, 38),
        ])
      ),
      (
        1,
        String::from("[2]: Some good, some not so good. "),
        Some(vec![
          (String::from("UNHIGHLIGHTED CONTENT"), 0, 34),
        ])
      ),
      (
        2,
        String::from(&format!("[3]: For example, here is some this that {}[4]:  I have to write some stuff about. ", &ExternalDocument.display_to_user())),
        Some(vec![
          (String::from("UNHIGHLIGHTED CONTENT"), 0, 41),
          (String::from("UNFOCUSED BLANK"), 41, 62),
          (String::from("UNHIGHLIGHTED CONTENT"), 62, 102),
        ])
      ),
      (
        3,
        String::from("[5]: Lame, right?"),
        Some(vec![
          (String::from("UNHIGHLIGHTED CONTENT"), 0, 17),
        ])
      ),
    ];

    assert_eq!(display_vecs, nt_output_vecs);

  }
  #[test]
  fn note_template_gets_sentence_end_indices() {
    let some_sentences = String::from("Sentence. Another sentence. Here's a third.");
    let sentence_indices: Vec<(usize, usize)> = vec![(0, 9), (10, 27), (28, 42)];
    let nt_indices = NoteTemplate::get_sentence_end_indices(0, some_sentences);
    assert_eq!(sentence_indices, nt_indices);
    let some_sentences_2 = String::from("Current user's pronouns are he. Jack's pronouns are them. Your pronouns are you. My pronouns are me.");
    let sentence_indices_2: Vec<(usize, usize)> = vec![(0, 31), (32, 57), (58, 80), (81, 99)];
    let nt_indices_2 = NoteTemplate::get_sentence_end_indices(0, some_sentences_2);
    assert_eq!(sentence_indices_2, nt_indices_2);
  }
  #[test]
  fn note_template_displays_properly_with_no_focus_id() {
    let nt4 = NoteTemplate::new(
      4,
      Sncd,
      true,
      format!(
        "A bunch of stuff happened today. Some good, some not so good. For example, here is some this that {} I have to write some stuff about. Lame, right?",
        ExternalDocument,
      ),
      vec![],
    );
    
    let (nt_display_string, formatting_vector4) = nt4.generate_display_content_string_with_blanks(None, None, None, None, None);

    let s1a = String::from("A bunch of stuff happened today. ").chars().count();
    let s1b = String::from("Some good, some not so good. ").chars().count() + s1a;
    let s1c = String::from("For example, here is some this that ").chars().count() + s1b;
    let s1d = format!("{}", &ExternalDocument.display_to_user()).chars().count() + s1c;
    let s1e = String::from(" I have to write some stuff about. ").chars().count() + s1d;
    let s1f = String::from(" Lame, right?").chars().count() + s1e;
    
    let formatting4: Vec<(String, usize, usize)> = vec![
      (String::from("CONTENT"), 0, s1a),
      (String::from("CONTENT"), s1a, s1b),
      (String::from("CONTENT"), s1b, s1c),
      (String::from("BLANK"), s1c, s1d),
      (String::from("CONTENT"), s1d, s1e),
      (String::from("CONTENT"), s1e, s1f),
    ];

    assert_eq!(formatting4, formatting_vector4);
    
    let nt_output_vecs: Vec<(usize, String, Option<Vec<(String, usize, usize)>>)> = NoteTemplate::get_display_content_vec_from_string(nt_display_string, Some(formatting_vector4));
    
    let display_vecs: Vec<(usize, String, Option<Vec<(String, usize, usize)>>)> = vec![
      (
        0,
        String::from("A bunch of stuff happened today. "),
        Some(vec![
          (String::from("CONTENT"), 0, 33),
        ])
      ),
      (
        1,
        String::from("Some good, some not so good. "),
        Some(vec![
          (String::from("CONTENT"), 0, 29),
        ])
      ),
      (
        2,
        String::from(&format!("For example, here is some this that {} I have to write some stuff about. ", &ExternalDocument.display_to_user())),
        Some(vec![
          (String::from("CONTENT"), 0, 36),
          (String::from("BLANK"), 36, ExternalDocument.display_to_user().len() + 36),
          (String::from("CONTENT"), ExternalDocument.display_to_user().len() + 36, ExternalDocument.display_to_user().len() + 36 + 35),
        ])
      ),
      (
        3,
        String::from("Lame, right?"),
        Some(vec![
          (String::from("CONTENT"), 0, 12),
        ])
      ),
    ];

    assert_eq!(display_vecs, nt_output_vecs);
  }
}
