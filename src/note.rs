#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_comparisons)]
#![allow(unused_attributes)]

use std::fmt;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::constants::*;

#[macro_use] use lazy_static::lazy_static;
use regex::Regex;
use regex::Match;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum StructureType {
  CarePlan,
  CarePlanVerbose,
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
  CustomStructure,
}


use StructureType::{CarePlan, CarePlanVerbose, Intake,
Assessment, Sncd, HomeVisit, AgendaPrep, Debrief, PhoneCall, Scheduling,
SentEmail, Referral, CustomStructure};

impl StructureType {
  pub fn iterator() -> impl Iterator<Item = StructureType> {
    [
      CarePlan,
      CarePlanVerbose,
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
      CustomStructure,
    ].iter().copied()
  }
  pub fn abbreviate(&self) -> &str {
    match self {
      CarePlan => "CPM",
      CarePlanVerbose => "CPM-V",
      Intake => "I",
      Assessment => "A",
      Sncd => "S",
      HomeVisit => "HV",
      AgendaPrep => "AP",
      Debrief => "D",
      PhoneCall => "PC",
      Scheduling => "SCH",
      SentEmail => "SE",
      Referral => "R",
      CustomStructure => "C",
    }
  }
}

impl fmt::Display for StructureType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      CarePlan => "Care Plan Meeting",
      CarePlanVerbose => "Care Plan Meeting Verbose",
      Intake => "Intake",
      Assessment => "Assessment",
      Sncd => "SNCD",
      HomeVisit => "Home Visit",
      AgendaPrep => "Agenda Prep",
      Debrief => "Debrief",
      PhoneCall => "Phone Call",
      Scheduling => "Scheduling",
      SentEmail => "Sent Email",
      Referral => "Referral",
      CustomStructure => "Custom",
    };
    write!(f, "{}", display)
  }
}

#[derive(Debug)]
pub struct NoteTemplate {
  pub id: u32,
  pub structure: StructureType,
  pub custom: bool,
  pub content: String,
  pub foreign_key: HashMap<String, u32>,
  pub foreign_keys: HashMap<String, Vec<u32>>,
}

impl NoteTemplate {
  pub fn new(id: u32, structure: StructureType, custom: bool, content: String, user_id: Option<u32>) -> NoteTemplate {
    let mut foreign_key: HashMap<String, u32> = HashMap::new();
    match user_id {
      Some(num) => {
        foreign_key.insert(String::from("user_id"), num);
      },
      None => (),
    }
    let foreign_keys: HashMap<String, Vec<u32>> = [
      (String::from("note_ids"), vec![]),
    ].iter().cloned().collect();
    NoteTemplate {
      id,
      structure,
      custom,
      content,
      foreign_key,
      foreign_keys,
    }
  }
  pub fn preview(&self) -> &str {
    if self.content.len() > 70 {
      &self.content[0..70]
    } else {
      &self.content[..]
    }
  }
  pub fn display_short(&self) -> String {
    let display_c = if self.custom { "custom" } else { "default" };
    format!("{} ({})", self.structure, display_c)
  }
  pub fn generate_display_content_string_with_blanks(&self, blank_focus_id: Option<u32>, content_focus_id: Option<u32>) -> String {
    let mut content_string = self.content.clone();

    match (blank_focus_id, content_focus_id) {
      (Some(_), Some(_)) => panic!("Focus IDs for both content and blank passed to generate_display_content_string_with_blanks on NoteTemplate."),
      _ => (),
    }

    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*#?[0-9]*#?---[)]").unwrap();
    }
    
    let mut typed_content: Vec<(usize, usize)> = vec![];
    let mut prev_end_idx: usize;

    let mut i: u32 = 1;
    loop {
      let m = RE_BLANK.find(&content_string);
      let m = match m {
        None => break,
        Some(m) => m,
      };

      let b: Blank = Blank::get_blank_from_str(&content_string[m.start()..m.end()]);

      let display_blank = match blank_focus_id {
        None => {
          format!("------ [{}]: {} ---", i, b.display_to_user())
        },
        Some(f_id) => {
          if i == f_id {
            format!("((>>>>>>>>>>---   [{}]: {}   ---<<<<<<<<<<))", i, b.display_to_user())
          } else {
            format!("------ [{}]: {} ---", i, b.display_to_user())
          }
        }
      };

      content_string = format!(
        "{}{}{}",
        &content_string[..m.start()],
        display_blank,
        &content_string[m.end()..]
      );

      if i == 1 {
        typed_content.push((0, m.start()));
      } else {
        typed_content.push((prev_end_idx, m.start()));
      }

      prev_end_idx = m.end();

      i += 1;
    }

    let c_i: u32 = 1;
    for (idx1, idx2) in typed_content {
      match content_focus_id {
        None => (),
        Some(f_id) => {
          if f_id == c_i {
            content_string = format!(
              "{} ((>>>>>>>>>>---   [{}]: {}   ---<<<<<<<<<<)) {}",
              &content_string[..idx1],
              c_i,
              &content_string[idx1..idx2],
              &content_string[idx2..],
            );
          } else {
            content_string = format!(
              "{} ((> [{}]: {} <)) {}",
              &content_string[..idx1],
              c_i,
              &content_string[idx1..idx2],
              &content_string[idx2..],
            );
          }
        },
      }
    }

    content_string
  }
  pub fn get_display_content_vec_from_string(display_content: String) -> Vec<(usize, String)> {
    let display_content_vec: Vec<String> = display_content.split(". ").map(|s| s.to_string() ).collect();
    let mut length_adjusted_vec = vec![];
    for (i, sent) in display_content_vec.iter().enumerate() {
      let mut sentence = sent.clone();
      sentence.push_str(".");
      if sentence.len() < 140 {
        length_adjusted_vec.push((i, sentence))
      } else {
        let mut long_sent = sentence.clone();
        while long_sent.len() > 140 {
          match &long_sent[..140].rfind(' ') {
            None => {
              length_adjusted_vec.push((i, String::from(&long_sent[..140])));
              long_sent = String::from(&long_sent[141..]);
            },
            Some(idx) => {
              length_adjusted_vec.push((i, String::from(&long_sent[..*idx])));
              long_sent = String::from(&long_sent[idx+1..]);
            }
          }
        }
        length_adjusted_vec.push((i, long_sent));
      }
    }
    length_adjusted_vec
  }
  pub fn display_content_from_vec(length_adjusted_vec: Vec<(usize, String)>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");
    println!("{:-^163}", "Current content");
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let mut current_i = 0;
    for (i, cont) in length_adjusted_vec {
      let display_i = if i == current_i {
        String::from("   ")
      } else {
        format!(" {} ", i)
      };
      println!("{:-^20} | {:-^140}", display_i, cont);
      current_i = i;
    }
  }
  pub fn generate_display_content_string(&self) -> String {
    let mut content_slice = self.content.clone();
    for b in Blank::iterator() {
      content_slice = content_slice.replace(&format!("{}", b), &b.display_to_user());
    }
    content_slice.clone()
  }
  pub fn get_display_content_vec(&self) -> Vec<(usize, String)> {
    Self::get_display_content_vec_from_string(self.generate_display_content_string())
  }
  pub fn display_content(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");
    let display_custom = if self.custom { "Custom" } else { "Default" };
    let heading = format!("{} {} template", display_custom, self.structure);
    println!("{:-^163}", heading);
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let mut current_i = 0;
    for (i, cont) in self.get_display_content_vec() {
      let display_i = if i == current_i {
        String::from("   ")
      } else {
        format!(" {} ", i)
      };
      println!("{:-^20} | {:-^140}", display_i, cont);
      current_i = i;
    }
  }
  pub fn display_edit_content(&self, blank_focus_id: Option<u32>, content_focus_id: Option<u32>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");
    let heading = format!(" Edit {} template ", self.structure);
    println!("{:-^163}", heading);
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let mut current_i = 0;
    let display_content_string = self.generate_display_content_string_with_blanks(blank_focus_id, content_focus_id);
    for (i, cont) in self.get_display_content_vec(display_content_string) {
      let display_i = if i == current_i {
        String::from("   ")
      } else {
        format!(" {} ", i)
      };
      println!("{:-^20} | {:-^140}", display_i, cont);
      current_i = i;
    }
  }
}

impl PartialEq for NoteTemplate {
  fn eq(&self, other: &Self) -> bool {
    self.structure == other.structure
      && self.content == other.content
  }
}

impl fmt::Display for NoteTemplate {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} | {} | {} | {} | {}\n",
      &self.id,
      &self.structure,
      &self.content,
      &self.foreign_key["user_id"],
      &self.foreign_keys["note_ids"]
        .iter()
        .map(|id| format!("{}", id))
        .collect::<Vec<String>>()
        .join("#"),
    )
  }
}

#[derive(Debug, Clone, Copy)]
pub enum NoteCategory {
  ICCNote(ICCNoteCategory),
  FPNote(FPNoteCategory),
}

impl PartialEq for NoteCategory {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (ICCNote(ca), ICCNote(cb)) => ca == cb,
      (FPNote(ca), FPNote(cb)) => ca == cb,
      _ => false,
    }
  }
}

impl fmt::Display for NoteCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      ICCNote(nt) => format!("ICC Note - {}", nt),
      FPNote(nt) => format!("FP Note - {}", nt),
    };
    write!(f, "{}", display)
  }
}

use NoteCategory::{ICCNote, FPNote};

#[derive(Debug, Clone, Copy)]
pub enum ICCNoteCategory {
  FaceToFaceContactWithClient,
  TelephoneContactWithClient,
  CareCoordination,
  Documentation,
  CarePlanningTeam,
  TransportClient,
  MemberOutreachNoShow,
}

impl ICCNoteCategory {
  pub fn iterator() -> impl Iterator<Item = ICCNoteCategory> {
    [
      FaceToFaceContactWithClient,
      TelephoneContactWithClient,
      CareCoordination,
      Documentation,
      CarePlanningTeam,
      TransportClient,
      MemberOutreachNoShow,
    ].iter().copied()
  }
}

impl PartialEq for ICCNoteCategory {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&FaceToFaceContactWithClient, &FaceToFaceContactWithClient) => true,
      (&TelephoneContactWithClient, &TelephoneContactWithClient) => true,
      (&CareCoordination, &CareCoordination) => true,
      (&Documentation, &Documentation) => true,
      (&CarePlanningTeam, &CarePlanningTeam) => true,
      (&TransportClient, &TransportClient) => true,
      (&MemberOutreachNoShow, &MemberOutreachNoShow) => true,
      _ => false,
    }
  }
}

impl fmt::Display for ICCNoteCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      FaceToFaceContactWithClient => "Face to face contact with client",
      TelephoneContactWithClient => "Telephone contact with client",
      CareCoordination => "Care coordination",
      Documentation => "Documentation",
      CarePlanningTeam => "Care planning team",
      TransportClient => "Transport client",
      MemberOutreachNoShow => "Member outreach/no-show",
    };
    write!(f, "{}", display)
  }
}

use ICCNoteCategory::{FaceToFaceContactWithClient, TelephoneContactWithClient,
CareCoordination, Documentation, CarePlanningTeam, TransportClient, MemberOutreachNoShow};

#[derive(Debug, Clone, Copy)]
pub enum FPNoteCategory {
  Tbd,
}

impl FPNoteCategory {
  pub fn iterator() -> impl Iterator<Item = FPNoteCategory> {
    [
      Tbd,
    ].iter().copied()
  }
}

impl fmt::Display for FPNoteCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      Tbd => "Tbd",
    };
    write!(f, "{}", display)
  }
}

impl PartialEq for FPNoteCategory {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&Tbd, &Tbd) => true,
      _ => false,
    }
  }
}

use FPNoteCategory::{Tbd};

pub struct Note {
  pub id: u32,
  pub category: NoteCategory,
  pub structure: StructureType,
  pub content: String,
  pub blanks: HashMap<u32, (Blank, String, Vec<u32>)>, // blank type, display string, foreign keys - hashed by position of blank
  pub foreign_key: HashMap<String, u32>,
  pub foreign_keys: HashMap<String, Vec<u32>>,
}

impl Note {
  pub fn new(
    id: u32,
    category: NoteCategory,
    structure: StructureType,
    content: String,
    user_id: u32,
    collateral_ids: Vec<u32>
  ) -> Note {
    let blanks = HashMap::new();
    let foreign_key: HashMap<String, u32> = [
      (String::from("user_id"), user_id),
    ].iter().cloned().collect();
    let foreign_keys: HashMap<String, Vec<u32>> = [
      (String::from("collateral_ids"), collateral_ids),
    ].iter().cloned().collect();
    Note {
      id,
      category,
      structure,
      content,
      blanks,
      foreign_key,
      foreign_keys,
    }
  }
  pub fn preview(&self) -> &str {
    if self.content.len() > 70 {
      &self.content[0..70]
    } else {
      &self.content[..]
    }
  }


  pub fn get_content_vec_from_string(display_content: String) -> Vec<(usize, String)> {
    let display_content_vec: Vec<String> = display_content.split(". ").map(|s| s.to_string() ).collect();
    let mut length_adjusted_vec = vec![];
    for (i, sent) in display_content_vec.iter().enumerate() {
      let mut sentence = sent.clone();
      sentence.push_str(".");
      if sentence.len() < 140 {
        length_adjusted_vec.push((i, sentence))
      } else {
        let mut long_sent = sentence.clone();
        while long_sent.len() > 140 {
          match &long_sent[..140].rfind(' ') {
            None => {
              length_adjusted_vec.push((i, String::from(&long_sent[..140])));
              long_sent = String::from(&long_sent[141..]);
            },
            Some(idx) => {
              let isize_idx_option = isize::try_from(*idx);
              let usize_idx = match isize_idx_option {
                Ok(i) => i as usize,
                Err(_) => panic!("Failed to cast index of string from REGEX match to isize in fn 'get_content_vec_from_string'"),
              };
              length_adjusted_vec.push((i, String::from_utf8(long_sent.as_bytes()[0..usize_idx].to_vec()).unwrap()));
              long_sent = String::from(&long_sent[idx+1..]);
            }
          }
        }
        length_adjusted_vec.push((i, long_sent));
      }
    }
    length_adjusted_vec
  }
  pub fn generate_display_content_string(&self) -> String {
    let mut content_string = self.content.clone();
    for (_, (blank_type, display_string, _)) in self.blanks.iter() {
      content_string = content_string.replacen(&format!("{}", blank_type)[..], &display_string[..], 1);
    }
    for blank_type in Blank::iterator() {
      content_string = str::replace(&content_string, &format!("{}", blank_type)[..], "_______________");
    }
    content_string
  }
  pub fn generate_display_content_string_with_blanks(&self, focus_id: Option<u32>) -> String {
    let mut content_string = self.content.clone();
    for (i, (blank_type, display_string, _)) in self.blanks.iter() {
      content_string = content_string.replacen(&format!("{}", blank_type)[..], &format!("[{}] {}", i, &display_string[..])[..], 1);
    }

    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*#?[0-9]*#?---[)]").unwrap();
    }
    
    let mut i: u32 = 1;
    let mut unfilled = 0;
    loop {
      if self.blanks.keys().any(|ki| ki == &i ) {
        i += 1;
        continue;
      }
      unfilled += 1;
      let m = RE_BLANK.find(&content_string);
      let m = match m {
        None => break,
        Some(m) => m,
      };

      let display_blank = match focus_id {
        None => {
          if unfilled == 1 {
            format!("[===| {} |===]", i)
          } else {
            format!("_______{}_______", i)
          }
        },
        Some(f_id) => {
          if i == f_id {
            format!("[===| {} |===]", i)
          } else {
            format!("_______{}_______", i)
          }
        }
      };

      content_string = format!(
        "{}{}{}",
        &content_string[..m.start()],
        display_blank,
        &content_string[m.end()..]
      );

      i += 1;
    }
    content_string
  }
  pub fn get_display_content_vec(&self) -> Vec<(usize, String)> {
    Self::get_content_vec_from_string(self.generate_display_content_string())
  }
  pub fn get_display_content_vec_and_blanks(&self, focus_id: Option<u32>) -> Vec<(usize, String)> {
    Self::get_content_vec_from_string(self.generate_display_content_string_with_blanks(focus_id))
  }
  pub fn get_blank_types(&self) -> Vec<Blank> {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*#?[0-9]*#?---[)]").unwrap();
    }
    let blank_content = self.content.clone();
    let mut blanks: Vec<Blank> = vec![];
    while RE_BLANK.is_match(&blank_content) {
      let m = RE_BLANK.find(&blank_content).unwrap();
      blanks.push(Blank::get_blank_from_str(&blank_content[m.start()..m.end()]));
      RE_BLANK.replace(&blank_content, "X");
    }
    blanks
  }
  pub fn get_empty_blanks_and_indexes(&self) -> Vec<(u32, Blank)> {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*#?[0-9]*#?---[)]").unwrap();
    }
    let num_total_blanks = RE_BLANK.find_iter(&self.content).count();
    let ordered_blanks = self.get_blank_types();
    let mut blanks: Vec<(u32, Blank)> = vec![];
    for i in 1..num_total_blanks {
      let i = i as u32;
      match self.blanks.get(&i) {
        Some(b_tup) => (),
        None => {
          blanks.push((i, ordered_blanks[i as usize]));
        }
      }
    }
    blanks
  }
  pub fn has_unfilled_blanks(&self) -> bool {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*#?[0-9]*#?---[)]").unwrap();
    }
    RE_BLANK.find_iter(&self.content[..]).count() < self.blanks.len()
  }
  pub fn number_of_blanks(&self) -> usize {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*#?[0-9]*#?---[)]").unwrap();
    }
    RE_BLANK.find_iter(&self.content[..]).count()
  }
  pub fn display_content(&self) {
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // this should not be included - Note doesn't know current user or client. Need separate heading
    println!("{:-^163}", "-");
    let cat_string = match self.category {
      ICCNote(ncat) => format!("'{}' by ICC", ncat),
      FPNote(ncat) => format!("'{}' by FP", ncat),
    };
    let heading = format!(" Current content of {} service entry note for {} ", self.structure, cat_string);
    println!("{:-^163}", heading);
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let mut current_i: isize = -1;
    for (i, cont) in self.get_display_content_vec() {
      let i = i as isize;
      let display_i = if i == current_i {
        String::from("   ")
      } else {
        format!(" {} ", i)
      };
      println!("{:-^20} | {:-^140}", display_i, cont);
      current_i = i;
    }
  }
}

impl PartialEq for Note {
  fn eq(&self, other: &Self) -> bool {
    self.category == other.category
      && self.structure == other.structure
      && self.content == other.content
      && self.blanks == other.blanks
  }
}

impl fmt::Display for Note {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut formatted_blanks = vec![];
    for (order, blanks_tup) in &self.blanks {
      let blanks_string = format!(
        "{}%{}%{}%{}",
        order,
        blanks_tup.0,
        blanks_tup.1,
        blanks_tup.2.iter().map(|id| id.to_string() ).collect::<Vec<String>>().join("-")
      );
      formatted_blanks.push(blanks_string);
    }
    let blanks_str: String = formatted_blanks.join("#");
    write!(
      f,
      "{} | {} | {} | {} | {} | {} | {}\n",
      &self.id,
      &self.category,
      &self.structure,
      &self.content,
      blanks_str,
      &self.foreign_key["user_id"],
      &self
        .foreign_keys["collateral_ids"]
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("#"),
    )
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Blank {
  CurrentUser,
  CurrentClientName,
  Collaterals,
  AllCollaterals,
  Pronoun1ForBlank(Option<u32>),
  Pronoun2ForBlank(Option<u32>),
  Pronoun3ForBlank(Option<u32>),
  Pronoun4ForBlank(Option<u32>),
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
  Action,
  Phrase,
  CustomBlank
}


use Blank::{
  CurrentUser,
  CurrentClientName,
  Collaterals,
  AllCollaterals,
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
  Action,
  Phrase,
  CustomBlank,
};

impl Blank {
  pub fn iterator() -> impl Iterator<Item = Blank> {
    [
      CurrentUser,
      CurrentClientName,
      Collaterals,
      AllCollaterals,
      Pronoun1ForBlank(None),
      Pronoun2ForBlank(None),
      Pronoun3ForBlank(None),
      Pronoun4ForBlank(None),
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
      Action,
      Phrase,
      CustomBlank,
    ].iter().copied()
  }
  pub fn vector_of_variants() -> Vec<Blank> {
    vec![
      CurrentUser,
      CurrentClientName,
      Collaterals,
      AllCollaterals,
      Pronoun1ForBlank(None),
      Pronoun2ForBlank(None),
      Pronoun3ForBlank(None),
      Pronoun4ForBlank(None),
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
      Action,
      Phrase,
      CustomBlank,
    ]
  }
  pub fn abbreviate(&self) -> String {
    match self {
      CurrentUser => String::from("u"),
      CurrentClientName => String::from("c"),
      Collaterals => String::from("co"),
      AllCollaterals => String::from("allco"),
      Pronoun1ForBlank(id_opt) => match id_opt {
        Some(id) => format!("pb1@{}@", id),
        None => String::from("pb1"),
      }
      Pronoun2ForBlank(id_opt) => match id_opt {
        Some(id) => format!("pb2@{}@", id),
        None => String::from("pb2"),
      }
      Pronoun3ForBlank(id_opt) => match id_opt {
        Some(id) => format!("pb3@{}@", id),
        None => String::from("pb3"),
      }
      Pronoun4ForBlank(id_opt) => match id_opt {
        Some(id) => format!("pb4@{}@", id),
        None => String::from("pb4"),
      }
      Pronoun1ForUser => String::from("pu1"),
      Pronoun2ForUser => String::from("pu2"),
      Pronoun3ForUser => String::from("pu3"),
      Pronoun4ForUser => String::from("pu4"),
      Pronoun1ForClient => String::from("pc1"),
      Pronoun2ForClient => String::from("pc2"),
      Pronoun3ForClient => String::from("pc3"),
      Pronoun4ForClient => String::from("pc4"),
      TodayDate => String::from("td"),
      NoteDayDate => String::from("ndd"),
      InternalDocument => String::from("id"),
      ExternalDocument => String::from("ed"),
      InternalMeeting => String::from("im"),
      ExternalMeeting => String::from("em"),
      Action => String::from("a"),
      Phrase => String::from("p"),
      CustomBlank => String::from("cu"),
    }
  }
  pub fn get_blank_from_str(s: &str) -> Blank {
    match &s[..] {
      "(---u---)" => CurrentUser,
      "(---c---)" => CurrentClientName,
      "(---co---)" => Collaterals,
      "(---allco---)" => AllCollaterals,
      "(---pu1---)" => Pronoun1ForUser,
      "(---pu2---)" => Pronoun2ForUser,
      "(---pu3---)" => Pronoun3ForUser,
      "(---pu4---)" => Pronoun4ForUser,
      "(---pc1---)" => Pronoun1ForClient,
      "(---pc2---)" => Pronoun2ForClient,
      "(---pc3---)" => Pronoun3ForClient,
      "(---pc4---)" => Pronoun4ForClient,
      "(---td---)" => TodayDate,
      "(---ndd---)" => NoteDayDate,
      "(---id---)" => InternalDocument,
      "(---ed---)" => ExternalDocument,
      "(---im---)" => InternalMeeting,
      "(---em---)" => ExternalMeeting,
      "(---a---)" => Action,
      "(---p---)" => Phrase,
      "(---cu---)" => CustomBlank,
      _ => { // only other options are pb@id@ which is a pronouns for blank, requires parsing 'id' to int
        let components = s.split("@").map(|st| st.to_string() ).collect::<Vec<String>>();

        let blank_id: u32 = match components[1].parse() {
          Ok(num) => num,
          Err(e) => panic!("Attempted to read Blank type from str: {}, {}", components[1], e),
        };

        match &format!("{}{}", components[0], components[2])[..] {
          "(---pb1---)" => Pronoun1ForBlank(Some(blank_id)),
          "(---pb2---)" => Pronoun2ForBlank(Some(blank_id)),
          "(---pb3---)" => Pronoun3ForBlank(Some(blank_id)),
          "(---pb4---)" => Pronoun4ForBlank(Some(blank_id)),
          _ => panic!("Failed to read Blank type from string."),
        }
      },
    }
  }
  pub fn display_to_user(&self) -> String {
    match self {
      CurrentUser => String::from("Current user"),
      CurrentClientName => String::from("Name of client"),
      Collaterals => String::from("One or more collaterals"),
      AllCollaterals => String::from("All collaterals for the current client"),
      Pronoun1ForBlank(b_id_option) => {
        let b_id = b_id_option.unwrap();
        format!("Subject pronouns of the person in blank #{} (he, she, they)", b_id)
      },
      Pronoun2ForBlank(b_id_option) => {
        let b_id = b_id_option.unwrap();
        format!("Object pronouns of the person in blank #{} (him, her, them)", b_id)
      },
      Pronoun3ForBlank(b_id_option) => {
        let b_id = b_id_option.unwrap();
        format!("Possessive detemriner pronouns of the person in blank #{} (his, her, their)", b_id)
      },
      Pronoun4ForBlank(b_id_option) => {
        let b_id = b_id_option.unwrap();
        format!("Possessive pronouns of the person in blank #{} (his, hers, theirs)", b_id)
      },
      Pronoun1ForUser => String::from("Subject pronoun of the current user"),
      Pronoun2ForUser => String::from("Object pronoun of the current user"),
      Pronoun3ForUser => String::from("Possessive determiner of the current user"),
      Pronoun4ForUser => String::from("Possessive pronoun of the current user"),
      Pronoun1ForClient => String::from("Subject pronoun of the current client"),
      Pronoun2ForClient => String::from("Object pronoun of the current client"),
      Pronoun3ForClient => String::from("Possessive determiner of the current client"),
      Pronoun4ForClient => String::from("Possessive pronoun of the current client"),
      TodayDate => String::from("Today's date"),
      NoteDayDate => String::from("The date of the current note"),
      InternalDocument => String::from("Internal document"),
      ExternalDocument => String::from("External document"),
      InternalMeeting => String::from("Wraparound meeting title"),
      ExternalMeeting => String::from("External meeting title"),
      Action => String::from("General action"),
      Phrase => String::from("Other phrase"),
      CustomBlank => String::from("Custom input"),
    }
  }
}

impl fmt::Display for Blank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(---{}---)", self.abbreviate())
  }
}