#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_comparisons)]
#![allow(unused_attributes)]

use chrono::{Local, NaiveDate, Datelike, TimeZone, Utc, Weekday};
use std::fmt;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::{thread, time};
use std::collections::HashMap;

use crate::user::*;
use crate::client::*;
use crate::collateral::*;
use crate::pronouns::*;
use crate::note_day::*;
use crate::note::*;
use crate::blank_enums::*;
use EmployeeRole::{FP, ICC};
use SupportType::{Natural, Formal};
use StructureType::{CarePlan, CarePlanVerbose, Intake, Assessment, Sncd, HomeVisit, AgendaPrep, Debrief, PhoneCall, Scheduling, SentEmail, Referral, CustomStructure};
use NoteCategory::{ICCNote, FPNote};
use ICCNoteCategory::{FaceToFaceContactWithClient, TelephoneContactWithClient, CareCoordination, Documentation, CarePlanningTeam, TransportClient, MemberOutreachNoShow};
use FPNoteCategory::{Tbd};
use Blank::{CurrentUser, CurrentClientName, Collaterals, AllCollaterals, Pronoun1ForBlank, Pronoun2ForBlank, Pronoun3ForBlank, Pronoun4ForBlank, Pronoun1ForUser, Pronoun2ForUser, Pronoun3ForUser, Pronoun4ForUser, Pronoun1ForClient, Pronoun2ForClient, Pronoun3ForClient, Pronoun4ForClient, TodayDate, NoteDayDate, InternalDocument, ExternalDocument, InternalMeeting, ExternalMeeting, Action, Phrase, CustomBlank};

// blank fill-ins (likely to be updated later on)
use InternalDocumentFillIn::{
  ReferralForm,
  TelehealthConsent,
  TechnologyPlan,
  FinancialAgreement,
  InformedConsent,
  ComprehensiveAssessment,
  ChildAndAdolescentNeedsAndStrengths,
  StrengthsNeedsAndCulturalDiscovery,
  IndividualCarePlan,
  TransitionSummary,
  OtherInternalDocument,
};
use ExternalDocumentFillIn::{
  NeuropsychologicalAssessment,
  SchoolAssessment,
  IndividualEducationPlan,
  OtherExternalDocument,
};
use InternalMeetingFillIn::{
  IntakeMeeting,
  AssessmentMeeting,
  SncdMeeting,
  HomeVisitMeeting,
  AgendaPrepMeeting,
  CarePlanMeeting,
  DebriefMeeting,
  CheckInMeeting,
  TransitionMeeting,
  OtherInternalMeeting
};
use ExternalMeetingFillIn::{
  IEPMeeting,
  SchoolAssessmentMeeting,
  Consult,
  TreatmentTeamMeeting,
  OtherExternalMeeting,
};
use ActionFillIn::{
  Called,
  Emailed,
  Texted,
  Elicited,
  Reflected,
  Summarized,
  Scheduled,
  Affirmed,
  Brainstormed,
  Reviewed,
};
use PhraseFillIn::{
  AllTeamMembersPresentAtMeeting,
  AllTeamMembers,
};

use crate::utils::*;
use crate::constants::*;

pub struct NoteArchive {
  pub users: Vec<User>,
  pub clients: Vec<Client>,
  pub collaterals: Vec<Collateral>,
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

fn display_blanks() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  println!("{:-^73}", "-");
  println!("{:-^73}", " Blanks ");
  println!("{:-^73}", "-");
  println!("{:-^10} | {:-^60}", " ID ", " Type ");
  println!("{:-^73}", "-");
  for (i, b) in Blank::iterator().enumerate() {
    println!("{:-^10} | {:-<60}", i, b.display_to_user());
  }
  println!("Choose blank type by ID.");
}
fn choose_blanks() -> usize {
  loop {
    display_blanks();
    let mut input = String::new();
    let input_attempt = io::stdin().read_line(&mut input);
    match input_attempt {
      Ok(_) => (),
      Err(e) => {
        println!("Failed to read input. Try again.");
        thread::sleep(time::Duration::from_secs(2));
        continue;
      }
    }
    match input.trim().parse::<usize>() {
      Ok(num) => {
        if num > 0 && num <= Blank::vector_of_variants().len() {
          break num-1;
        } else {
          println!("Invalid ID.");
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      },
      Err(e) => {
        println!("Failed to read input as a number. Try again.");
        thread::sleep(time::Duration::from_secs(2));
        continue;
      }
    }
  }
}
fn choose_blanks_option() -> Option<usize> {
  loop {
    display_blanks();
    println!("Enter 'QUIT / Q' at any time to cancel.");
    let mut input = String::new();
    let input_attempt = io::stdin().read_line(&mut input);
    match input_attempt {
      Ok(_) => (),
      Err(e) => {
        println!("Failed to read input. Try again.");
        thread::sleep(time::Duration::from_secs(2));
        continue;
      }
    }
    match &input.trim()[..] {
      "QUIT" | "quit" | "Quit" | "q" | "Q" => return None,
      _ => (),
    }
    match input.trim().parse::<usize>() {
      Ok(num) => {
        if num > 0 && num <= Blank::vector_of_variants().len() {
          break Some(num-1);
        } else {
          println!("Invalid ID.");
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      },
      Err(e) => {
        println!("Failed to read input as a number. Try again.");
        thread::sleep(time::Duration::from_secs(2));
        continue;
      }
    }
  }
}

impl NoteArchive {
  pub fn run(&mut self) {
    self.choose_user();
    self.write_to_files();

    self.logged_in_action();
  }
  fn display_decrypt_files() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^58}", "-");
    println!("{:-^58}", " Files not readable ");
    println!("{:-^58}", "-");
    println!("{:-^15} | {:-^40}", " Command ", " Function ");
    println!("{:-^58}", "-");
    
    println!("{:-^58}", "-");
    println!(
      "{: >15} | {: <40}",
      " DECRPYT / D ", " Attempt to decrypt files with a password "
    );
    println!(
      "{: >15} | {: <40}",
      " DELETE ", " Delete all data and start over "
    );
    println!(
      "{: >15} | {: <40}",
      " QUIT / Q ", " Close program "
    );
    println!("{:-^58}", "-");

  }
  fn choose_decrypt_files(
    user_filepath: &str,
    client_filepath: &str,
    collateral_filepath: &str,
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
          println!("Failed to read input. Please try again.");
        }
      }
      choice = choice.trim().to_string();
      match &choice[..] {
        "DECRYPT" | "decrypt" | "Decrypt" | "D" | "d" => {
          println!("Enter password to attempt decryption.");
          let mut pw = String::new();
          let pw_attempt = io::stdin().read_line(&mut pw);
          match pw_attempt {
            Ok(_) => (),
            Err(e) => {
              println!("Failed to read input. Please try again.");
            }
          }
          pw = pw.trim().to_string();
          break match Self::decrypt_all_files(
            user_filepath,
            client_filepath,
            collateral_filepath,
            pronouns_filepath,
            note_day_filepath,
            note_template_filepath,
            note_filepath,
            &pw
          ) {
            Ok(_) => true,
            Err(_) => {
              println!("Incorrect password. Option to try again in 10 seconds.");
              thread::sleep(time::Duration::from_secs(10));
              continue;
            }
          }
        },
        "DELETE" | "delete" | "Delete" => {
          fs::remove_file(user_filepath).unwrap();
          fs::remove_file(client_filepath).unwrap();
          fs::remove_file(collateral_filepath).unwrap();
          fs::remove_file(pronouns_filepath).unwrap();
          fs::remove_file(note_day_filepath).unwrap();
          fs::remove_file(note_template_filepath).unwrap();
          fs::remove_file(note_filepath).unwrap();
          break true;
        },
        "QUIT" | "quit" | "Quit" | "Q" | "q" => {
          break false;
        },
        _ => {
          println!("Invalid command.");
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
      Err(e) => {
        build_note_archive = Self::choose_decrypt_files(
          &filepaths["user_filepath"],
          &filepaths["client_filepath"],
          &filepaths["collateral_filepath"],
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
        collaterals: Self::read_collaterals(&filepaths["collateral_filepath"]).unwrap(),
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
      ICC,
      1,
      vec![1, 2],
      vec![1, 2],
    );
    let user_2 = User::new(
      2,
      String::from("Sandy"),
      String::from("Sandyson"),
      FP,
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
      Some(2)
    );
    let nt2 = NoteTemplate::new(
      2,
      PhoneCall,
      true,
      String::from("ICC called (---co---) to discuss a referral for IHT services."),
      Some(1),
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
    self.write_collaterals().unwrap();
    self.write_pronouns().unwrap();
    self.write_note_days().unwrap();
    self.write_note_templates().unwrap();
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
    match Self::read_collaterals(&self.filepaths["collateral_filepath"]) {
      Ok(_) => encrypt_file(&self.filepaths["collateral_filepath"], pw)?,
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
    collateral_filepath: &str,
    pronouns_filepath: &str,
    note_day_filepath: &str,
    note_template_filepath: &str,
    note_filepath: &str,
    pw: &str) -> Result<(), Error> {
    decrypt_file(user_filepath, "decrypt_attempt_user.txt", pw)?;
    decrypt_file(client_filepath, "decrypt_attempt_client.txt", pw)?;
    decrypt_file(collateral_filepath, "decrypt_attempt_collateral.txt", pw)?;
    decrypt_file(pronouns_filepath, "decrypt_attempt_pronouns.txt", pw)?;
    decrypt_file(note_day_filepath, "decrypt_attempt_note_day.txt", pw)?;
    decrypt_file(note_template_filepath, "decrypt_attempt_note_template.txt", pw)?;
    decrypt_file(note_filepath, "decrypt_attempt_note.txt", pw)?;
    let user_result = Self::read_users("decrypt_attempt_user.txt");
    let client_result = Self::read_clients("decrypt_attempt_client.txt");
    let collateral_result = Self::read_collaterals("decrypt_attempt_collateral.txt");
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
      collateral_result,
      pronouns_result,
      note_day_result,
      note_template_result,
      note_result
    ) {
      (Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_), Ok(_)) => {
        decrypt_file(user_filepath, user_filepath, pw)?;
        decrypt_file(client_filepath, client_filepath, pw)?;
        decrypt_file(collateral_filepath, collateral_filepath, pw)?;
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
    println!("{:-^58}", "-");
    let heading_with_spaces = format!(" {} ", self.current_user().name_and_title()); 
    println!("{:-^58}", heading_with_spaces);
    println!("{:-^58}", " Mission control ");
    println!("{:-^58}", "-");
    println!("{:-^15} | {:-^40}", " Command ", " Function ");
    println!("{:-^58}", "-");
    
    // once for each command
    
    println!("{:-^58}", "-");
    println!(
      "{: >15} | {: <40}",
      " NOTE / N ", " Write, view, and edit note records "
    );

    println!("{:-^58}", "-");
    
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
    
    println!("{:-^58}", "-");
    
    println!("{: >15} | {: <40}", " USER / U ", " Switch user ");
    println!("{: >15} | {: <40}", " PRNS / P ", " View/edit pronoun records ");
    println!("{: >15} | {: <40}", " DELETE / D ", " Delete current user ");
    println!("{: >15} | {: <40}", " SECURITY / S ", " Security options ");
    println!("{: >15} | {: <40}", " QUIT / Q ", " End program ");

    println!("{:-^58}", "-");
  }
  fn logged_in_action(&mut self) {
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
        "NOTE" | "note" | "Note" | "N" | "n" => {
          self.choose_note_days();
        }
        "client" | "c" | "CLIENT" | "C" | "Client" => {
          self.choose_clients();
        },
        "collateral" | "co" | "COLLATERAL" | "CO" | "Collateral" | "Co" | "collat" | "COLLAT" | "Collat" | "COL" | "col" | "Col" => {
          self.choose_collaterals();
        },
        "edit" | "e" | "EDIT" | "E" | "Edit" => {
          self.choose_edit_user();
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
        "delete" | "d" | "DELETE" | "D" | "Delete" => {
          self.choose_delete_user();
          self.choose_user();
        },
        "SECURITY" | "security" | "Security" | "S" | "s" => {
          self.choose_security_options();
          if self.encrypted {
            break;
          }
        },
        "quit" | "q" | "QUIT" | "Q" | "Quit" => {
          break ();
        },
        _ => {
          println!("Invalid command.");
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
          println!("Failed to read input. Please try again.");
          continue;
        }
      }
      choice = choice.trim().to_string();
      match &choice[..] {
        "ENCRYPT" | "encrypt" | "Encrypt" => {
          self.choose_encrypt_all_files();
          break;
        },
        "quit" | "q" | "QUIT" | "Q" | "Quit" => {
          break ();
        },
        _ => {
          println!("Invalid command.");
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      }
    }

  }
  fn display_security_options(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^58}", "-");
    println!("{:-^58}", " Security ");
    println!("{:-^58}", "-");
    println!("{:-^15} | {:-^40}", " Command ", " Function ");
    println!("{:-^58}", "-");
    
    println!(
      "{: >15} | {: <40}",
      " ENCRYPT ", " Encrypt all files and protect with a password | WARNING: cannot be reversed if password is lost. "
    );
    println!(
      "{: >15} | {: <40}",
      " QUIT / Q ", " Cancel "
    );
    
    println!("{:-^58}", "-");
  }
  fn choose_encrypt_all_files(&mut self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("If you forget your password, accessing this program's data will be impossible.");
    println!("This applies to all data for all users and clients associated with this program on your computer.");
    println!("Encryption is not and is not intended to be HIPPA compliant.");
    println!("By continuing, you agree that the responsibility to store and transfer data privately and securely is entirely your own.");
    println!("To cancel and return to the program, enter QUIT / Q.");
    println!("To continue with secure encryption, enter YES / Y.");

    loop {
      let mut choice = String::new();
      let choice_attempt = io::stdin().read_line(&mut choice);
      match choice_attempt {
        Ok(_) => (),
        Err(e) => {
          println!("Failed to read input. Please try again.");
          continue;
        }
      }
      choice = choice.trim().to_string();
      match &choice[..] {
        "YES" | "yes" | "Yes" | "Y" | "y" => {
          let new_password = loop {
            println!("Enter new password for encryption (minimum 8 characters):");
            let mut choice = String::new();
            let choice_attempt = io::stdin().read_line(&mut choice);
            match choice_attempt {
              Ok(_) => {
                if choice.trim().len() < 8 {
                  println!("Password not long enough.");
                  continue;
                } else {
                  println!("Confirm password:");
                  let mut confirm = String::new();
                  let confirm_attempt = io::stdin().read_line(&mut confirm);
                  match confirm_attempt {
                    Ok(_) => {
                      if confirm.trim() != choice.trim() {
                        println!("Passwords do not match.");
                        continue;
                      } else {
                        break confirm.trim().to_string()
                      }
                    },
                    Err(e) => {
                      println!("Passwords do not match (error: {})", e);
                      continue;
                    }
                  }
                  
                }
              },
              Err(e) => {
                println!("Failed to read input. Please try again.");
                continue;
              }
            }
          };
          match self.encrypt_all_files(&new_password) {
            Ok(_) => (),
            Err(e) => {
              println!("Failed to encrypt files: {}", e);
              continue;
            },
          }
          self.encrypted = true;
          print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
          println!("Files encrypted successfully.");
          thread::sleep(time::Duration::from_secs(2));
          break;
        },
        "quit" | "q" | "QUIT" | "Q" | "Quit" => {
          break ();
        },
        _ => {
          println!("Invalid command.");
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
    println!("{:-^66}", "-");
    println!("{:-^66}", " Users ");
    println!("{:-^66}", "-");
    println!("{:-^10} | {:-^10} | {:-^40}", " ID ", " Role ", " Name ");
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
    let verified_id = 'outer: loop {
      self.display_users();
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
            let maybe_user_id = self.create_user_get_id();
            match maybe_user_id {
              Some(num) => break num,
              None => continue 'outer,
            }
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
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      }
    };
    verified_id
  }
  fn create_user_get_id(&mut self) -> Option<u32> {
    let user = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        println!("Enter 'CANCEL' at any time to cancel.");
        println!("First name:");
        let first_name_attempt = io::stdin().read_line(&mut first_name_choice);
        match first_name_attempt {
          Ok(_) => break String::from(first_name_choice.trim()),
          Err(e) => {
            println!("Invalid first name: {}", e);
            continue;
          }
        };
      };
      if first_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let last_name = loop {
        let mut last_name_choice = String::new();
        println!("Last name:");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println!("Invalid last name: {}", e);
            continue;
          }
        };
      };
      if last_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let role: EmployeeRole = loop {
        let mut role_choice = String::new();
        println!("Role ('ICC' or 'FP'):");
        let role_attempt = io::stdin().read_line(&mut role_choice);
        match role_attempt {
          Ok(_) => match &role_choice.trim().to_ascii_lowercase()[..] {
            "icc" => break ICC,
            "fp" => break FP,
            "cancel" => return None,
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
      let pronouns = 'pronouns: loop {
        match self.choose_pronouns_option() {
          Some(p) => break p,
          None => {
            loop {
              println!("Cancel? (Y/N)");
              let mut cancel = String::new();
              let cancel_attempt = io::stdin().read_line(&mut cancel);
              match cancel_attempt {
                Ok(_) => match &cancel.trim().to_lowercase()[..] {
                  "yes" | "y"  => return None,
                  "no" | "n" | "cancel" => continue 'pronouns,
                  _ => {
                    println!("Please choose either 'yes/y' or 'no/n'.");
                    continue;
                  }
                },
                Err(e) => {
                  println!("Failed to read input.");
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
          println!("User could not be generated: {}.", e);
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
    let id: u32 = self.users.len() as u32 + 1;

    match self.user_dup_id_option(&first_name, &last_name, &role) {
      Some(_) => Err(format!("There is already a {} with the name '{} {}'.", role, first_name, last_name)),
      None => Ok(User::new(id, first_name, last_name, role, pronouns, vec![], vec![])),
    }
  }
  fn save_user(&mut self, user: User) {
    let pos = self.users.binary_search_by(|u| u.id.cmp(&user.id) ).unwrap_or_else(|e| e);
    self.users.insert(pos, user);
    self.write_users().unwrap();
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
          let pronouns_option = self.choose_pronouns_option();
          match pronouns_option {
            Some(p) => self.current_user_mut().pronouns = p,
            None => (),
          }
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
      self.current_user().foreign_keys["client_ids"].len(),
    );
    println!("{:-^79}", "-");
  }
  fn delete_current_user(&mut self) {
    let id = self.foreign_key.get("current_user_id").unwrap();
    self.users.retain(|u| u.id != *id);
    self.reindex_users();
    self.foreign_key.remove("current_user_id");
    self.foreign_key.remove("current_client_id");
    self.foreign_key.remove("current_collateral_id");

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
    let mut heading = String::from(" ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", heading);
    println!("{:-^96}", "-");
    println!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Name ", " DOB ");
    match self.foreign_key.get("current_user_id") {
      Some(_) => {
        for c in self.get_current_clients() {
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
    println!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Name ", " DOB ");
    match self.foreign_key.get("current_user_id") {
      Some(_) => {
        for c in self.get_current_clients() {
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
  fn get_noncurrent_clients(&self) -> Vec<&Client> {
    self.clients.iter().filter(|client| !self.current_user().foreign_keys["client_ids"]
        .iter()
        .any(|&id| id == client.id)
      )
      .collect()
  }
  fn display_add_client(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", " Clients ");
    println!("{:-^96}", "-");
    println!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Name ", " DOB ");
    match &self.foreign_key.get("current_user_id") {
      Some(c_ids) => {
        for c in self.get_noncurrent_clients() {
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
    println!("| {} | {} | {}", "Enter ID to add client.", "NEW / N: create new", "QUIT / Q: quit menu");
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
              println!("Please select from the available choices.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            } else {
              match self.load_client(num) {
                Ok(_) => {
                  self.current_user_mut().foreign_keys.get_mut("client_ids").unwrap().push(num);
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
          let maybe_new_id = self.create_client_get_id();
          match maybe_new_id {
            Some(new_id) => self.update_current_clients(new_id),
            None => (),
          }
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
            if !self.get_current_clients()
              .iter()
              .any(|&c| c.id == num) {
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
              if !self.get_current_clients().iter().any(|c| c.id == num) {
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
  fn display_specify_clients(&self, purpose: String) {
    let heading = format!(" Choose client for {} ", purpose);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", heading);
    println!("{:-^96}", "-");
    println!("{:-^10} | {:-^40} | {:-^40}", " ID ", " NAME ", " DOB ");
    match self.foreign_key.get("current_user_id") {
      Some(_) => {
        for c in self.get_current_clients() {
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
  fn specify_client(&mut self, purpose: String) -> u32 {
    let id: u32 = loop {
      let input = loop {
        self.display_specify_clients(purpose.clone());
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
          let maybe_new_id = self.create_client_get_id();
          match maybe_new_id {
            Some(new_id) => self.update_current_clients(new_id),
            None => (),
          }
          continue;
        },
        "ADD" | "add" | "Add" | "a" | "A" => {
          self.add_client();
          continue;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.get_current_clients()
              .iter()
              .any(|c| c.id == num) {
                println!("Please choose from among the listed clients.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              } else {
                match self.load_client(num) {
                  Ok(_) => {
                    self.foreign_key.remove("current_client_id");
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
      println!("| {} | {} | {} | {}", "EDIT / E: edit client", "DELETE: delete client", "COLLATERAL / COL: view/edit client collaterals", "QUIT / Q: quit menu");
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
  fn create_client_get_id(&mut self) -> Option<u32> {
    let client = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        println!("Enter 'CANCEL' at any time to cancel.");
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
      if first_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
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
      if last_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let dob: NaiveDate = loop {
        let birth_year = loop {
          let mut birth_year_choice = String::new();
          println!("Enter client's birth year.");
          let birth_year_attempt = io::stdin().read_line(&mut birth_year_choice);
          let birth_year_attempt = match birth_year_attempt {
            Ok(_) => {
              if birth_year_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                return None;
              }
              birth_year_choice.trim().parse()
            },
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
            Ok(_) => {
              if birth_month_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                return None;
              }
              birth_month_choice.trim().parse()
            },
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
            Ok(_) => {
              if birth_day_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                return None;
              }
              birth_day_choice.trim().parse()
            },
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

      let pronouns = 'pronouns: loop {
        match self.choose_pronouns_option() {
          Some(p) => break p,
          None => {
            loop {
              println!("Cancel? (Y/N)");
              let mut cancel = String::new();
              let cancel_attempt = io::stdin().read_line(&mut cancel);
              match cancel_attempt {
                Ok(_) => match &cancel.trim().to_lowercase()[..] {
                  "yes" | "y"  => return None,
                  "no" | "n" | "cancel" => continue 'pronouns,
                  _ => {
                    println!("Please choose either 'yes/y' or 'no/n'.");
                    continue;
                  }
                },
                Err(e) => {
                  println!("Failed to read line.");
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
          println!(
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
                      println!("Would you like to use the existing record? (Y/N)");
                      let conf_attempt = io::stdin().read_line(&mut conf);
                      match conf_attempt {
                        Ok(_) => break String::from(conf.trim()),
                        Err(_) => {
                          println!("Failed to read input.");
                          continue;
                        }
                      }
                    };
                    match &choice[..] {
                      "YES" | "yes" | "Y" | "y" => break client.clone(),
                      "NO" | "no" | "N" | "n" => continue,
                      _ => println!("Invalid command.")
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
    self.write_clients().unwrap();
  }
  fn update_current_clients(&mut self, id: u32) {
    self.current_user_mut().foreign_keys.get_mut("client_ids").unwrap().push(id);
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
          let maybe_pronouns = self.choose_pronouns_option();
          match maybe_pronouns {
            Some(p) => self.current_client_mut().pronouns = p,
            None => (),
          }
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
      self.get_current_collaterals().len(),
    );
    println!("{:-^114}", "-");
  }
  fn delete_current_client(&mut self) {
    let id = self.foreign_key.get("current_client_id").unwrap();
    self.clients.retain(|c| c.id != *id);
    self.reindex_clients();
    self.foreign_key.remove("current_client_id");
    self.foreign_key.remove("current_collateral_id");
  }
  fn reindex_clients(&mut self) {
    let mut i: u32 = 1;
    for mut c in &mut self.clients {
      for u in &mut self.users {
        for c_id in &mut u.foreign_keys.get_mut("client_ids").unwrap().iter_mut() {
          if c_id == &c.id {
            *c_id = i;
          }
        }
      }
      c.id = i;
      i += 1;
    }
  }
  fn get_client_option_by_id(&self, id: u32) -> Option<&Client> {
    self.clients.iter().find(|c| c.id == id)
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
  fn current_user_collaterals(&self) -> Vec<&Collateral> {
    let collats = self.collaterals
      .iter()
      .filter(|co|
        self.current_user().foreign_keys["collateral_ids"]
          .iter()
          .any(|co_id| co_id == &co.id )
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
  fn display_client_collaterals(&self) {
    let mut heading = String::from(" ");
    heading.push_str(&self.current_client().full_name()[..]);
    heading.push_str("'s Collaterals ");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^113}", "-");
    println!("{:-^113}", heading);
    println!("{:-^113}", "-");
    println!("{:-^10} | {:-<100}", " ID ", "Info ");
    match self.foreign_key.get("current_client_id") {
      Some(_) => {
        for c in self.get_current_collaterals() {
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
    println!("| {} | {} | {} | {} | {}",
      "Enter ID to choose collateral.",
      "NEW / N: new collateral",
      "ADD / A: add from other client/user",
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
    println!("{:-^113}", "-");
    println!("{:-^113}", heading);
    println!("{:-^113}", "-");
    println!("{:-^10} | {:-<100}", " ID ", " Info ");
    match self.foreign_key.get("current_client_id") {
      Some(_) => {
        for c in self.get_current_collaterals() {
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
    println!("{:-^10} | {:-<100} | {:-<30}", " ID ", "Info ", "Youth(s) ");

    for co in self.current_user_collaterals() {

      println!(
        "{: ^10} | {: <100} | {: <30}",
        co.id,
        co.full_name_and_title(),
        self.collateral_clients_string(co.id),
      );
    }
    println!("{:-^146}", "-");
    println!("| {} | {} | {} | {} | {}",
      "Enter ID to choose collateral.",
      "EDIT / E: edit", "NEW / N: new collateral",
      "ADD / A: Add from other user/client",
      "QUIT / Q: quit menu"
    );
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
    println!("{:-^10} | {:-<100} | {:-<30}", " ID ", "Info ", "Youths ");

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
    println!("| {} | {} | {} | {}", "EDIT / E: edit collateral", "CLIENT / C: add client ", "DELETE: delete collateral", "QUIT / Q: quit menu");
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
  fn display_add_collateral(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^113}", "-");
    println!("{:-^113}", " Other collateral records ");
    println!("{:-^113}", "-");
    println!("{:-^10} | {:-<100}", " ID ", "Info ");
    match self.foreign_key.get("current_client_id") {
      Some(_) => {
        for c in self.get_noncurrent_collaterals() {
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
    println!("| {} | {} | {}", "Enter ID to add collateral.", "NEW / N: new collateral", "QUIT / Q: quit menu");
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
          let maybe_new_id = self.create_collateral_get_id();
          match maybe_new_id {
            Some(new_id) => self.update_current_collaterals(new_id),
            None => (),
          }
          continue;
        },
        "ADD" | "add" | "Add" | "a" | "A" => {
          self.add_collateral();
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
            if !self.get_current_collaterals().iter().any(|co| co.id == num) {
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
  fn choose_get_client_collateral(&mut self, selected_id: u32) -> Option<&Collateral> {
    loop {
      match self.get_current_collaterals().iter().find(|co| co.id == selected_id) {
        Some(collat) => {
          break Some(collat);
        },
        None => {
          println!("Please select one of the listed IDs.");
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
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
            if !self.get_current_collaterals().iter().any(|co| co.id == num) {
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
          let maybe_new_id = self.create_collateral_get_id();
          match maybe_new_id {
            Some(_) => (),
            None => (),
          }
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
            if !self.current_user().foreign_keys["collateral_ids"].iter().any(|n| n == &num) {
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
            if !self.current_user().foreign_keys["collateral_ids"].iter().any(|n| n == &num) {
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
        "CLIENT" | "client" | "Client" | "C" | "c" => {
          let c_id = self.specify_client(String::from("collateral"));
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
  fn create_collateral_get_id(&mut self) -> Option<u32> {
    let collateral = loop {
      let first_name = loop {
        let mut first_name_choice = String::new();
        println!("Enter 'CANCEL' at any time to cancel.");
        println!("Collateral's first name:");
        let first_name_attempt = io::stdin().read_line(&mut first_name_choice);
        match first_name_attempt {
          Ok(_) => break String::from(first_name_choice.trim()),
          Err(e) => {
            println!("Invalid first name: {}", e);
            continue;
          }
        };
      };
      if first_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let last_name = loop {
        let mut last_name_choice = String::new();
        println!("Collateral's last name:");
        let last_name_attempt = io::stdin().read_line(&mut last_name_choice);
        match last_name_attempt {
          Ok(_) => break String::from(last_name_choice.trim()),
          Err(e) => {
            println!("Invalid last name: {}", e);
            continue;
          }
        };
      };
      if last_name.to_ascii_lowercase() == String::from("cancel") {
        return None;
      }
      let title = loop {
        let mut title_choice = String::new();
        println!("Enter collateral's role/title.");
        let title_attempt = io::stdin().read_line(&mut title_choice);
        match title_attempt {
          Ok(_) => break String::from(title_choice.trim()),
          Err(e) => {
            println!("Invalid title: {}", e);
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
              println!("Cancel? (Y/N)");
              let mut cancel = String::new();
              let cancel_attempt = io::stdin().read_line(&mut cancel);
              match cancel_attempt {
                Ok(_) => match &cancel.trim().to_lowercase()[..] {
                  "yes" | "y" => return None,
                  "no" | "n" | "cancel" => continue 'pronouns,
                  _ => {
                    println!("Please choose either 'yes/y' or 'no/n'.");
                    continue;
                  }
                },
                Err(e) => {
                  println!("Failed to read input.");
                  continue;
                }
              }
  
            }
          }
        } 
      };

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
              "CANCEL" | "cancel" | "Cancel" => return None,
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
          let i: bool;
          match s {
            Natural => i = false,
            Formal => {
              print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
              let mut indirect_choice = String::new();
              println!("Is this collateral a provider for the selected client?");
              println!("YES / Y | NO / N");
              let indirect_attempt = io::stdin().read_line(&mut indirect_choice);
              i = match indirect_attempt {
                Ok(_) => match indirect_choice.trim() {
                  "YES" | "yes" | "Y" | "y" => false,
                  "NO" | "no" | "N" | "n" => true,
                  "CANCEL" | "cancel" | "Cancel" => return None,
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
            }
          }
          let institution = loop {
            if s == Formal {
              let mut institution_choice = String::new();
              println!("Enter collateral's institution.");
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
        Err(error_hash) => {
          println!(
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
                      println!("Would you like to use the existing record? (Y/N)");
                      let conf_attempt = io::stdin().read_line(&mut conf);
                      match conf_attempt {
                        Ok(_) => break String::from(conf.trim()),
                        Err(_) => {
                          println!("Failed to read input.");
                          continue;
                        }
                      }
                    };
                    match &choice[..] {
                      "YES" | "yes" | "Y" | "y" => break collat.clone(),
                      "NO" | "no" | "N" | "n" => continue,
                      "CANCEL" | "Cancel" | "cancel" => return None,
                      _ => println!("Invalid command."),
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
        if !self.current_client().foreign_keys["collateral_ids"].iter().any(|co_id| co_id == &id ) {
          self.current_client_mut().foreign_keys.get_mut("collateral_ids").unwrap().push(id)
        }
      }
      None => {
        let c_id = self.specify_client(String::from("collateral"));
        if !self.get_client_by_id(c_id).unwrap().foreign_keys.get("collateral_ids").unwrap().iter().any(|co_id| co_id == &id ) {
          self.get_client_by_id_mut(c_id).unwrap().foreign_keys.get_mut("collateral_ids").unwrap().push(id);
        }
      }
    }
    match self.get_collateral_by_id(id) {
      Some(_) => (),
      None => self.save_collateral(collateral),
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
  fn generate_unique_new_collateral(
    &mut self,
    first_name: String,
    last_name: String,
    title: String,
    institution: Option<String>,
    pronouns: u32,
    support_type: SupportType,
    indirect_support: bool,
  ) -> Result<Collateral, HashMap<String, u32>> {
    let id: u32 = self.collaterals.len() as u32 + 1;

    match self.collateral_dup_id_option(&first_name, &last_name, &title, &institution) {
      Some(match_id) => Err([(String::from("duplicate"), match_id)].iter().cloned().collect::<HashMap<String, u32>>()),
      None => Ok(Collateral::new(id, first_name, last_name, title, institution, pronouns, support_type, indirect_support))
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

      let c = Collateral::new(id, first_name, last_name, title, institution, pronouns, support_type, indirect_support);
      collaterals.push(c);
    }
    Ok(collaterals)
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
  fn get_collateral_user_id(&self, id:u32) -> Option<u32> {
    match self.users
      .iter()
      .find(|u|
        u.foreign_keys["collateral_ids"].iter().any(|co_id| co_id == &id)
      ) {
        Some(u) => Some(u.id),
        None => None,
      }
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

    let client_ids: Vec<Option<u32>> = self.collaterals.iter().map(|c|
      match self.get_first_client_with_collat_id(c.id) {
        Some(client) => Some(client.id),
        None => None,
      }
    ).collect();
    let mut collaterals_and_client_ids = client_ids
      .iter()
      .enumerate()
      .map(|(index, client_id_option)| (self.collaterals[index].clone(), client_id_option) )
      .collect::<Vec<(Collateral, &Option<u32>)>>();

    collaterals_and_client_ids.sort_by(|(i_a, u_a), (i_b, u_b)|
      u_a.cmp(&u_b)
    );

    let collat_refs_by_client = collaterals_and_client_ids
      .iter()
      .map(|(collat, _)| collat )
      .collect::<Vec<&Collateral>>();

    self.collaterals = collat_refs_by_client.iter().map(|col| *col ).cloned().collect();

    // sort by user ID (first located user ID)

    let user_ids: Vec<Option<u32>> = self.collaterals.iter().map(|c| self.get_collateral_user_id(c.id)).collect();
    let mut collaterals_and_user_ids = user_ids
      .iter()
      .enumerate()
      .map(|(index, user_id)| (self.collaterals[index].clone(), user_id) )
      .collect::<Vec<(Collateral, &Option<u32>)>>();

    collaterals_and_user_ids.sort_by(|(i_a, u_a), (i_b, u_b)| u_a.cmp(&u_b) );

    let collat_refs_by_user = collaterals_and_user_ids
      .iter()
      .map(|(collat, _)| collat )
      .collect::<Vec<&Collateral>>();

    self.collaterals = collat_refs_by_user.iter().map(|col| *col ).cloned().collect();

    let current_user_sorted_collateral_ids: Vec<u32> = self.collaterals
      .iter()
      .map(|c| c.id)
      .filter(|c_id| self.current_user().foreign_keys["collateral_ids"].iter().any(|id| id == c_id ) )
      .collect();

    self.current_user_mut().foreign_keys.insert(String::from("collateral_ids"), current_user_sorted_collateral_ids);
  }
  pub fn save_collateral(&mut self, collateral: Collateral) {
    let collat_id = collateral.id;
    self.collaterals.push(collateral);
    self.current_user_mut().foreign_keys.get_mut("collateral_ids").unwrap().push(collat_id);

    self.sort_collaterals();
    self.write_collaterals().unwrap();
  }
  fn update_current_collaterals(&mut self, id: u32) {
    self.current_client_mut().foreign_keys.get_mut("collateral_ids").unwrap().push(id);
    self.sort_collaterals();
  }
  fn add_collateral(&mut self) {
    loop {
      self.display_add_collateral();
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
          let maybe_new_id = self.create_collateral_get_id();
          match maybe_new_id {
            Some(new_id) => {
              self.update_current_collaterals(new_id);
              println!("Collateral added to client '{}'.", self.current_client().full_name());
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
              println!("Please select from the available choices.");
              thread::sleep(time::Duration::from_secs(1));
              continue;
            } else {
              match self.load_collateral(num) {
                Ok(_) => {
                  self.current_client_mut().foreign_keys.get_mut("collateral_ids").unwrap().push(num);
                  self.update_current_collaterals(num);
                  break;
                }
                Err(e) => {
                  println!("Unable to load collateral with id {}: {}", num, e);
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
          let maybe_pronouns = self.choose_pronouns_option();
          match maybe_pronouns {
            Some(p) => self.current_collateral_mut().pronouns = p,
            None => (),
          }
        },
        "FORMAL" | "formal" | "Formal" => {
          if self.current_collateral().support_type == Formal {
            println!("Collateral is already a formal support.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            loop {
              println!("Institution required for formal support. Enter institution below, or 'NONE' to cancel:");
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
                        println!("A collateral already exists with that information. Consider selecting ADD from the collateral menu.");
                        thread::sleep(time::Duration::from_secs(3));
                        break;
                      },
                      None => self.current_collateral_mut().institution = Some(inst_choice.trim().to_string()),
                    }
                  }
                },
                Err(_) => {
                  println!("Failed to read line.");
                  thread::sleep(time::Duration::from_secs(1));
                  continue;
                },
              }
              self.current_collateral_mut().support_type = Formal;
              break;
            }
          }
        },
        "NATURAL" | "natural" | "Natural" => {
          if self.current_collateral().support_type == Natural {
            println!("Collateral is already a natural support.");
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
                  println!("Creates duplicate record because another natural support has the same name and no institution.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                },
                None => {
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
                },
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
    let id = self.foreign_key.get("current_collateral_id").unwrap();
    self.collaterals.retain(|c| c.id != *id);
    self.reindex_collaterals();
    self.foreign_key.remove("current_collateral_id");
  }
  fn reindex_collaterals(&mut self) {
    let mut i: u32 = 1;
    for mut co in &mut self.collaterals {
      for cl in &mut self.clients {
        for co_id in &mut cl.foreign_keys.get_mut("collateral_ids").unwrap().iter_mut() {
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
  fn get_collateral_by_id_mut(&mut self, id: u32) -> Option<&mut Collateral> {
    self.collaterals.iter_mut().find(|p| p.id == id)
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
        println!("| {} | {} | {} | {}", "NEW / N: new", "EDIT / E: edit (for all)", "DELETE / D: delete (for all)", "QUIT / Q: quit menu/cancel ");
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
    println!("{:-^10} | {:-^31}", " ID ", " Pronouns ");
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
  fn pronouns_already_exist(&self, subject: String, object: String, possessive_determiner: String, possessive: String) -> Option<u32> {
    let id: u32 = self.pronouns.len() as u32 + 1;

    let new_pronouns = Pronouns::new(
      id,
      subject.to_lowercase(),
      object.to_lowercase(),
      possessive_determiner.to_lowercase(),
      possessive.to_lowercase(),
    );

    match self.pronouns.iter().find(|p| p == &&new_pronouns) {
      Some(p) => Some(p.id),
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
    self.write_pronouns().unwrap();
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
    match self.foreign_key.get("current_user_id") {
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
  /// returns the first 10 notedays for the current user
  /// completed within 4 days of the most recent date
  fn current_user_recent_10_note_days(&self) -> Vec<&NoteDay> {
    let user_note_days: Vec<&NoteDay> = self.note_days.iter().filter(|nd| nd.foreign_key["user_id"] == self.current_user().id )
      .collect();

    if user_note_days.len() == 0 {
      vec![]
    } else {
      user_note_days.iter().map(|nd| *nd ).take(10).collect()
    } 
    
  }
  fn display_user_all_note_days(&self) {
    let mut heading = String::from(" All notes for ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", heading);
    println!("{:-^96}", "-");
    println!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Client ", " Date of note ");
    for nd in self.current_user_note_days() {
      println!(
        "{: ^10} | {: ^40} | {: <12} {: >26}",
        nd.id,
        self.get_client_by_id(nd.foreign_key["client_id"]).unwrap().full_name(),
        nd.fmt_date(),
        nd.fmt_date_long()
      );
    }
    println!("{:-^96}", "-");
    println!("| {} | {}", "Choose note by ID.", "NEW / N: New note");
  }
  fn display_user_recent_note_days(&self) {
    let mut heading = String::from(" Recent notes for ");
    heading.push_str(&self.current_user().full_name()[..]);
    heading.push_str("'s clients ");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^96}", heading);
    println!("{:-^96}", "-");
    println!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Client ", " Date of note ");
    for nd in self.current_user_recent_10_note_days() {
      println!(
        "{: ^10} | {: ^40} | {: <12} {: >26}",
        nd.id,
        self.get_client_by_id(nd.foreign_key["client_id"]).unwrap().full_name(),
        nd.fmt_date(),
        nd.fmt_date_long()
      );
    }
    println!("{:-^96}", "-");
    println!("| {} | {} | {} | {}", "Choose note by ID.", "NEW / N: New note", "ALL / A: View all notes", "QUIT / Q: quit menu");
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
  fn choose_note_days(&mut self) {
    let mut display_all = false;
    loop {
      let input = loop {
        if display_all {
          self.display_user_all_note_days();
        } else {
          self.display_user_recent_note_days();
        }
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
          let maybe_new_id = self.create_note_day_get_id();
          match maybe_new_id {
            Some(_) => (),
            None => (),
          }
          continue;
        },
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          // self.choose_edit_note_days();
          println!("          // self.choose_edit_note_days();");
          continue;
        },
        "ALL" | "all" | "All" | "A" | "a" => {
          display_all = true;
          continue;
        },
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_note_days()
              .iter()
              .any(|&nd| nd.id == num) {
                println!("Please choose from among the listed dates, or 'NEW / N' to begin a new record.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note_day(num) {
              Ok(_) => self.choose_note_day(),
              Err(e) => {
                println!("Unable to load records with ID {}: {}", num, e);
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
  fn display_note_day(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^174}", "-");
    
    let notes = self.current_note_day_notes();
    
    let nd = self.current_note_day();
    let c = self.get_client_by_note_day_id(nd.id).unwrap();
    let heading = format!(" Notes for {} for {} ", c.full_name(), nd.fmt_date());
    println!("{:-^174}", heading);
    println!("{:-^174}", "-");
    println!("{:-^6} | {:-^35} | {:-^30} | {:-^10} | {:-^79}", " ID ", " Category ", " Topic/structure ", " Word count ", " Content sample " );
    println!("{:-^174}", "-");
    for n in notes {
      let nt_opt = self.get_note_template_by_note_id(n.id);
      let nt_display = match nt_opt {
        Some(nt) => format!("{}", nt),
        None => String::from("n/a"),
      };

      let words: Vec<&str> = n.content.split(" ").collect();
      let sample = if n.generate_display_content_string().len() > 75 {
        format!("{}{}", String::from(&n.generate_display_content_string()[..75]), String::from("..."))
      } else {
        n.generate_display_content_string()
      };

      let cat = match n.category {
        ICCNote(c) => c.to_string(),
        FPNote(c) => c.to_string(),
      };
      let n_structure = n.structure.to_string();
      
      println!("{:-^6} | {:-^35} | {:-^30} | {:-^10} | {:-^79}", n.id, cat, n_structure, words.len(), sample);
    }
    println!("{:-^174}", "-");
  }
  fn choose_delete_notes(&mut self) {
    loop {

      self.display_note_day();
      println!("Select note ID to delete.");
      println!("CANCEL / C: Cancel");

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

        "CANCEL" | "cancel" | "Cancel" | "C" | "c" => {
          break;
        }
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_notes()
              .iter()
              .any(|&nd| nd.id == num) {
                println!("Please choose from among the listed note IDs.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note(num) {
              Ok(_) => {
                self.choose_delete_note();
                break;
              }
              Err(e) => {
                println!("Unable to load records with ID {}: {}", num, e);
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
  fn choose_note_day(&mut self) {
    loop {

      self.display_note_day();

      println!("| {} | {} | {}", "NEW / N: new note", "DELETE: delete note", "QUIT / Q: quit menu");
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
          self.choose_delete_notes();
        }
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          println!("Choose note by ID to edit its content.");
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
        "NEW" | "new" | "New" | "n" | "N" => {
          let n_id = self.create_note_get_id(None);
        }
        _ => {
          match input.to_string().parse() {
            Ok(num) => {
              if !self.current_user_notes().iter().any(|n| n.id == num ) {
                println!("Please choose from among the listed note IDs.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              } else {
                match self.get_note_option_by_id(num) {
                  None => {
                    println!("Invalid note ID.");
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  },
                  Some(n) => {
                    match self.load_note(num) {
                      Ok(_) => self.choose_note(),
                      Err(e) => {
                        println!("Unable to load template with ID {}: {}", num, e);
                        thread::sleep(time::Duration::from_secs(1));
                        continue;
                      }
                    }                    
                  }
                }
              }
            },
            Err(e) => {
              println!("Invalid command.");
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
      println!("Enter 'CANCEL' at any time to cancel.");
      println!("Note for today? (Y/N)");
      let today_attempt = io::stdin().read_line(&mut today_choice);
      match today_attempt {
        Ok(_) => (),
        Err(e) => {
          println!("Invalid repsonse: {}", e);
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
            println!("This year ({})? (Y/N)", today.year());
            let this_year_attempt = io::stdin().read_line(&mut this_year_choice);
            match this_year_attempt {
              Ok(_) => match &this_year_choice.trim()[..] {
                "YES" | "yes" | "Yes" | "Y" | "y" => break today.year(),
                "NO" | "no" | "No" | "N" | "n" => {
                  let mut year_choice = String::new();
                  println!("What year?");
                  let year_attempt = io::stdin().read_line(&mut year_choice);
                  match year_attempt {
                    Ok(_) => {
                      if year_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                        return None;
                      }
                      match year_choice.trim().parse() {
                        Ok(val) => {
                          if val > 9999 || val < 1000 {
                            println!("Please enter a valid year.");
                            continue;
                          } else {
                            break val;
                          }
                        }
                        Err(e) => {
                          println!("Invalid repsonse: {}", e);
                          continue;
                        }
                      }
                    }
                    Err(e) => {
                      println!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                },
                "Cancel" | "CANCEL" | "cancel" => return None,
                _ => {
                  println!("Please choose 'yes' or 'no.'");
                  continue;
                }
              },
              Err(e) => {
                println!("Invalid repsonse: {}", e);
                continue;
              }
            }
          };
          let month = loop {
            let mut this_month_choice = String::new();
            println!("This month ({})? (Y/N)", today.month());
            let this_month_attempt = io::stdin().read_line(&mut this_month_choice);
            match this_month_attempt {
              Ok(_) => match &this_month_choice.trim()[..] {
                "YES" | "yes" | "Yes" | "Y" | "y" => break today.month(),
                "NO" | "no" | "No" | "N" | "n" => {
                  let mut month_choice = String::new();
                  println!("What month?");
                  let month_attempt = io::stdin().read_line(&mut month_choice);
                  match month_attempt {
                    Ok(_) => {
                      if month_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                        return None;
                      }
                      match month_choice.trim().parse() {
                        Ok(val) => {
                          if val > 12 || val < 1 {
                            println!("Please enter a valid month.");
                            continue;
                          } else {
                            break val;
                          }
                        }
                        Err(e) => {
                          println!("Invalid repsonse: {}", e);
                          continue;
                        }
                      }
                    }
                    Err(e) => {
                      println!("Invalid repsonse: {}", e);
                      continue;
                    }
                  }
                },
                "CANCEL" | "cancel" | "Cancel" => return None,
                _ => {
                  println!("Please choose 'yes' or 'no.'");
                  continue;
                }
              },
              Err(e) => {
                println!("Invalid repsonse: {}", e);
                continue;
              }
            }
          };
          let day = loop {
            let mut day_choice = String::new();
            println!("What day?");
            let day_attempt = io::stdin().read_line(&mut day_choice);
            match day_attempt {
              Ok(_) => {
                if day_choice.trim().to_ascii_lowercase() == String::from("cancel") {
                  return None;
                }
                match day_choice.trim().parse() {
                  Ok(val) => {
                    if val > 31 || val < 1 {
                      println!("Please enter a valid day.");
                      continue;
                    } else {
                      break val;
                    }
                  }
                  Err(e) => {
                    println!("Invalid repsonse: {}", e);
                    continue;
                  }
                }
              }
              Err(e) => {
                println!("Invalid repsonse: {}", e);
                continue;
              }
            }
          };
          match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => date,
            None => {
              println!(
                "{}-{}-{} does not appear to be a valid date. Please try again.",
                year, month, day
              );
              continue;
            }
          }
        },
        "CANCEL" | "cancel" | "Cancel" => return None,
        _ => {
          println!("Invalid command.");
          continue;
        }
      };


      let client_id = match self.foreign_key.get("current_client_id") {
        Some(c) => *c,
        None => self.specify_client(String::from("note record")),
      };
      let user_id = self.current_user().id;

      match self.generate_unique_new_note_day(date, user_id, client_id) {
        Ok(note_day) => break note_day,
        Err(e) => {
          println!("Failed to generate record for notes: {}", e);
          continue;
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
      Some(dup_id) => Err(String::from("A note record already exists for that client on the given date.")),
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
    note_days.sort_by(|a, b| a.date.cmp(&b.date));
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
    self.write_note_days().unwrap();
  }
  fn choose_delete_note_day(&mut self) {
    loop {
      self.display_delete_note_day();
      println!("Are you sure you want to delete all records for {} from {}?", self.current_client().full_name(), self.current_note_day().fmt_date_long());
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
          self.delete_current_note_day();
          break;
        }
        _ => {
          break;
        }
      }
    }
  }
  fn display_delete_note_day(&self) {
    let mut heading = String::from(" DELETE NOTE RECORD ");
    let date_string = &format!("from {} ", self.current_note_day().date.format("%m/%d/%Y"));
    let client_string = &format!("for {} ", self.current_client().full_name());
    heading.push_str(date_string);
    heading.push_str(client_string);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^146}", "-");
    println!("{:-^146}", heading);
    println!("{:-^146}", "-");
    println!(
      "{:-^20} | {:-^50} | {:-<70}",
      " Category ", " Collaterals ", " First sentence ",
    );

    println!("ITERATE OVER ALL NOTES IN THIS NOTEDAY");

    println!("{:-^146}", "-");
  }
  fn delete_current_note_day(&mut self) {
    let id = self.foreign_key.get("current_note_day_id").unwrap();
    self.note_days.retain(|nd| nd.id != *id);
    self.foreign_key.remove("current_note_day_id");
  }
  fn get_note_day_option_by_id(&self, id: u32) -> Option<&NoteDay> {
    self.note_days.iter().find(|nd| nd.id == id)
  }
  fn get_note_day_by_id(&self, id: u32) -> Option<&NoteDay> {
    self.note_days.iter().find(|nd| nd.id == id)
  }
  fn get_note_day_by_id_mut(&mut self, id: u32) -> Option<&mut NoteDay> {
    self.note_days.iter_mut().find(|nd| nd.id == id)
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
      match nt.foreign_key.get("user_id") {
        Some(u_id) => {
          nt.foreign_key["user_id"] == self.current_user().id
        },
        None => true, // because current_user_note_templates returns all for the current user
                      // current_user_personal_note_templates returns the ones they should be able to edit
      }
    ).collect()
  }
  fn current_user_personal_note_templates(&self) -> Vec<&NoteTemplate> {
    self.note_templates.iter().filter(|nt|
      match nt.foreign_key.get("user_id") {
        Some(u_id) => {
          nt.foreign_key["user_id"] == self.current_user().id
        },
        None => false, // because current_user_note_templates returns all for the current user
                      // current_user_personal_note_templates returns the ones they should be able to edit
      }
    ).collect()
  }
  fn current_user_personal_note_templates_mut(&mut self) -> Vec<&mut NoteTemplate> {
    let current_user_id = self.current_user().id;
    self.note_templates.iter_mut().filter(|nt|
      match nt.foreign_key.get("user_id") {
        Some(u_id) => {
          nt.foreign_key["user_id"] == current_user_id
        },
        None => false, // because current_user_note_templates returns all for the current user
                      // current_user_personal_note_templates_mut returns the ones they should be able to edit
      }
    ).collect()
  }
  fn display_user_note_templates(&self) {
    let heading = format!(" All note templates for {} ", &self.current_user().full_name()[..]);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^136}", "-");
    println!("{:-^136}", heading);
    println!("{:-^136}", "-");
    println!("{:-^10} | {:-^40} | {:-^80}", " ID ", " Type ", " Preview ");
    for nt in self.current_user_note_templates() {
      let mut type_string = format!("{}", nt.structure);
      if nt.custom {
        type_string.push_str(" (custom)");
      }
      println!(
        "{: ^10} | {: ^40} | {: ^80}",
        nt.id,
        type_string,
        nt.preview(),
      );
    }
    println!("{:-^136}", "-");
    println!("| {} | {}", "Choose template by ID.", "NEW / N: New template");
  }
  fn display_edit_note_templates(&self) {
    let heading = format!(" Edit note templates for {} ", &self.current_user().full_name()[..]);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^136}", "-");
    println!("{:-^136}", heading);
    println!("{:-^136}", "-");
    println!("{:-^10} | {:-^40} | {:-^80}", " ID ", " Type ", " Preview ");
    for nt in self.current_user_personal_note_templates() {
      let mut type_string = format!("{}", nt.structure);
      if nt.custom {
        type_string.push_str(" (custom)");
      }
      println!(
        "{: ^10} | {: ^40} | {: ^80}",
        nt.id,
        type_string,
        nt.preview(),
      );
    }
    println!("{:-^136}", "-");
    println!("Choose template to edit by ID.");
  }
  fn choose_edit_note_templates(&mut self) {
    loop {
      self.display_edit_note_templates();
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
        _ => match field_to_edit.parse() {
            Ok(num) => match self.get_note_template_option_by_id(num) {
              Some(nt) => {
                if self.current_user_personal_note_templates().iter().any(|no_t| no_t.id == num ) {
                  match self.load_note_template(num) {
                    Ok(_) => self.choose_edit_note_template(),
                    Err(e) => panic!("Failed to load a note template by ID listed in the current user's note templates."),
                  }
                } else {
                  println!("Please choose from among the listed note templates.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              },
              None => {
                println!("No note template for id '{}'", field_to_edit);
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
            },
            Err(e) => {
              println!("Unable to parse {} to int: {}", field_to_edit, e);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            },
          }
        },
      }
    }
  }
  fn display_edit_note_template() {
    let current_nt = self.current_note_template();
    current_nt.display_content();
    println!(
      "{} | {} | {}",
      "STRUCTURE / S: Edit structure type",
      "BLANKS / B: Edit blanks",
      "CONTENT / C: Edit default content",
    );
    println!("Choose blank by ID to delete or change it to a different type of blank.");
  }
  fn choose_edit_note_template(&mut self) {
    loop {
      self.display_edit_note_template();
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
        "quit" | "q" | "QUIT" | "Q" | "Quit" => break,
        "structure" | "s" | "STRUCTURE" | "S" | "Structure" => {
          let structure = loop {
            self.display_structure_types();
            println!("Choose a new structure for the selected note template from the menu.");
            println!("Enter 'CANCEL' at any time to cancel.");
            let mut structure_choice = String::new();
            let structure_attempt = io::stdin().read_line(&mut structure_choice);
            match structure_attempt {
              Ok(_) => (),
              Err(e) => {
                println!("Invalid repsonse: {}", e);
                continue;
              }
            }
            let structure = structure_choice.trim();
            break match &structure[..] {
              "1" | "CPM" | "cpm" | "Cpm" | "Care Plan Meeting" | "Care plan meeting" | "CARE PLAN MEETING" | "care plan meeting" => Some(CarePlan),
              "2" | "CPM-V" | "cpm-v" | "Cpm-v" | "Care Plan Meeting Verbose" | "Care plan meeting verbose" | "CARE PLAN MEETING VERBOSE" | "care plan meeting verbose" => Some(CarePlanVerbose),
              "3" | "INTAKE" | "intake" | "Intake" | "I" | "i" => Some(Intake),
              "4" | "ASSESSMENT" | "assessment" | "Assessment" | "A" | "a" => Some(Assessment),
              "5" | "Sncd" | "sncd" | "SNCD" | "Strengths, Needs and Cultural Discovery" | "Strengths, needs and cultural discovery" | "S" | "s" => Some(Sncd),
              "6" | "Home Visit" | "home visit" | "Home visit" | "HV" | "hv" | "Hv" => Some(HomeVisit),
              "7" | "Agenda Prep" | "Agenda prep" | "agenda prep" | "AGENDA PREP" | "AP" | "ap" | "Ap" => Some(AgendaPrep),
              "8" | "Debrief" | "debrief" | "DEBRIEF" | "D" | "d" => Some(Debrief),
              "9" | "Phone call" | "Phone Call" | "PHONE CALL" | "phone call" | "PC" | "pc" => Some(PhoneCall),
              "10" | "Scheduling" | "scheduling" | "SCHEDULING" | "sch" | "Sch" | "SCH" => Some(Scheduling),
              "11" | "Sent email" | "Sent Email" | "SENT EMAIL" | "sent email" | "SE" | "se" | "Se" => Some(SentEmail),
              "12" | "Referral" | "REFERRAL" | "referral" | "R" | "r" => Some(Referral),
              "13" | "Custom" | "CUSTOM" | "custom" | "C" | "c" => Some(CustomStructure),
              "cancel" | "CANCEL" | "Cancel" => None,
              _ => {
                println!("Invalid choice.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              },
            }
          };
          match structure {
            Some(s) => self.current_note_template_mut().structure = s,
            None => continue,
          }
        },
        "blanks" | "b" | "BLANKS" | "B" | "Blank" => {
          let mut blank_focus_id: Option<u32> = Some(1);
          let content_focus_id: Option<u32> = None;
          loop {
            self.current_note_template().display_edit_content(blank_focus_id);
            println!(
              "{} | {}",
              "EDIT / E: Edit selected blank type",
              "DELETE / D: Delete blank",
            );
            println!("Choose blank by blank ID.");
            println!("Enter 'QUIT / Q' at any time to return to the editing menu.");
            let mut blank_choice = String::new();
            let blank_attempt = io::stdin().read_line(&mut blank_choice);
            match blank_attempt {
              Ok(_) => (),
              Err(e) => {
                println!("Invalid repsonse: {}", e);
                continue;
              }
            }
            let blank = blank_choice.trim();
            match &blank[..] {
              "QUIT" | "quit" | "Quit" | "Q" | "q" => {
                break;
              },
              "EDIT" | "edit" | "Edit" | "E" | "e" => {
                display_blanks();
                let b_idx_opt = choose_blanks_option();
                let b = match b_idx_opt {
                  None => break,
                  Some(b_idx) => {
                    Blank::vector_of_variants()[b_idx]
                  }
                };
                
                // use RE in lazystatic to find all matches, then return the nth one where n is the index of the current blank
                // (focus id - 1)

                // then replace the content with content around the match and blank in the middle.

              },
              "DELETE" | "delete" | "Delete" | "D" | "d" => {
                // delete blank, find all matches and remove the current one
              },
              _ => {
                // parse to int and find that blank
              }
            }
          }
          // we want to edit by blank like one would for a note, but also by the actual content.
          // So iterate through the text portions and the spaces between them.
          // Then break text portions up into sentences and let them choose from those.


          
          let blank_idx = choose_blanks();
          content.push_str(&format!("{}", Blank::vector_of_variants()[blank_idx]));
          display_content.push_str(&format!(" [ {} ]", Blank::vector_of_variants()[blank_idx].display_to_user()));


        },
        "content" | "c" | "CONTENT" | "C" | "Content" => {

        },
        _ => {
          
        },
      }



      let mut content = String::new();
      let mut display_content = String::new();
      loop {
        let display_content_vec = NoteTemplate::get_display_content_vec_from_string(display_content.clone());
        NoteTemplate::display_content_from_vec(display_content_vec);
        println!("Add text or blank?");
        println!("{} | {} | {} ", "TEXT / T: add custom text", "BLANK / B: add custom blank", "SAVE / S: finish and save template");
        let mut custom_choice = String::new();
        let custom_attempt = io::stdin().read_line(&mut custom_choice);
        match custom_attempt {
          Ok(_) => (),
          Err(e) => {
            println!("Invalid repsonse: {}", e);
            continue;
          }
        }
        let custom = custom_choice.trim();
        match &custom[..] {
          "TEXT" | "text" | "Text" | "T" | "t" => {
            loop {
              println!("Enter custom text as you would like it to appear.");
              let mut text_choice = String::new();
              let text_attempt = io::stdin().read_line(&mut text_choice);
              match text_attempt {
                Ok(_) => {
                  content.push_str(&format!("{}{}", &text_choice.trim()[..], " "));
                  display_content.push_str(&format!("{}{}", &text_choice.trim()[..], " "));
                  continue;
                },
                Err(e) => {
                  println!("Invalid repsonse: {}", e);
                  continue;
                }
              }
            }
          },
        }

    }
  }

  // also need choose_edit_note_template() which is really more like the match statements in above function
  // templates just lets you pick which one, really
  // fields to allow for edit are just structure and content
  // and those won't affect any ideas


  fn choose_note_templates(&mut self) {
    loop {
      let input = loop {
        self.display_user_note_templates();
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
          let maybe_new_id = self.create_note_template_get_id();
          match maybe_new_id {
            Some(_) => (),
            None => (),
          }
          continue;
        },
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          self.choose_edit_note_templates();
          continue;
        },
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_note_templates()
              .iter()
              .any(|&nt| nt.id == num) {
                println!("Please choose from among the listed templates, or 'NEW / N' to create a new template.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note_template(num) {
              Ok(_) => self.choose_note_template(),
              Err(e) => {
                println!("Unable to load template with ID {}: {}", num, e);
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
  fn select_note_template(&mut self) -> Option<u32> {
    loop {
      let input = loop {
        self.display_user_note_templates();
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
          let maybe_new_id = self.create_note_template_get_id();
          match maybe_new_id {
            Some(_) => (),
            None => (),
          }
          continue;
        },
        "EDIT" | "edit" | "Edit" | "e" | "E" => {
          self.choose_edit_note_templates();
          continue;
        },
        "QUIT" | "quit" | "Quit" | "q" | "Q" => {
          break None;
        },
        _ => match input.parse() {
          Ok(num) => {
            if !self.current_user_note_templates()
              .iter()
              .any(|&nt| nt.id == num) {
                println!("Please choose from among the listed templates, or 'NEW / N' to create a new template.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
            match self.load_note_template(num) {
              Ok(_) => break Some(num),
              Err(e) => {
                println!("Unable to load template with ID {}: {}", num, e);
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
  fn choose_note_template(&mut self) {
    loop {

      // self.display_note_template();
      println!("      // self.display_note_template();");

      println!(
        "| {} | {} | {} | {}",
        "USE / U: Use this template to create a new note",
        "CUSTOM / C: use template to create new custom template",
        "DELETE: delete custom template",
        "QUIT / Q: quit menu"
      );
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
          // self.choose_delete_note();
          println!("          // self.choose_delete_note();");
          break;
        }
        "CUSTOM" | "custom" | "Custom" | "C" | "c" => {
          // self.choose_edit_note();
          println!("          // self.choose_edit_note();");
        }
        "USE" | "use" | "Use" | "U" | "u" => {
          // let n_id = self.create_note_get_id();
          println!("          let n_id = self.create_note_get_id();");
        }
        _ => println!("Invalid command."),
      }
    }
  }
  fn display_structure_types(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^96}", "-");
    println!("{:-^10} | {:-^40} | {:-^40}", " ID ", " Template type ", " Abbreviation ");
    println!("{:-^96}", "-");
    for (i, st) in StructureType::iterator().enumerate() {
      println!("{:-^10} | {:-^40} | {:-^40}", i, st, st.abbreviate());
    }
    println!("{:-^96}", "-");
    println!("| {}", "Choose template type by name, ID, or abbreviation.");
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
    let note_template = loop {
      let structure = loop {
        self.display_structure_types();
        println!("Enter 'CANCEL' at any time to cancel.");
        println!("Build a template for what kind of record?");
        let mut structure_choice = String::new();
        let structure_attempt = io::stdin().read_line(&mut structure_choice);
        match structure_attempt {
          Ok(_) => (),
          Err(e) => {
            println!("Invalid repsonse: {}", e);
            continue;
          }
        }
        let structure = structure_choice.trim();
        break match &structure[..] {
          "1" | "CPM" | "cpm" | "Cpm" | "Care Plan Meeting" | "Care plan meeting" | "CARE PLAN MEETING" | "care plan meeting" => CarePlan,
          "2" | "CPM-V" | "cpm-v" | "Cpm-v" | "Care Plan Meeting Verbose" | "Care plan meeting verbose" | "CARE PLAN MEETING VERBOSE" | "care plan meeting verbose" => CarePlanVerbose,
          "3" | "INTAKE" | "intake" | "Intake" | "I" | "i" => Intake,
          "4" | "ASSESSMENT" | "assessment" | "Assessment" | "A" | "a" => Assessment,
          "5" | "Sncd" | "sncd" | "SNCD" | "Strengths, Needs and Cultural Discovery" | "Strengths, needs and cultural discovery" | "S" | "s" => Sncd,
          "6" | "Home Visit" | "home visit" | "Home visit" | "HV" | "hv" | "Hv" => HomeVisit,
          "7" | "Agenda Prep" | "Agenda prep" | "agenda prep" | "AGENDA PREP" | "AP" | "ap" | "Ap" => AgendaPrep,
          "8" | "Debrief" | "debrief" | "DEBRIEF" | "D" | "d" => Debrief,
          "9" | "Phone call" | "Phone Call" | "PHONE CALL" | "phone call" | "PC" | "pc" => PhoneCall,
          "10" | "Scheduling" | "scheduling" | "SCHEDULING" | "sch" | "Sch" | "SCH" => Scheduling,
          "11" | "Sent email" | "Sent Email" | "SENT EMAIL" | "sent email" | "SE" | "se" | "Se" => SentEmail,
          "12" | "Referral" | "REFERRAL" | "referral" | "R" | "r" => Referral,
          "13" | "Custom" | "CUSTOM" | "custom" | "C" | "c" => CustomStructure,
          "cancel" | "CANCEL" | "Cancel" => return None,
          _ => {
            println!("Invalid choice.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          },
        }
      };
      let mut content = String::new();
      let mut display_content = String::new();
      loop {
        let display_content_vec = NoteTemplate::get_display_content_vec_from_string(display_content.clone());
        NoteTemplate::display_content_from_vec(display_content_vec);
        println!("Add text or blank?");
        println!("{} | {} | {} ", "TEXT / T: add custom text", "BLANK / B: add custom blank", "SAVE / S: finish and save template");
        let mut custom_choice = String::new();
        let custom_attempt = io::stdin().read_line(&mut custom_choice);
        match custom_attempt {
          Ok(_) => (),
          Err(e) => {
            println!("Invalid repsonse: {}", e);
            continue;
          }
        }
        let custom = custom_choice.trim();
        match &custom[..] {
          "TEXT" | "text" | "Text" | "T" | "t" => {
            loop {
              println!("Enter custom text as you would like it to appear.");
              let mut text_choice = String::new();
              let text_attempt = io::stdin().read_line(&mut text_choice);
              match text_attempt {
                Ok(_) => {
                  content.push_str(&format!("{}{}", &text_choice.trim()[..], " "));
                  display_content.push_str(&format!("{}{}", &text_choice.trim()[..], " "));
                  continue;
                },
                Err(e) => {
                  println!("Invalid repsonse: {}", e);
                  continue;
                }
              }
            }
          },
          "BLANK" | "blank" | "Blank" | "B" | "b" => {
            let blank_idx = choose_blanks();
            content.push_str(&format!("{}", Blank::vector_of_variants()[blank_idx]));
            display_content.push_str(&format!(" [ {} ]", Blank::vector_of_variants()[blank_idx].display_to_user()));
            continue;
          },
          "SAVE" | "save" | "Save" | "S" | "s" => {
            if display_content.len() == 0 {
              println!("You must add at least one field to the content of a template. Please add either some text or at least one blank.");
              thread::sleep(time::Duration::from_secs(4));
              continue;
            }
            break;
          },
          "CANCEL" | "cancel" | "Cancel" => return None,
          _ => {
            println!("Invalid command.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
        }
      };

      match self.generate_unique_new_note_template(structure, content, self.current_user().id) {
        Ok(nt) => break nt,
        Err(e) => {
          println!("Failed to generate template: {}", e);
          thread::sleep(time::Duration::from_secs(3));
          continue;
        }
      }
    };

    let id = note_template.id;
    self.save_note_template(note_template);
    Some(id)
  }
  fn note_template_dup_id_option(&self, structure: &StructureType, content: String, user_id: u32) -> Option<u32> {
    let template_fields: Vec<(&StructureType, &str, u32, u32)> = self
      .note_templates
      .iter()
      .map(|nt| (&nt.structure, &nt.content[..], nt.foreign_key["user_id"], nt.id))
      .collect();

    match template_fields
      .iter()
      .find(|(s, c, u, _)| s == &structure && c == &&content[..] && u == &user_id) {
        Some(field_tup) => Some(field_tup.3),
        None => None,
      }
  }
  fn generate_unique_new_note_template(
    &mut self,
    structure: StructureType,
    content: String,
    user_id: u32,
  ) -> Result<NoteTemplate, String> {
    let id: u32 = self.note_templates.len() as u32 + 1;

    match self.note_template_dup_id_option(&structure, content.clone(), user_id) {
      Some(dup_id) => Err(String::from("A template exists for the same user with matching type and content.")),
      None => Ok(NoteTemplate::new(id, structure, true, content, Some(user_id)))
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

    for (i, def) in DEFAULT_NOTE_TEMPLATES.iter().enumerate() {
      let i = i as u32;
      let structure = match def.0 {
        "Care Plan Meeting" => CarePlan,
        "Care Plan Meeting Verbose" => CarePlanVerbose,
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
        "Custom" => CustomStructure,
        _ => {
          panic!("Support not added for reading default Structure Type from constant.");
        }
      };
      let content = String::from(def.1);
      let nt = NoteTemplate::new(i, structure, false, content, None);
      note_templates.push(nt);
    }

    for line in lines {
      let line_string = line?;

      let values: Vec<String> = line_string
        .split(" | ")
        .map(|val| val.to_string())
        .collect();

      let id: u32 = values[0].parse().unwrap();

      let structure = match &values[1][..] {
        "Care Plan Meeting" => CarePlan,
        "Care Plan Meeting Verbose" => CarePlanVerbose,
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
        "Custom" => CustomStructure,
        _ => return Err(Error::new(
          ErrorKind::Other,
          "Unsupported StructureType saved to file.",
        )),
      };

      let content = values[2].clone();

      let user_id = Some(values[3].parse().unwrap());

      let note_ids: Vec<u32> = match &values[4][..] {
        "" => vec![],
        _ => values[4]
          .split("#")
          .map(|val| val.parse().unwrap())
          .collect(),
      };
      let nt = NoteTemplate::new(id, structure, true, content, user_id);
      note_templates.push(nt);
    }
    note_templates.sort_by(|a, b| a.id.cmp(&b.id));
    note_templates.sort_by(|a, b|
      match (&a.foreign_key.get("user_id"), &b.foreign_key.get("user_id")) {
        (Some(anum), Some(bnum)) => anum.cmp(&bnum),
        _ => b.foreign_key.get("user_id").cmp(&a.foreign_key.get("user_id")),
      }
    );
    note_templates.sort_by(|a, b| a.structure.cmp(&b.structure));
    Ok(note_templates)
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

    let pos = self.note_templates.binary_search_by(|nt| nt.structure.cmp(&note_template.structure)
      .then_with(|| match (&nt.foreign_key.get("user_id"), &note_template.foreign_key.get("user_id")) {
        (Some(anum), Some(bnum)) => anum.cmp(&bnum),
        _ => note_template.foreign_key["user_id"].cmp(&nt.foreign_key["user_id"]),
      } )
      .then_with(|| nt.id.cmp(&note_template.id))
    ).unwrap_or_else(|e| e);

    self.note_templates.insert(pos, note_template);
    self.write_note_templates().unwrap();
  }
  fn current_user_custom_note_templates(&self) -> Vec<&NoteTemplate> {
    self.note_templates.iter().filter(|nt| nt.custom ).filter(|nt| nt.foreign_key["user_id"] == self.current_user().id).collect()
  }
  fn current_user_custom_note_templates_mut(&mut self) -> Vec<&mut NoteTemplate> {
    let current_id = self.current_user().id;
    self.note_templates.iter_mut().filter(|nt| nt.custom ).filter(|nt| nt.foreign_key["user_id"] == current_id).collect()
  }
  fn choose_delete_note_template(&mut self) {
    loop {
      self.display_delete_note_template();
      println!("Are you sure you want to delete this note template?");
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
          self.delete_current_note_template();
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
    println!("{:-^146}", "-");
    println!("{:-^146}", heading);
    println!("{:-^146}", "-");

    self.current_note_template().display_content();

    println!("{:-^146}", "-");
  }
  fn delete_current_note_template(&mut self) {
    let id = self.foreign_key.get("current_note_template_id").unwrap();
    self.note_templates.retain(|nd| nd.id != *id);
    self.foreign_key.remove("current_note_template_id");
  }
  fn get_note_template_option_by_id(&self, id: u32) -> Option<&NoteTemplate> {
    self.note_templates.iter().find(|nt| nt.id == id)
  }
  fn get_note_template_option_by_id_mut(&mut self, id: u32) -> Option<&mut NoteTemplate> {
    self.note_templates.iter_mut().find(|nt| nt.id == id)
  }

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
  fn current_note_day_notes(&self) -> Vec<&Note> {
    self.notes.iter().filter(|n| self.current_note_day().foreign_keys["note_ids"].iter().any(|n_id| n_id == &n.id )).collect()
  }
  fn get_note_day_by_note_id(&self, id: u32) -> Option<&NoteDay> {
    self.note_days.iter().find(|nd| nd.foreign_keys["note_ids"].iter().any(|n_id| n_id == &id) )
  }
  fn get_note_template_by_note_id(&self, id: u32) -> Option<&NoteTemplate> {
    self.note_templates.iter().find(|nd| nd.foreign_keys["note_ids"].iter().any(|n_id| n_id == &id) )
  }
  fn display_note_sentences(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");

    let n = self.current_note();
    let nd = self.get_note_day_by_note_id(n.id).unwrap();
    let c = self.get_client_by_note_day_id(nd.id).unwrap();
    let nt = self.get_note_template_by_note_id(n.id).unwrap();
    let heading = format!("{} {} note for {}", nd.fmt_date(), n.structure, c.full_name());
    println!("{:-^163}", heading);
    println!("{:-^50} | {:-^50} | {:-^50}", " Author ", " Template ", " Category ");
    println!("{:-^50} | {:-^50} | {:-^50}", self.current_user().name_and_title(), nt.display_short(), n.category);
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let mut current_i = 0;
    for (i, cont) in n.get_display_content_vec() {
      let display_i = if i == current_i {
        String::from("   ")
      } else {
        format!(" {} ", i)
      };
      println!("{:-^20} | {:-^140}", display_i, cont);
      current_i = i;
    }
    println!("{} | {} | {}", " EDIT / E: Edit entry ", " DELETE / D: Delete entry ", " CANCEL / C: Cancel");
  }
  fn display_note_sentences_and_blanks(&self, focus_id: Option<u32>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");

    let n = self.current_note();
    let nd = self.get_note_day_by_note_id(n.id).unwrap();
    let c = self.get_client_by_note_day_id(nd.id).unwrap();
    let nt = self.get_note_template_by_note_id(n.id).unwrap();
    let heading = format!("{} {} note for {}", nd.fmt_date(), n.structure, c.full_name());
    println!("{:-^163}", heading);
    println!("{:-^50} | {:-^50} | {:-^50}", " Author ", " Template ", " Category ");
    println!("{:-^50} | {:-^50} | {:-^50}", self.current_user().name_and_title(), nt.display_short(), n.category);
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let mut current_i = 0;
    for (i, cont) in n.get_display_content_vec_and_blanks(focus_id) {
      let display_i = if i == current_i {
        String::from("   ")
      } else {
        format!(" {} ", i)
      };
      println!("{:-^20} | {:-^140}", display_i, cont);
      current_i = i;
    }
    println!(
      "| {} \n| {} \n| {} \n| {} \n| {} \n| {}",
      "Press ENTER to add data to the currently selected blank (indicated by '[===|   |===]').",
      "Enter ID of blank to skip directly to that blank.",
      "SKIP / S: Skip to the next blank",
      "DELETE / D: Delete data from current blank",
      "CLEAR / C: Delete data from all blanks",
      "QUIT / Q: Save progress and quit",
    );
  }
  fn display_note(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");

    let n = self.current_note();
    let nd = self.get_note_day_by_note_id(n.id).unwrap();
    let c = self.get_client_by_note_day_id(nd.id).unwrap();

    let nt_string = match self.get_note_template_by_note_id(n.id) {
      Some(nt) => nt.display_short(),
      None => String::from("n/a"),
    };

    let heading = format!("{} {} note for {}", nd.fmt_date(), n.structure, c.full_name());
    println!("{:-^163}", heading);
    println!("{:-^50} | {:-^50} | {:-^50}", " Author ", " Template ", " Category ");
    println!("{:-^50} | {:-^50} | {:-^50}", self.current_user().name_and_title(), nt_string, n.category);
    println!("{:-^163}", "-");
    println!("{:-^163}", " Content ");
    println!("{:-^163}", "-");
    println!("{}", n.generate_display_content_string());
    println!("{:-^163}", "-");
  }
  fn display_new_note(&self, n: &Note) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let nd = self.current_note_day();
    let c = self.current_client();
    println!("{:-^163}", "-");
    let heading = format!("{} - {} - {} {} note for {}", self.current_user().full_name(), n.category, nd.fmt_date(), n.structure, c.full_name());
    println!("{:-^163}", heading);
    println!("{:-^163}", "-");
    println!("{:-^163}", " Content ");
    println!("{:-^163}", "-");
    println!("{}", n.generate_display_content_string());
    println!("{:-^163}", "-");
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
    println!("{} | {} | {}", " EDIT / E: Edit entry ", " DELETE / D: Delete entry ", " QUIT / Q: Quit menu");
    loop {
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
          self.choose_delete_note();
          break;
        }
        "EDIT" | "edit" | "Edit" | "E" | "e" => {
          // self.choose_edit_note();
          println!("self.choose_edit_note();");
        }
        _ => println!("Invalid command."),
      }
    }
  }
  fn display_blank_fill_in(blank_type: Blank) {
    let display_category = match blank_type {
      InternalDocument => String::from("internal document"),
      ExternalDocument => String::from("external document"),
      InternalMeeting => String::from("Wraparound meeting title"),
      ExternalMeeting => String::from("external meeting title"),
      Action => String::from("general action"),
      Phrase => String::from("other phrase"),
      _ => panic!("Incompatible blank fill in string passed to fn 'display_blank_fill_in'")
    };
    let fill_ins: Vec<String> = match blank_type {
      InternalDocument => {
        InternalDocumentFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      ExternalDocument => {
        ExternalDocumentFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      InternalMeeting => {
        InternalDocumentFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      ExternalMeeting => {
        ExternalMeetingFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      Action => {
        ActionFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      Phrase => {
        PhraseFillIn::iterator_of_blanks().map(|b| format!("{}", b) ).collect()
      },
      _ => panic!("Incompatible blank fill in string passed to fn 'display_blank_fill_in'")
    };
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^113}", "-");
    println!("{:-^113}", format!(" Fill in blank with {} ", display_category));
    println!("{:-^113}", "-");
    println!("{:-^10} | {:-^100}", " ID ", " Content ");
    for (i, fi) in fill_ins.iter().enumerate() {
      println!("{:-^10} | {:-^100}", i, fi);
    }
    println!("{:-^113}", "-");
    println!("{} | {}", " Select content to add to blank by ID. ", " CANCEL / C: Cancel and return to editing note");
  }
  fn choose_blank_fill_in(blank_type: Blank) -> Option<String> {
    loop {
      NoteArchive::display_blank_fill_in(blank_type);
      let mut fill_in_choice = String::new();
      let fill_in_attempt = io::stdin().read_line(&mut fill_in_choice);
      let selected_option = match fill_in_attempt {
        Ok(_) => fill_in_choice.trim().to_ascii_lowercase(),
        Err(e) => {
          println!("Failed to read line: {}", e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      };
      match &selected_option[..] {
        "cancel" | "c" => return None,
        _ => (),
      }
      let chosen_id: usize = match selected_option.parse() {
        Ok(num) => num,
        Err(e) => {
          println!("Failed to parse '{}' as int: {}", selected_option, e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      };
      let selected_content: String = match blank_type {
        InternalMeeting => {
          match InternalMeetingFillIn::iterator_of_blanks().enumerate().find(|(i, f)| i == &chosen_id ) {
            Some(b_tup) => format!("{}", b_tup.1),
            None => {
              println!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        ExternalMeeting => {
          match ExternalMeetingFillIn::iterator_of_blanks().enumerate().find(|(i, f)| i == &chosen_id ) {
            Some(b_tup) => format!("{}", b_tup.1),
            None => {
              println!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        InternalDocument => {
          match InternalDocumentFillIn::iterator_of_blanks().enumerate().find(|(i, f)| i == &chosen_id ) {
            Some(b_tup) => format!("{}", b_tup.1),
            None => {
              println!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        ExternalDocument => {
          match ExternalDocumentFillIn::iterator_of_blanks().enumerate().find(|(i, f)| i == &chosen_id ) {
            Some(b_tup) => format!("{}", b_tup.1),
            None => {
              println!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        Action => {
          match ActionFillIn::iterator_of_blanks().enumerate().find(|(i, f)| i == &chosen_id ) {
            Some(b_tup) => format!("{}", b_tup.1),
            None => {
              println!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        Phrase => {
          match PhraseFillIn::iterator_of_blanks().enumerate().find(|(i, f)| i == &chosen_id ) {
            Some(b_tup) => format!("{}", b_tup.1),
            None => {
              println!("Index '{}' not found. Please select content from among the listed options.", chosen_id);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        },
        _ => panic!("Incompatible fill in type passed to fn 'choose_phrase_flll_in'"),
      };
      return Some(selected_content)
    }
  }
  fn display_icc_note_categories() {
    println!("{:-^58}", "-");
    println!("{:-^58}", " ICC Note Categories ");
    println!("{:-^58}", "-");
    println!("{: ^5} | {:-^50}", " ID ", " Note category ");
    for (i, cat) in ICCNoteCategory::iterator().enumerate() {
      println!("{: ^5} | {:-<50}", i, cat);
    }
    println!("{:-^58}", "-");
    println!(
      "| {} | {}",
      "Enter ID to select a note category.",
      "QUIT / Q: Quit menu",
    );
  }
  fn display_fp_note_categories() {
    println!("{:-^58}", "-");
    println!("{:-^58}", " FP Note Categories ");
    println!("{:-^58}", "-");
    println!("{:-^5} | {:-^50}", " ID ", " Note category ");
    for (i, cat) in FPNoteCategory::iterator().enumerate() {
      println!("{:-^5} | {:-<50}", i, cat);
    }
    println!("{:-^58}", "-");
    println!(
      "| {} | {}",
      "Enter ID to select a note category.",
      "QUIT / Q: Quit menu",
    );
  }
  fn choose_note_category(&self) -> Option<NoteCategory> {
    let current_role = match self.current_user().role {
      ICC => ICC,
      FP => FP,
    };
    loop {
      match current_role {
        ICC => {
          NoteArchive::display_icc_note_categories();
        },
        FP => {
          NoteArchive::display_fp_note_categories();
        },
      }

      let mut ncat_choice = String::new();
      let ncat_attempt = io::stdin().read_line(&mut ncat_choice);
      let ncat_input = match ncat_attempt {
        Ok(_) => ncat_choice.trim().to_ascii_lowercase(),
        Err(e) => {
          println!("Failed to read input.");
          thread::sleep(time::Duration::from_secs(1));
          continue;
        }
      };
      match &ncat_input[..] {
        "quit" | "q" => return None,
        _ => match ncat_input.parse::<usize>() {
          Err(e) => {
            println!("Failed to parse input as a number: {}. Try again.", e);
            thread::sleep(time::Duration::from_secs(1));
            continue;
          },
          Ok(num) => {
            match current_role {
              ICC => {
                let iccncat = ICCNoteCategory::iterator().nth(num);
                match iccncat {
                  Some(icccat) => return Some(ICCNote(icccat)),
                  None => {
                    println!("Index out of bounds for available Note Categories. Please identify a note category by valid ID.");
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                  }
                }
              },
              FP => {
                let fpncat = FPNoteCategory::iterator().nth(num);
                match fpncat {
                  Some(fpcat) => return Some(FPNote(fpcat)),
                  None => {
                    println!("Index out of bounds for available Note Categories. Please identify a note category by valid ID.");
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                  }
                }
              },
            }
          }
        }
      }
    }
  }
  fn create_note_from_template_get_id(&mut self, nt_id: Option<u32>) -> Option<u32> {
    let nt_id = match nt_id {
      Some(id) => id,
      None => loop {
        let nt_id_opt = self.select_note_template();
        match nt_id_opt {
          Some(id) => break id,
          None => {
            let decision = loop {
              let mut cancel_choice = String::new();
              println!("You must select a note template to build a note from a template.");
              println!("Cancel writing note? ( Y / N )");
              let cancel_choice_attempt = io::stdin().read_line(&mut cancel_choice);
              break match cancel_choice_attempt {
                Ok(_) => cancel_choice.trim().to_ascii_lowercase(),
                Err(e) => {
                  println!("Failed to read line: {}", e);
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
                println!("Invalid command.");
                continue;
              },
            }
          }
        }
      }
    };
    
    let nt = match self.get_note_template_option_by_id(nt_id) {
      Some(nt) => nt,
      None => {
        panic!("Failed to load a note template with the ID passed from the function that creates new Note Templates.");
      }
    };
    let nst = nt.structure;
    let ncnt = nt.content.clone();

    let ncat = match self.current_user().role {
      ICC => {
        match nst {
          CarePlan => ICCNote(FaceToFaceContactWithClient),
          CarePlanVerbose => ICCNote(FaceToFaceContactWithClient),
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
          CustomStructure => {
            loop {
              match self.choose_note_category() {
                Some(ncat) => break ncat,
                None => {
                  let decision = loop {
                    let mut cancel_choice = String::new();
                    println!("You must select a note category to fill in a custom note template.");
                    println!("Cancel writing note? ( Y / N )");
                    let cancel_choice_attempt = io::stdin().read_line(&mut cancel_choice);
                    let cancel_choice_content = match cancel_choice_attempt {
                      Ok(_) => cancel_choice.trim().to_ascii_lowercase(),
                      Err(e) => {
                        println!("Failed to read line: {}", e);
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
                      println!("Invalid command.");
                      continue;
                    },
                  }
                }
              }

            }
          }
        }
      }
      FP => {
        match nst {
          CarePlan => FPNote(Tbd),
          CarePlanVerbose => FPNote(Tbd),
          Intake => FPNote(Tbd),
          Assessment => FPNote(Tbd),
          Sncd => FPNote(Tbd),
          HomeVisit => FPNote(Tbd),
          AgendaPrep => FPNote(Tbd),
          Debrief => FPNote(Tbd),
          PhoneCall => FPNote(Tbd),
          Scheduling => FPNote(Tbd),
          SentEmail => FPNote(Tbd),
          Referral => FPNote(Tbd),
          CustomStructure => FPNote(Tbd),
        }
      }
    };

    let mut n = self.generate_note(ncat, nst, ncnt).unwrap();
    let nd = self.get_note_day_by_note_id(n.id).unwrap();
    let nd_date = nd.date;
    let nd_date_string = format!("{}, {}-{}-{}", nd_date.weekday(), nd_date.year(), nd_date.month(), nd_date.day());

    // autofill blanks that do not require user input
    // (can clearly determine output)
    let mut empty_blanks = n.get_empty_blanks_and_indexes();
    for (i, b) in empty_blanks {
      let i = i as u32;
      match b {
        CurrentUser => {
          n.blanks.insert(i, (b.clone(), format!("{}", self.current_user().role), vec![self.current_user().id]));
        }
        CurrentClientName => {
          let client = self.current_client();
          let blank_string = client.full_name_with_label();
          let id_vec = vec![client.id];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        }
        AllCollaterals => {
          let client = self.current_client();
          let collaterals = self.get_current_collaterals();

          let blank_string = if collaterals.len() > 1 {
            format!(
              "{} {} {}",
              collaterals[..collaterals.len()-1].iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", "),
              "and",
              collaterals[collaterals.len()-1].full_name_and_title(),
            )
          } else {
            collaterals[0].full_name_and_title()
          };
          let id_vec = client.foreign_keys[&String::from("collaterals")].to_owned();
          n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
          n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
        },
        Pronoun1ForUser => {
          let u = self.current_user();
          let p = self.get_pronouns_by_id(u.pronouns).unwrap();
          let blank_string = p.subject.clone();
          let id_vec = vec![u.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun2ForUser => {
          let u = self.current_user();
          let p = self.get_pronouns_by_id(u.pronouns).unwrap();
          let blank_string = p.object.clone();
          let id_vec = vec![u.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun3ForUser => {
          let u = self.current_user();
          let p = self.get_pronouns_by_id(u.pronouns).unwrap();
          let blank_string = p.possessive_determiner.clone();
          let id_vec = vec![u.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun4ForUser => {
          let u = self.current_user();
          let p = self.get_pronouns_by_id(u.pronouns).unwrap();
          let blank_string = p.possessive.clone();
          let id_vec = vec![u.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun1ForClient => {
          let c = self.current_client();
          let p = self.get_pronouns_by_id(c.pronouns).unwrap();
          let blank_string = p.subject.clone();
          let id_vec = vec![c.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun2ForClient => {
          let c = self.current_client();
          let p = self.get_pronouns_by_id(c.pronouns).unwrap();
          let blank_string = p.object.clone();
          let id_vec = vec![c.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun3ForClient => {
          let c = self.current_client();
          let p = self.get_pronouns_by_id(c.pronouns).unwrap();
          let blank_string = p.possessive_determiner.clone();
          let id_vec = vec![c.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        Pronoun4ForClient => {
          let c = self.current_client();
          let p = self.get_pronouns_by_id(c.pronouns).unwrap();
          let blank_string = p.possessive.clone();
          let id_vec = vec![c.pronouns.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        TodayDate => {
          let today = Local::now().naive_local().date();
          let blank_string = format!("{}, {}-{}-{}", today.weekday(), today.year(), today.month(), today.day());
          let id_vec = vec![];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        NoteDayDate => {
          let blank_string = nd_date_string.clone();
          let id_vec = vec![nd.id.clone()];
          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
        },
        _ => (), // all others need to be filled in based on user input
      }
    }

    let mut focus_id_option: Option<u32> = None;
    'choice: loop {

      // increments the current blank by going to the next one only when the current is filled,
      // or alternatively when focus_id_option is set to Some(id) because the user selected it

      empty_blanks = n.get_empty_blanks_and_indexes();

      let (i, b) = match focus_id_option {
        Some(f_id) => {
          let f_id_b = empty_blanks.iter().find(|b_tup| b_tup.0 == f_id ).unwrap().1;
          (f_id, f_id_b)
        },
        None => (empty_blanks[0].0, empty_blanks[0].1)
      };
      let i = i as u32;

      let choice = loop {
        let mut note_choice = String::new();
        self.display_note_sentences_and_blanks(focus_id_option);
        let note_attempt = io::stdin().read_line(&mut note_choice);
        match note_attempt {
          Ok(_) => break note_choice.trim().to_ascii_lowercase(),
          Err(e) => {
            println!("Failed to read line: {}", e);
            thread::sleep(time::Duration::from_secs(2));
            continue;
          },
        }
      };
      match &choice[..] {
        "skip" | "s" => {
          match focus_id_option {
            Some(focus_id) => {
              focus_id_option = Some(focus_id + 1);
              continue;
            },
            None => {
              if empty_blanks.len() > 1 {
                focus_id_option = Some(empty_blanks[2].0);
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
                println!("Delete blank currently filled in with '{}'?", b_tup.1);
                let delete_choice_attempt = io::stdin().read_line(&mut delete_choice);
                let delete_choice_content = match delete_choice_attempt {
                  Ok(_) => delete_choice.trim().to_ascii_lowercase(),
                  Err(e) => {
                    println!("Failed to read line: {}", e);
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
                        for (idx, blank_tup) in &n.blanks {
                          for co_id in &blank_tup.2 {
                            if !collat_ids_included_elsewhere.clone().iter().any(|id| id == co_id ) {
                              collat_ids_included_elsewhere.push(*co_id)
                            }
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
                    break;
                  },
                  "no" | "n" => {
                    break;
                  },
                  _ => {
                    println!("Invalid command.");
                    continue;
                  },
                }
              }
            },
            None => {
              println!("Current blank is already empty.");
              thread::sleep(time::Duration::from_secs(2));
              continue;
            },
          }
        },
        "clear" | "c" => {
          if n.blanks.len() == 0 {
            println!("All blanks are already empty.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          } else {
            loop {
              let mut clear_choice = String::new();
              println!("Delete all blanks from the current template and start over?");
              let clear_choice_attempt = io::stdin().read_line(&mut clear_choice);
              let clear_choice_content = match clear_choice_attempt {
                Ok(_) => clear_choice.trim().to_ascii_lowercase(),
                Err(e) => {
                  println!("Failed to read line: {}", e);
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
                  println!("Invalid command.");
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
          self.display_note_sentences_and_blanks(focus_id_option);
          if empty_blanks.len() == 0 {
            break;
          }
          match b {
            Collaterals => {
              // SKIP option, CANCEL, enter int
              // also eventually browsing collaterals and other menus to add instead of vice versa
              // probably under an 'OPTIONS' submenu
              // This can be done by collecting the indexes of the blanks that are in that category on the first iteration
              // and checking for them on every second iteration.
              let mut collats: Vec<Collateral> = vec![];
              'keep_adding: loop {
                let initial_input = loop {
                  self.display_client_collaterals();
                  let mut choice = String::new();
                  let read_attempt = io::stdin().read_line(&mut choice);
                  match read_attempt {
                    Ok(_) => break choice.trim().to_string(),
                    Err(e) => {
                      println!("Could not read input; try again ({}).", e);
                      continue;
                    }
                  }
                };
                match &initial_input[..] {
                  "NEW" | "new" | "New" | "n" | "N" => {
                    let maybe_new_id = self.create_collateral_get_id();
                    match maybe_new_id {
                      Some(new_id) => self.update_current_collaterals(new_id),
                      None => (),
                    }
                    continue;
                  },
                  "ADD" | "add" | "Add" | "a" | "A" => {
                    self.add_collateral();
                    continue;
                  },
                  "EDIT" | "edit" | "Edit" | "E" | "e" => {
                    self.choose_edit_client_collaterals();
                    continue;
                  },
                  "QUIT" | "quit" | "Quit" | "q" | "Q" => {
                    break;
                  },
                  _ => {
                    let selected_id_res: Result<u32, _> = initial_input.parse();
                    match selected_id_res {
                      Ok(num) => {
                        let collat = match self.choose_get_client_collateral(num) {
                          Some(co) => co.clone(),
                          None => continue,
                        };
                        if !collats.iter().any(|co| co == &collat ) {
                          collats.push(collat);
                        } else {
                          println!("Collateral already added to blank.");
                        }
                      },
                      Err(e) => {
                        println!("Invalid input: {}; error: {}", initial_input, e);
                        thread::sleep(time::Duration::from_secs(3));
                        continue;
                      }
                    }
                    let add_another = 'another: loop {
                      println!("Add another collateral to this blank? (y/n)");
                      let mut add_another_choice = String::new();
                      let maybe_add_another = io::stdin().read_line(&mut add_another_choice);
                      match maybe_add_another {
                        Ok(_) => match &add_another_choice.trim().to_ascii_lowercase()[..] {
                          "yes" | "y" => continue 'keep_adding,
                          "no" | "n" => break 'keep_adding,
                          _ => {
                            println!("Invalid response.");
                            thread::sleep(time::Duration::from_secs(2));
                            continue 'another;
                          }
                        },
                        Err(e) => {
                          println!("Failed to read line: {}", e);
                          continue;
                        }
                      }
                    };
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
              } else {
                collats[0].full_name_and_title()
              };
              let id_vec: Vec<u32> = collats.iter().map(|co| co.id ).collect();
              n.blanks.insert(i, (b.clone(), blank_string, id_vec.clone()));
              n.foreign_keys.insert(String::from("collateral_ids"), id_vec);
            },
            InternalDocument => {
              let blank_string = match Self::choose_blank_fill_in(InternalDocument) {
                Some(s) => s,
                None => continue,
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), blank_string, id_vec));
            },
            ExternalDocument => {
              let blank_string = match Self::choose_blank_fill_in(ExternalDocument) {
                Some(s) => s,
                None => continue,
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), blank_string, id_vec));
            },
            InternalMeeting => {
              let blank_string = match Self::choose_blank_fill_in(InternalMeeting) {
                Some(s) => s,
                None => continue,
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), blank_string, id_vec));
            },
            ExternalMeeting => {
              let blank_string = match Self::choose_blank_fill_in(ExternalMeeting) {
                Some(s) => s,
                None => continue,
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), blank_string, id_vec));
            },
            Action => {
              let blank_string = match Self::choose_blank_fill_in(Action) {
                Some(s) => s,
                None => continue,
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), blank_string, id_vec));
            },
            Phrase => {
              let blank_string = match Self::choose_blank_fill_in(Phrase) {
                Some(s) => s,
                None => continue,
              };
              let id_vec = vec![];
              n.blanks.insert(i, (b.clone(), blank_string, id_vec));
            },
            CustomBlank => {
              loop {
                let mut custom_choice = String::new();
                println!("Enter custom content for the current blank.");
                let custom_attempt = io::stdin().read_line(&mut custom_choice);
                let custom_content = match custom_attempt {
                  Ok(_) => custom_choice.trim(),
                  Err(e) => {
                    println!("Failed to read line: {}", e);
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                  }
                };
                let blank_string = custom_content.to_string();
                let id_vec = vec![];
                n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                break;
              }
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
                          let blank_string = if collat_ids.len() > 1 {
                            String::from("they")
                          } else {
                            match self.get_pronouns_by_id(collat_ids[0]) {
                              Some(p) => p.subject.clone(),
                              None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                            }
                          };
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                        },
                        AllCollaterals => {
                          let collats = self.current_client_collaterals();
                          let blank_string = if collats.len() > 1 {
                            String::from("they")
                          } else {
                            match self.get_pronouns_by_id(collats[0].id) {
                              Some(p) => p.subject.clone(),
                              None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                            }
                          };
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                        },
                        _ => panic!("A pronoun blank was connected to a type of blank for which pronouns do not apply."),
                      }
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
                          let blank_string = if collat_ids.len() > 1 {
                            String::from("they")
                          } else {
                            match self.get_pronouns_by_id(collat_ids[0]) {
                              Some(p) => p.object.clone(),
                              None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                            }
                          };
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                        },
                        AllCollaterals => {
                          let collats = self.current_client_collaterals();
                          let blank_string = if collats.len() > 1 {
                            String::from("they")
                          } else {
                            match self.get_pronouns_by_id(collats[0].id) {
                              Some(p) => p.object.clone(),
                              None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                            }
                          };
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                        },
                        _ => panic!("A pronoun blank was connected to a type of blank for which pronouns do not apply."),
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
                          let blank_string = if collat_ids.len() > 1 {
                            String::from("they")
                          } else {
                            match self.get_pronouns_by_id(collat_ids[0]) {
                              Some(p) => p.possessive_determiner.clone(),
                              None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                            }
                          };
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                        },
                        AllCollaterals => {
                          let collats = self.current_client_collaterals();
                          let blank_string = if collats.len() > 1 {
                            String::from("they")
                          } else {
                            match self.get_pronouns_by_id(collats[0].id) {
                              Some(p) => p.possessive_determiner.clone(),
                              None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                            }
                          };
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                        },
                        _ => panic!("A pronoun blank was connected to a type of blank for which pronouns do not apply."),
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
                          let blank_string = if collat_ids.len() > 1 {
                            String::from("they")
                          } else {
                            match self.get_pronouns_by_id(collat_ids[0]) {
                              Some(p) => p.possessive.clone(),
                              None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                            }
                          };
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                        },
                        AllCollaterals => {
                          let collats = self.current_client_collaterals();
                          let blank_string = if collats.len() > 1 {
                            String::from("they")
                          } else {
                            match self.get_pronouns_by_id(collats[0].id) {
                              Some(p) => p.possessive.clone(),
                              None => panic!("The selected collateral's pronouns cannot be entered due to a missing record."),
                            }
                          };
                          n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                        },
                        _ => panic!("A pronoun blank was connected to a type of blank for which pronouns do not apply."),
                      }
                    },
                    None => (), 
                  }
                }
                None => (),
              }
            },
            _ => (),
          }
        },
        _ => {
          let blank_id: usize = match choice.parse() {
            Ok(num) => num,
            Err(e) => {
              match b {
                CustomBlank => {
                  loop {
                    loop {
                      let mut fill_choice = String::new();
                      println!("Fill custom blank with '{}'? ( Y / N )", choice);
                      let fill_choice_attempt = io::stdin().read_line(&mut fill_choice);
                      let fill_choice_content = match fill_choice_attempt {
                        Ok(_) => fill_choice.trim().to_ascii_lowercase(),
                        Err(e) => {
                          println!("Failed to read line: {}", e);
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
                          println!("Invalid command.");
                          continue;
                        },
                      }
                    }
                    loop {
                      let mut custom_choice = String::new();
                      println!("Enter custom content for the current blank.");
                      let custom_attempt = io::stdin().read_line(&mut custom_choice);
                      let custom_content = match custom_attempt {
                        Ok(_) => custom_choice.trim(),
                        Err(e) => {
                          println!("Failed to read line: {}", e);
                          thread::sleep(time::Duration::from_secs(2));
                          continue;
                        }
                      };
                      let blank_string = custom_content.to_string();
                      let id_vec = vec![];
                      n.blanks.insert(i, (b.clone(), blank_string, id_vec));
                      continue 'choice;
                    }
                  }
                },
                _ => {
                  println!("Invalid command.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              }
            },
          };
          if blank_id < 0 || blank_id > n.number_of_blanks() {
            println!("Invalid blank ID.");
            thread::sleep(time::Duration::from_secs(2));
            continue;
          }
          focus_id_option = Some(blank_id as u32)
        },
      }
    }

    let note_id = n.id;
    self.notes.push(n);
    Some(note_id)
    
  }
  fn display_blank_menus() {

    let mut internal_docs = InternalDocumentFillIn::iterator_of_blanks();
    let mut external_docs = ExternalDocumentFillIn::iterator_of_blanks();
    let mut internal_meetings = InternalMeetingFillIn::iterator_of_blanks();
    let mut external_meetings = ExternalMeetingFillIn::iterator_of_blanks();
    let mut actions = ActionFillIn::iterator_of_blanks();
    let mut phrases = PhraseFillIn::iterator_of_blanks();

    let mut row_vec: Vec<Vec<String>> = vec![];

    let mut idx = 0;
    loop {
      let this_id = internal_docs.nth(0);
      let this_ed = external_docs.nth(0);
      let this_im = internal_meetings.nth(0);
      let this_em = external_meetings.nth(0);
      let this_a = actions.nth(0);
      let this_p = phrases.nth(0);
      
      match (
        this_id,
        this_ed,
        this_im,
        this_em,
        this_a,
        this_p,
      ) {
        (None, None, None, None, None, None) => break,
        _ => (),
      }

      let mut internal_docs_string = String::new();
      match this_id {
        Some(val) => internal_docs_string = format!("[{}{}] {}", val.alpha_index(), idx, val),
        None => (),
      }
      let mut external_docs_string = String::new();
      match this_ed {
        Some(val) => external_docs_string = format!("[{}{}] {}", val.alpha_index(), idx, val),
        None => (),
      }
      let mut internal_meetings_string = String::new();
      match this_im {
        Some(val) => internal_meetings_string = format!("[{}{}] {}", val.alpha_index(), idx, val),
        None => (),
      }
      let mut external_meetings_string = String::new();
      match this_em {
        Some(val) => external_meetings_string = format!("[{}{}] {}", val.alpha_index(), idx, val),
        None => (),
      }
      let mut actions_string = String::new();
      match this_a {
        Some(val) => actions_string = format!("[{}{}] {}", val.alpha_index(), idx, val),
        None => (),
      }
      let mut phrases_string = String::new();
      match this_p {
        Some(val) => phrases_string = format!("[{}{}] {}", val.alpha_index(), idx, val),
        None => (),
      }
      row_vec.push(vec![internal_docs_string, external_docs_string, internal_meetings_string, external_meetings_string, actions_string, phrases_string]);
      idx += 1;
    }

    let mut rows: HashMap<u32, Vec<String>> = HashMap::new();
    let mut row_idx = 1;
    let n_chars = 20;
    for s_vec in row_vec {
      if !s_vec.iter().any(|s| s.chars().count() > n_chars ) {
        rows.insert(row_idx, s_vec);
      } else {
        let mut new_vecs: Vec<Vec<String>> = vec![];
        // get the number of vectors needed by finding how many 20-char segments are in the longest string
        let chars_in_line = s_vec.iter().max_by(|s1, s2| s1.chars().count().cmp(&s2.chars().count()) ).unwrap().chars().count(); 
        let number_of_vecs = chars_in_line / n_chars + if chars_in_line % n_chars != 0 { 1 } else { 0 };
        for vec_i in 0..number_of_vecs {
          let idx1 = vec_i * n_chars;
          let idx2 = (vec_i * n_chars) + n_chars;
          let new_vec: Vec<String> = s_vec.iter().map(|s|
            s.chars()
              .enumerate()
              .filter(|(i, _)| i >= &idx1 && i < &idx2  )
              .map(|(_, c)| c.to_string() )
              .collect::<Vec<String>>()
              .join("")
          ).collect();
          new_vecs.push(new_vec);
        }
        // println!("{:?}", new_vecs);
        for n_vec in new_vecs {
          rows.insert(row_idx, n_vec);
          row_idx += 1;
        }
      }
      // println!("{:?}", &rows);
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^195}", "-");
    println!("{:-^195}", " All fill-ins for blanks ");
    println!("{:-^195}", "-");
    println!(
      "{:-^25} | {:-^25} | {:-^25} | {:-^25} | {:-^25} | {:-^25}",
      " Riverside documents ",
      " External documents ",
      " Riverside meetings ",
      " External meetings ",
      " General actions ",
      " Other phrases ",
    );
    
    // print value in each row
    for row_i in 1..*rows.keys().max().unwrap() {
      let i = row_i as u32;
      let (s1, s2, s3, s4, s5, s6) = (
        &rows[&i][0],
        &rows[&i][1],
        &rows[&i][2],
        &rows[&i][3],
        &rows[&i][4],
        &rows[&i][5],
      );
      println!(
        "{:-^25} | {:-^25} | {:-^25} | {:-^25} | {:-^25} | {:-^25}",
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
      );
    }

    println!("{:-^195}", "-");
    println!("Select menu item by ID (seen above in brackets).");
    println!("You may also enter the alphabetic portion to view that menu by itself (e.g., 'rm' for Riverside meetings).");
    println!(
      "| {}",
      "CANCEL / C: Cancel",
    );
  }
  fn get_blank_from_menu() -> Option<(Blank, String)> {
    'main: loop {
      NoteArchive::display_blank_menus();

      let mut buffer = String::new();
      let idx_attempt = io::stdin().read_line(&mut buffer);
      let idx = match idx_attempt {
        Ok(_) => buffer.trim().to_ascii_lowercase(),
        Err(e) => {
          println!("Failed to read line.");
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      };

      let return_value = match &idx[..] {
        "cancel" | "c" => None,
        "rd" => {
          match Self::choose_blank_fill_in(InternalDocument) {
            Some(s) => Some((InternalDocument, s)),
            None => continue,
          }
        },
        "ed" => {
          match Self::choose_blank_fill_in(ExternalDocument) {
            Some(s) => Some((ExternalDocument, s)),
            None => continue,
          }
        },
        "rm" => {
          match Self::choose_blank_fill_in(InternalMeeting) {
            Some(s) => Some((InternalMeeting, s)),
            None => continue,
          }
        },
        "em" => {
          match Self::choose_blank_fill_in(ExternalMeeting) {
            Some(s) => Some((ExternalMeeting, s)),
            None => continue,
          }
        },
        "a" => {
          match Self::choose_blank_fill_in(Action) {
            Some(s) => Some((Action, s)),
            None => continue,
          }
        },
        "p" => {
          match Self::choose_blank_fill_in(Phrase) {
            Some(s) => Some((Phrase, s)),
            None => continue,
          }
        },
        _ => {
          let selected_blank_tup: Option<(Blank, String)> = loop {
            let chars1 = idx.chars();
            let chars2 = idx.chars();
            let mut chars3 = idx.chars();

            let alpha_chars = chars1.take_while(|c| c.is_alphabetic() );
            let num_alpha = chars2.take_while(|c| c.is_alphabetic() ).count();
            for _ in 0..num_alpha {
              chars3.next();
            }
            let num_chars = chars3.take_while(|c| c.is_numeric() );

            let first_part = alpha_chars.map(|c| c.to_string() ).collect::<Vec<String>>().join("");
            let last_part = num_chars.map(|c| c.to_string() ).collect::<Vec<String>>().join("");

            if format!("{}", idx) != format!("{}{}", &first_part, &last_part) {
              println!("Invalid index syntax. If using an alphanumeric index, enter alphabetic portion followed by numeric portion.");
              thread::sleep(time::Duration::from_secs(2));
              continue 'main;
            }

            let num_result = last_part.parse();
            let num = match num_result {
              Ok(num) => num,
              Err(e) => {
                println!("Invalid index.");
                thread::sleep(time::Duration::from_secs(2));
                continue 'main;
              }
            };
            match &first_part[..] {
              "rd" => match InternalDocumentFillIn::iterator_of_blanks().nth(num) {
                Some(b) => break Some((InternalDocument, format!("{}", b))),
                None => break None,
              },
              "ed" => match ExternalDocumentFillIn::iterator_of_blanks().nth(num) {
                Some(b) => break Some((ExternalDocument, format!("{}", b))),
                None => break None,
              },
              "rm" => match InternalMeetingFillIn::iterator_of_blanks().nth(num) {
                Some(b) => break Some((InternalMeeting, format!("{}", b))),
                None => break None,
              },
              "em" => match ExternalMeetingFillIn::iterator_of_blanks().nth(num) {
                Some(b) => break Some((ExternalMeeting, format!("{}", b))),
                None => break None,
              },
              "a" => match ActionFillIn::iterator_of_blanks().nth(num) {
                Some(b) => break Some((Action, format!("{}", b))),
                None => break None,
              },
              "p" => match PhraseFillIn::iterator_of_blanks().nth(num) {
                Some(b) => break Some((Phrase, format!("{}", b))),
                None => break None,
              },
              _ => {
                println!("Invalid ID. Please use the alphanumeric IDs provided to select a phrase to add to your note.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              }
            }
          };
          match selected_blank_tup {
            Some(_) => selected_blank_tup,
            None => {
              println!("Valid ID format, but ID not found in the given list.");
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        }
      };
      break return_value
    }
  }
  fn create_note_manually_get_id(&mut self) -> Option<u32> {

    let ncat = loop {
      match self.choose_note_category() {
        Some(nc) => break nc,
        None => {
          let mut choice = String::new();
          let choice_att = io::stdin().read_line(&mut choice);
          println!("In order to enter a note manually, you must select a category.");
          println!("Do you wish to cancel writing this note?");
          match choice_att {
            Ok(_) => {
              match &choice.trim().to_ascii_lowercase()[..] {
                "y" | "yes" => return None,
                "no" | "n" => continue,
                _ => {
                  println!("Invalid entry. Please select a template or cancel to return to the previous menu.");
                  thread::sleep(time::Duration::from_secs(2));
                  continue;
                }
              }
            },
            Err(e) => {
              println!("Failed to read line: {}", e);
              thread::sleep(time::Duration::from_secs(2));
              continue;
            }
          }
        }
      }
    };
    
    let mut n = loop {
      match self.generate_note(ncat, CustomStructure, String::new()) {
        Ok(nopt) => break nopt,
        Err(e) => panic!("Failed to generate Note with custom structure and '{}' as note category: {}.", format!("{}", ncat), e),
      }
    };
    
    // prepare current client collaterals to display for entering into Note
    let collats = self.current_client_collaterals();
    let mut collats_iter = collats.iter();
    let mut col_rows: Vec<Vec<&Collateral>> = vec![];
    'adding_rows: loop {
      let mut new_vec: Vec<&Collateral> = vec![];
      for _ in 1..=4 {
        match collats_iter.next() {
          Some(col) => new_vec.push(col),
          None => {
            col_rows.push(new_vec);
            break 'adding_rows;
          }
        }
      }

      col_rows.push(new_vec.clone());
    }

    // add blanks
    let mut current_blank = 1;
    loop {
      self.display_new_note(&n);
      println!("{:-^163}", " Collaterals ");
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
        println!(
          "{:-^38} | {:-^38} | {:-^38} | {:-^38}",
          val1,
          val2,
          val3,
          val4,
        );
      println!("{:-^163}", "-");

      }
      println!("Press ENTER to choose a phrase from the saved menus.");
      println!("You may also enter text directly or choose from the following shortcuts.");
      println!(
        "| {} | {}",
        "YOUTH / Y: Youth's full name",
        "ALL / A: All collaterals",
      );
      println!(
        "| {} | {} | {}",
        "BACK / B: Delete last word or 5 characters",
        "SAVE / S: Finish writing and save to current record",
        "CANCEL / C: Cancel and discard",
      );
      let mut choice = String::new();
      let choice_att = io::stdin().read_line(&mut choice);
      match choice_att {
        Ok(_) => {
          match &choice.trim().to_ascii_lowercase()[..] {
            "" => (),
            "youth" | "y" | "YOUTH" | "Youth" | "Y" => {
              let current_client_string = &self.current_client().full_name_with_label();
              if n.content.trim_end() != n.content {
                n.content.push_str(&current_client_string[..]);
              } else {
                n.content.push_str(&format!("{}{}", " ", &current_client_string[..])[..]);
              }
              continue;
            },
            "all" | "a" | "ALL" | "All" | "A" => {
              let current_collaterals = self.current_client_collaterals();
              let num_collats = current_collaterals.len();
              let all_collaterals_string = if num_collats == 0 {
                println!("No collaterals are saved for the current client.");
                thread::sleep(time::Duration::from_secs(2));
                continue;
              } else if num_collats == 1 {
                current_collaterals[0].full_name_and_title()
              } else if num_collats > 1 {
                let part1 = current_collaterals[..num_collats-2].to_owned().iter().map(|co| co.full_name_and_title() ).collect::<Vec<String>>().join(", ");
                let part2 = current_collaterals[num_collats-1].full_name_and_title();
                format!("{} and {}", part1, part1)
              } else {
                // else condition is impossible because vec must have positive length
                String::from("")
              };
              if n.content.trim_end() != n.content {
                n.content.push_str(&all_collaterals_string[..]);
              } else {
                n.content.push_str(&format!("{}{}", " ", &all_collaterals_string[..])[..]);
              }
              continue;
            },
            "back" | "b" | "BACK" | "Back" | "B" => {

              n.content = n.content.trim().to_string();

              let last_space = n.content.rfind(' ');

              match last_space {
                None => {
                  if n.content.chars().count() > 5 {
                    let end_index: usize = n.content.chars().count() - 5;
                    n.content = String::from(&n.content[..end_index]);
                  } else {
                    n.content = String::new();
                  }
                },
                Some(idx) => n.content = n.content[..idx].to_string(),
              }
              for co_id in n.foreign_keys["collateral_ids"].clone() {
                if !n.content.contains(&self.get_collateral_by_id(co_id).unwrap().full_name()) {
                  let new_ids: Vec<u32> = n.foreign_keys["collateral_ids"].clone().iter()
                    .map(|num| *num )
                    .filter(|saved_co_id|
                      saved_co_id != &co_id
                    ).collect();

                  n.foreign_keys.insert(String::from("collateral_ids"), new_ids);
                }
              }
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
                  if !self.current_user_collaterals().iter().any(|col| col.id == *num ) {
                    ()
                  } else {
                    match self.get_collateral_by_id(*num) {
                      Some(collat) => {
                        let collateral_display_string = collat.full_name_and_title();
                        if n.content.trim_end() != n.content {
                          n.content.push_str(&collateral_display_string[..]);
                        } else {
                          n.content.push_str(&format!("{}{}", " ", &collateral_display_string[..])[..]);
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
              if n.content.trim_end() != n.content || String::from(";:,.'\"!@#$%^*)`]}-_+=>?/").contains(&choice.trim()[..]) || String::new() == n.content {
                n.content.push_str(&choice.trim()[..]);
              } else {
                n.content.push_str(&format!("{}{}", " ", choice.trim())[..]);
              }
              continue;
            }
          }
        },
        Err(e) => {
          println!("Failed to read line: {}", e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      }

      let b_opt = NoteArchive::get_blank_from_menu();
      let (blank, blank_fill) = match b_opt {
        Some((b, bf)) => (b, bf),
        None => continue,
      };
      n.blanks.insert(current_blank, (blank, blank_fill, vec![]));
      n.content.push_str(&format!(" {}", blank));
      current_blank += 1;
      continue;
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
      println!(
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
          println!("Failed to read line: {}", e);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      };
      match &input_val[..] {
        "template" | "t" => return self.create_note_from_template_get_id(None),
        "scratch" | "s" => return self.create_note_manually_get_id(),
        "cancel" => return None,
        _ => {
          println!("Invalid input: {}", input_val);
          thread::sleep(time::Duration::from_secs(2));
          continue;
        }
      }
    }
  }
  fn generate_note(
    &mut self,
    category: NoteCategory,
    structure: StructureType,
    content: String,
  ) -> Result<Note, String> {
    let id: u32 = self.notes.len() as u32 + 1;
    let user_id = self.current_user().id;
    let collateral_ids: Vec<u32> = vec![];

    Ok(Note::new(id, category, structure, content, user_id, collateral_ids))
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

      let category_strings: Vec<String> = values[1].split(" - ").map(|s| s.to_string()).collect();
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
            "Tbd" => Tbd,
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

      let structure = match &values[2][..] {
        "Care Plan Meeting" => CarePlan,
        "Care Plan Meeting Verbose" => CarePlanVerbose,
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
        "Custom" => CustomStructure,
        _ => return Err(Error::new(
          ErrorKind::Other,
          "Unsupported StructureType saved to file.",
        )),
      };

      let content = values[3].clone();

      let blanks_strings: Vec<String> = values[4].split('#').map(|s| s.to_string() ).collect();
      let mut blanks: HashMap<u32, (Blank, String, Vec<u32>)> = HashMap::new();

        // pub blanks: HashMap<u32, (Blank, String, Vec<u32>)> 

      for b_string in blanks_strings {
        if b_string != String::from("") {
          let blank_values: Vec<String> = b_string.split('%').map(|st| st.to_string() ).collect();
  
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

      let note_user_id: u32 = values[5].parse().unwrap();

      let collateral_ids: Vec<u32> = match &values[6][..] {
        "" => vec![],
        _ => values[6]
          .split("#")
          .map(|val| val.parse().unwrap())
          .collect(),
      };

      let mut n = Note::new(id, category, structure, content, note_user_id, collateral_ids);
      n.blanks = blanks;

      notes.push(n);
    }
    notes.sort_by(|a, b| a.id.cmp(&b.id));
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

    let mut saved_ids: Vec<u32> = self.current_note_day().foreign_keys["note_ids"].clone();
    saved_ids.push(note.id);

    self.current_note_day_mut().foreign_keys.insert(String::from("note_ids"), saved_ids);
    
    self.notes.insert(pos, note);

    self.write_notes().unwrap();
    self.write_note_days().unwrap();
  }
  fn current_user_notes(&self) -> Vec<&Note> {
    self.notes.iter().filter(|n| n.foreign_key["user_id"] == self.current_user().id).collect()
  }
  fn current_user_notes_mut(&mut self) -> Vec<&mut Note> {
    let current_id = self.current_user().id;
    self.notes.iter_mut().filter(|n| n.foreign_key["user_id"] == current_id).collect()
  }
  fn choose_delete_note(&mut self) {
    loop {
      self.display_delete_note();
      println!("Are you sure you want to delete this note?");
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
          self.delete_current_note();
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
    println!("{:-^163}", "-");
    println!("{:-^163}", heading);
    println!("{:-^163}", "-");

    // the length of each line is 163
    self.current_note().display_content();

    println!("{:-^163}", "-");
  }
  fn delete_current_note(&mut self) {
    let id = self.foreign_key.get("current_note_id").unwrap();
    self.notes.retain(|n| n.id != *id);
    self.foreign_key.remove("current_note_id");
  }
  fn get_note_option_by_id(&self, id: u32) -> Option<&Note> {
    self.notes.iter().find(|n| n.id == id)
  }
  fn get_note_option_by_id_mut(&mut self, id: u32) -> Option<&mut Note> {
    self.notes.iter_mut().find(|n| n.id == id)
  }
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
        (String::from("collateral_filepath"), String::from("some_random_blank_collateral_file_name.txt"),),
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
    fs::remove_file("some_random_blank_pronouns_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_note_day_file_name.txt").unwrap();
    fs::remove_file("some_random_blank_note_template_file_name.txt").unwrap();
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
          (String::from("collateral_filepath"), String::from("test_load_collateral.txt"),),
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
    fs::remove_file("test_load_pronouns.txt").unwrap();
    fs::remove_file("test_load_note_days.txt").unwrap();
    fs::remove_file("test_load_note_templates.txt").unwrap();
  }
  #[test]
  fn creates_unique_new_instances() {
    let filepaths: HashMap<String, String> = [
      (String::from("user_filepath"), String::from("test_user_new_instance.txt"),),
      (String::from("client_filepath"), String::from("test_client_new_instance.txt"),),
      (String::from("collateral_filepath"), String::from("test_collateral_new_instance.txt"),),
      (String::from("pronouns_filepath"), String::from("test_pronouns_new_instance.txt"),),
      (String::from("note_day_filepath"), String::from("test_note_days_new_instance.txt"),),
      (String::from("note_template_filepath"), String::from("test_note_templates_new_instance.txt"),),
      (String::from("note_filepath"), String::from("test_note_new_instance.txt"),),
    ].iter().cloned().collect();

    let mut notes = NoteArchive::new_test(filepaths.clone());

    let new_user_attempt =
      notes.generate_unique_new_user(String::from("Carl"), String::from("Carlson"), ICC, 1);
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
        ICC,
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
      (String::from("collateral_filepath"), String::from("test_collateral_current_pronouns.txt"),),
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
      (String::from("collateral_filepath"), String::from("test_collateral_updates_pronouns.txt"),),
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
}
