use std::fmt;
use std::collections::HashMap;

use crate::constants::*;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum StructureType {
  CarePlanMeeting,
  CarePlanMeetingVerbose,
  Intake,
  Assessment,
  SNCD,
  HomeVisit,
  AgendaPrep,
  Debrief,
  PhoneCall,
  Scheduling,
  SentEmail,
  Referral
}


use crate::StructureType::{CarePlanMeeting, CarePlanMeetingVerbose, Intake,
Assessment, SNCD, HomeVisit, AgendaPrep, Debrief, PhoneCall, Scheduling,
SentEmail, Referral};

impl StructureType {
  pub fn iterator() -> impl Iterator<Item = StructureType> {
    [
      CarePlanMeeting,
      CarePlanMeetingVerbose,
      Intake,
      Assessment,
      SNCD,
      HomeVisit,
      AgendaPrep,
      Debrief,
      PhoneCall,
      Scheduling,
      SentEmail,
      Referral
    ].iter().copied()
  }
  pub fn abbreviate(&self) -> &str {
    match self {
      CarePlanMeeting => "CPM",
      CarePlanMeetingVerbose => "CPM-V",
      Intake => "I",
      Assessment => "A",
      SNCD => "S",
      HomeVisit => "HV",
      AgendaPrep => "AP",
      Debrief => "D",
      PhoneCall => "PC",
      Scheduling => "SCH",
      SentEmail => "SE",
      Referral => "R",
    }
  }
}

impl fmt::Display for StructureType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      CarePlanMeeting => "Care Plan Meeting",
      CarePlanMeetingVerbose => "Care Plan Meeting Verbose",
      Intake => "Intake",
      Assessment => "Assessment",
      SNCD => "SNCD",
      HomeVisit => "Home Visit",
      AgendaPrep => "Agenda Prep",
      Debrief => "Debrief",
      PhoneCall => "Phone Call",
      Scheduling => "Scheduling",
      SentEmail => "Sent Email",
      Referral => "Referral",
    };
    write!(f, "{}", display)
  }
}

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
          length_adjusted_vec.push((1, String::from(&long_sent[..140])));
          long_sent = String::from(&long_sent[141..]);
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
  fn generate_display_content_string(&self) -> String {
    let mut content_slice = self.content.clone();
    for b in BLANKS.iter() {
      content_slice = str::replace(&content_slice, b.0, b.1);
    }
    content_slice.clone()
  }
  pub fn get_display_content_vec(&self) -> Vec<(usize, String)> {
    Self::get_display_content_vec_from_string(self.generate_display_content_string())
  }
  pub fn display_content(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");
    let display_custom = if self.custom { "custom" } else { "default" };
    let heading = format!("Default content of {} {} template", display_custom, self.structure);
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

#[derive(Debug, Clone)]
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

use crate::NoteCategory::{ICCNote, FPNote};

#[derive(Debug, Clone)]
pub enum ICCNoteCategory {
  FaceToFaceContactWithClient,
  TelephoneContactWithClient,
  CareCoordination,
  Documentation,
  CarePlanningTeam,
  TransportClient,
  MemberOutreachNoShow,
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

use crate::ICCNoteCategory::{FaceToFaceContactWithClient, TelephoneContactWithClient,
CareCoordination, Documentation, CarePlanningTeam, TransportClient, MemberOutreachNoShow};

#[derive(Debug, Clone)]
pub enum FPNoteCategory {
  Tbd,
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

use crate::FPNoteCategory::{Tbd};

pub struct Note {
  pub id: u32,
  pub category: NoteCategory,
  pub structure: StructureType,
  pub content: String,
  pub blanks: Vec<(String, u32)> // table name and id
}

impl Note {
  pub fn new(
    id: u32,
    category: NoteCategory,
    structure: StructureType,
    note_day_id: u32,
    note_template_id: u32,
    content: String) -> Note {
    let blanks = vec![];
    Note {
      id,
      category,
      structure,
      content,
      blanks,
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
    let blanks_str: String = self.blanks.iter().map(|(t, id)| format!("{}-{}", t, id) ).collect::<Vec<String>>().join("#");
    write!(
      f,
      "{} | {} | {} | {} | {}\n",
      &self.id,
      &self.category,
      &self.structure,
      &self.content,
      blanks_str,
    )
  }
}