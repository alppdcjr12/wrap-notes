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
use ansi_term::Colour::{Black, Red, Green, Yellow, Blue, Purple, Cyan, White, RGB};
use ansi_term::{Style, ANSIStrings, ANSIString};

use std::{thread, time};

// bold, dimmed, italic, underline, blink, reverse, hidden, strikethrough, on

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

#[derive(Debug, Clone)]
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
  pub fn preview(&self) -> String {
    let (this_content, _) = self.generate_display_content_string_with_blanks(None, None);
    if this_content.len() > 95 {
      format!("{}{}", &this_content[0..95], "...")
    } else {
      format!("{}{}", &this_content[..], "...")
    }
  }
  pub fn display_short(&self) -> String {
    let display_c = if self.custom { "custom" } else { "default" };
    format!("{} ({})", self.structure, display_c)
  }
  pub fn get_typed_content_indices(&self) -> Vec<(usize, usize)> {
    let mut content_string = self.content.clone();

    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    
    let mut typed_content_indices: Vec<(usize, usize)> = vec![];
    let mut prev_end_idx: usize = 0;
    
    let mut i = 1;
    loop {
      let find_match_string = content_string.clone();
      let m = RE_BLANK.find(&find_match_string);
      let m = match m {
        None => break,
        Some(m) => m,
      };

      let b: Blank = Blank::get_blank_from_str(&content_string[m.start()..m.end()]);

      let display_blank = String::from("X");

      content_string = format!(
        "{}{}{}",
        &content_string[..m.start()],
        display_blank,
        &content_string[m.end()..]
      );

      if i == 1 {
        typed_content_indices.push((0, m.start()));
      } else {
        typed_content_indices.push((prev_end_idx, m.start()));
      }

      prev_end_idx = m.end();

      i += 1;
    }
    typed_content_indices
  }
  pub fn get_blanks_with_new_ids(mut blanks: Vec<Blank>, position: u32) -> Vec<Blank> {
    let copied_blanks = blanks.clone();

    for (i, b) in copied_blanks.iter().enumerate() {
      match b {
        Pronoun1ForBlank(id) => {
          if id.unwrap() + 1 >= position {
            blanks = blanks.iter().enumerate().map(|(oi, ob)| 
              if oi == i {
                Pronoun1ForBlank(Some(id.unwrap()+1))
              } else {
                Pronoun1ForBlank(Some(id.unwrap()))
              }
            ).collect::<Vec<Blank>>();

          }
        },
        Pronoun2ForBlank(id) => {
          if id.unwrap() + 1 >= position {
            blanks = blanks.iter().enumerate().map(|(oi, ob)| 
              if oi == i {
                Pronoun2ForBlank(Some(id.unwrap()+1))
              } else {
                Pronoun2ForBlank(Some(id.unwrap()))
              }
            ).collect::<Vec<Blank>>();
          }
        },
        Pronoun3ForBlank(id) => {
          if id.unwrap() + 1 >= position {
            blanks = blanks.iter().enumerate().map(|(oi, ob)| 
              if oi == i {
                Pronoun3ForBlank(Some(id.unwrap()+1))
              } else {
                Pronoun3ForBlank(Some(id.unwrap()))
              }
            ).collect::<Vec<Blank>>();
          }
        },
        Pronoun4ForBlank(id) => {
          if id.unwrap() + 1 >= position {
            blanks = blanks.iter().enumerate().map(|(oi, ob)| 
              if oi == i {
                Pronoun4ForBlank(Some(id.unwrap()+1))
              } else {
                Pronoun4ForBlank(Some(id.unwrap()))
              }
            ).collect::<Vec<Blank>>();
          }
        },
        _ => (),
      }
    }
    blanks
  }
  pub fn get_sentence_end_indices(current_idx: usize, content: String) -> Vec<(usize, usize)> {
    let mut output_vec: Vec<(usize, usize)> = vec![];
    let mut content_string = content.clone();
    let mut offset = 0;

    while content_string.contains(". ") && content_string != String::from(". ") {
      let mut sents = content_string.split(". ");
      let first_sent = sents.nth(0).unwrap().to_string();
      let num_chars = first_sent.chars().count() + 1;
      output_vec.push((current_idx + offset, current_idx + offset + num_chars));
      offset += num_chars+1;
      content_string = content_string.split(". ").collect::<Vec<&str>>()[1..].join(". ");
    }
    let num_chars = content_string.chars().count();
    if num_chars > 0 && content_string != String::from(". ") {
      output_vec.push((current_idx + offset, (current_idx + offset + num_chars) - 1 ));
    }
    output_vec
  }
  pub fn generate_display_content_string_with_blanks(&self, blank_focus_id: Option<u32>, content_focus_id: Option<u32>) -> (String, Vec<(String, usize, usize)>) {
    let mut content_string = self.content.clone();
    let mut format_vec: Vec<(String, usize, usize)> = vec![];
    let mut cont_i = 1;

    match (blank_focus_id, content_focus_id) {
      (Some(_), Some(_)) => panic!("Focus IDs for both content and blank passed to generate_display_content_string_with_blanks on NoteTemplate."),
      _ => (),
    }

    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }

    let mut prev_end_idx: usize = 0;
    let mut content = String::new();
    let mut i: u32 = 1;
    loop {
      let find_match_string = content_string.clone();
      let m = RE_BLANK.find(&find_match_string);
      let m = match m {
        None => {
          if content.len() == 0 {
            if find_match_string.chars().count() == 0 {
              break;
            }
            content.push_str(&find_match_string.clone());
            match content_focus_id {
              Some(id) => {
                let sentence_indices = NoteTemplate::get_sentence_end_indices(content.chars().count(), find_match_string.clone());
                let last_tuple = sentence_indices[sentence_indices.len()-1].clone();
                for (idx1, idx2) in sentence_indices {
                  let mut idx2_updated = idx2+1;
                  if idx1 == last_tuple.0 && idx2 == last_tuple.1 {
                    idx2_updated += 1;
                  }
                  if cont_i == id {
                    format_vec.push((String::from("HIGHLIGHTED CONTENT"), idx1, idx2_updated));
                  } else {
                    format_vec.push((String::from("UNHIGHLIGHTED CONTENT"), idx1, idx2_updated));
                  }
                  cont_i += 1;
                }
              },
              None => {
                let sentence_indices = NoteTemplate::get_sentence_end_indices(content.chars().count(), find_match_string.clone());
                let last_tuple = sentence_indices[sentence_indices.len()-1];
                for (idx1, idx2) in sentence_indices {
                  let mut idx2_updated = idx2+1;
                  if idx1 == last_tuple.0 && idx2 == last_tuple.1 {
                    idx2_updated += 1;
                  }
                  format_vec.push((String::from("CONTENT"), idx1, idx2_updated));
                }
              },
            }
            break;
          } else {
            if prev_end_idx <= find_match_string.chars().count()-1 {
              match content_focus_id {
                Some(focus_id) => {
                  let sentence_indices = NoteTemplate::get_sentence_end_indices(
                    prev_end_idx,
                    format!("{}", &content_string[prev_end_idx..]),
                  );
                  let last_tuple = sentence_indices[sentence_indices.len()-1];
                  for (idx1, idx2) in sentence_indices.clone() {
                    let num_to_add = if idx1 == last_tuple.0 && idx2 == last_tuple.1 { 1 } else { 2 };
                    let display_content = format!("[{}]: {}", cont_i, &String::from(&content_string[idx1..idx2+num_to_add]));
                    let cidx1 = content.chars().count();
                    content.push_str(&display_content);
                    let cidx2 = content.chars().count();
                    if focus_id == cont_i {
                      format_vec.push((String::from("HIGHLIGHTED CONTENT"), cidx1, cidx2));
                    } else {
                      format_vec.push((String::from("UNHIGHLIGHTED CONTENT"), cidx1, cidx2));
                    }
                    cont_i += 1;
                  }
                  // cont_i += 1;
                  // last iteration, so no need to increment
                },
                None => {
                  let end_string = String::from(&find_match_string[prev_end_idx..]);
                  let sentence_indices = NoteTemplate::get_sentence_end_indices(
                    content.chars().count(),
                    format!("{}", &find_match_string[prev_end_idx..]),
                  );
                  content.push_str(&end_string);
                  for (idx1, idx2) in sentence_indices {
                    format_vec.push((String::from("CONTENT"), idx1, idx2+1));
                  }
                }
              }
            }
            break;
          }
        }
        Some(m) => m,
      };

      let num_chars = m.end() - m.start();
      let mut replacement = String::new();
      for _ in 0..num_chars {
        replacement.push_str("X");
      }
      
      let b: Blank = Blank::get_blank_from_str(&content_string[m.start()..m.end()]);
      content_string = format!("{}{}{}", &content_string[..m.start()], &replacement, &content_string[m.end()..]);

      let display_blank = match blank_focus_id {
        None => format!("{}", b.display_to_user()),
        Some(_) => format!("[{}]: {}", i, b.display_to_user()),
      };

      match content_focus_id {
        None => {
          let display_content = String::from(&content_string[prev_end_idx..m.start()]);

          let last_idx_before_adding = content.chars().count();

          let cidx1 = content.chars().count();
          content.push_str(&display_content);
          let cidx2 = content.chars().count();

          if cidx1 != cidx2 {
            let sentence_indices = NoteTemplate::get_sentence_end_indices(
              last_idx_before_adding,
              format!("{}", &content_string[prev_end_idx..m.start()]),
            );
            for (idx1, idx2) in sentence_indices {
              format_vec.push((String::from("CONTENT"), idx1, idx2+1));
            }
          }
        },
        Some(f_id) => {
          
          let sentence_indices = NoteTemplate::get_sentence_end_indices(
            prev_end_idx,
            format!("{}", &content_string[prev_end_idx..m.start()]),
          );
          let last_tuple = if sentence_indices.len() > 0 {
            sentence_indices[sentence_indices.len()-1]
          } else {
            (0, 0)
          };
          for (idx1, idx2) in sentence_indices.clone() {
            let num_to_add = if idx1 == last_tuple.0 && idx2 == last_tuple.1 { 1 } else { 2 };
            let display_content = format!("[{}]: {}", cont_i, &String::from(&content_string[idx1..idx2+num_to_add]));
            let cidx1 = content.chars().count();
            content.push_str(&display_content);
            let cidx2 = content.chars().count();
            if f_id == cont_i {
              format_vec.push((String::from("HIGHLIGHTED CONTENT"), cidx1, cidx2));
            } else {
              format_vec.push((String::from("UNHIGHLIGHTED CONTENT"), cidx1, cidx2));
            }
            cont_i += 1;
          }
        }
      }

      let bidx1 = content.chars().count();
      content.push_str(&display_blank);
      let bidx2 = content.chars().count();
      
      if bidx1 != bidx2 {
        match blank_focus_id {
          Some(f_id) => {
            if f_id == i {
              format_vec.push((String::from("HIGHLIGHTED BLANK"), bidx1, bidx2));
            } else {
              format_vec.push((String::from("UNHIGHLIGHTED BLANK"), bidx1, bidx2));
            }
          },
          None => {
            format_vec.push((String::from("BLANK"), bidx1, bidx2));
          }
        }
      }

      prev_end_idx = m.end();

      i += 1;
    }
    (content, format_vec)
  }
  pub fn get_ordered_blanks(&self) -> Vec<Blank> {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    RE_BLANK.find_iter(&self.content).map(|m| Blank::get_blank_from_str(&self.content[m.start()..m.end()]) ).collect::<Vec<Blank>>()
  }
  pub fn get_display_content_vec_from_string(display_content: String, color_formatting: Option<Vec<(String, usize, usize)>>) -> Vec<(usize, String, Option<Vec<(String, usize, usize)>>)> {
    let display_content_vec: Vec<String> = display_content.split(". ").map(|s| s.to_string() ).collect();
    let mut length_adjusted_vec = vec![];
    let mut current_idx = 0;
    for (i, sent) in display_content_vec.iter().enumerate() {
      if sent.chars().count() > 0 {
        let mut sentence = sent.clone();
        if i != display_content_vec.len() - 1 && sentence != String::from("") && sentence != String::from(" ") {
          sentence.push_str(". ");
        }
        if sentence.chars().count() < 140 {
          match color_formatting.clone() {
            None => length_adjusted_vec.push((i, sentence.clone(), None)),
            Some(f) => {
              let sentence_formatting: Vec<(String, usize, usize)> = f.iter()
                .filter(|(_, i1, i2)| i1 >= &current_idx && i2 <= &(sentence.chars().count() + current_idx) )
                .map(|(s, i1, i2)| (s.to_string(), i1-current_idx, i2-current_idx) )
                .collect();
              
              length_adjusted_vec.push((i, sentence.clone(), Some(sentence_formatting)));
            },
          }
          current_idx += sentence.chars().count();
        } else {
          let mut long_sent = sentence.clone();
          while long_sent.len() > 140 {
            let overflowing_blank: Option<(String, usize, usize)> = match color_formatting.clone() {
              None => None,
              Some(f) => {
                let maybe_blank = f.iter().find(|(_, i1, i2)| i1 != &0 && i1 <= &(current_idx+140) && i2 >= &(current_idx+140) );
                match maybe_blank {
                  None => None,
                  Some(tup) => Some(tup.clone()),
                }
              },
            };
            match overflowing_blank {
              Some(b) => {
                let sentence_formatting: Vec<(String, usize, usize)> = color_formatting.clone().unwrap().iter()
                  .filter(|(_, i1, i2)| i1 >= &current_idx && i2 < &b.2)
                  .map(|(s, i1, i2)|
                    (s.to_string(), i1-current_idx, i2-current_idx)
                  )
                  .collect();
                length_adjusted_vec.push((i, String::from(&long_sent[..b.1-current_idx]), Some(sentence_formatting)));
                long_sent = String::from(&long_sent[b.1-current_idx..]);
                current_idx += b.1-current_idx;
              },
              None => {
                match color_formatting.clone() {
                  None => {
                    match &long_sent[..140].rfind(' ') {
                      None => {
                        length_adjusted_vec.push((i, String::from(&long_sent[..140]), None));
                        long_sent = String::from(&long_sent[141..]);
                        current_idx += 140;
                      },
                      Some(idx) => {
                        length_adjusted_vec.push((i, String::from(&long_sent[..*idx]), None));
                        long_sent = String::from(&long_sent[*idx..]);
                        current_idx += idx;
                      },
                    }
                  },
                  Some(f) => {
                    let mut current_sent = long_sent.clone();
                    let rightmost_space = loop {
                      match current_sent.rfind(' ') {
                        None => break None,
                        Some(space_idx) => {
                          match f.iter().find(|(s, i1, i2)| i1-current_idx < space_idx && i2-current_idx > space_idx && !String::from("UNHIGHLIGHTED BLANK").contains(&s[..]) ) {
                            // "UNHIGHLIGHTED BLANK" contains all 3 blank descriptor variations
                            Some(tup) => {
                              current_sent = String::from(&current_sent[..tup.1-current_idx]);
                              continue;
                            },
                            None => break Some(space_idx),
                          }
                        }
                      }
                    };
                    match rightmost_space {
                      None => {
                        let last_divider_idx = f.iter().find(|(s, i1, i2)| i1 > &current_idx && i2 >= &(current_idx+140) );
                        match last_divider_idx {
                          None => {
                            let sentence_formatting: Vec<(String, usize, usize)> = f.iter()
                              .filter(|(s, i1, i2)| i1 > &current_idx && i2 <= &(current_idx+140) )
                              .map(|(s, i1, i2)|
                                if i2 <= &(current_idx+140) {
                                  (s.to_string(), i1-current_idx, i2-current_idx)
                                } else {
                                  (s.to_string(), i1-current_idx, 140)
                                }
                              )
                              .collect();
                              
                            length_adjusted_vec.push((i, String::from(&long_sent[..140]), Some(sentence_formatting)));
                            long_sent = String::from(&long_sent[141..]);
                            current_idx += 140;
                          },
                          Some(idx_tup) => {
                            let pos = idx_tup.1 - current_idx;
                            let sentence_formatting: Vec<(String, usize, usize)> = f.iter()
                              .filter(|(s, i1, i2)| i1 >= &current_idx && i2 <= &(current_idx+pos) )
                              .map(|(s, i1, i2)|
                                if i2 <= &(current_idx+pos) {
                                  (s.to_string(), i1-current_idx, i2-current_idx)
                                } else {
                                  (s.to_string(), i1-current_idx, pos)
                                }
                              )
                              .collect();
                            length_adjusted_vec.push((i, String::from(&long_sent[..pos]), Some(sentence_formatting)));
                            long_sent = String::from(&long_sent[pos..]);
                            current_idx += pos;
                          }
                        }
                      },
                      Some(spc) => {
                        let sentence_formatting: Vec<(String, usize, usize)> = f.iter()
                          .filter(|(s, i1, i2)| i1 > &current_idx && i2 <= &(current_idx+140) )
                          .map(|(s, i1, i2)|
                            if i2 > &(current_idx+spc) {
                              (s.to_string(), i1-current_idx, spc)
                            } else {
                              (s.to_string(), i1-current_idx, i2-current_idx)
                            }
                          )
                          .collect();
                        length_adjusted_vec.push((i, String::from(&long_sent[..spc]), None));
                        long_sent = String::from(&long_sent[spc..]);
                        current_idx += 140;
                      },
                    }
                  }
                }
              },
            }
          }
          match color_formatting.clone() {
            None => length_adjusted_vec.push((i, long_sent, None)),
            Some(f) => {
              let sentence_formatting: Vec<(String, usize, usize)> = f.iter()
                .filter(|(_, i1, i2)| i1 >= &current_idx && i2 <= &(long_sent.chars().count() + current_idx) )
                .map(|(s, i1, i2)| (s.to_string(), i1-current_idx, i2-current_idx) )
                .collect();
                
              length_adjusted_vec.push((i, long_sent.clone(), Some(sentence_formatting)));
              current_idx += long_sent.chars().count();
            }
          }
        }
      }
    }
    length_adjusted_vec
  }
  pub fn display_content_from_vec(length_adjusted_vec: Vec<(usize, String)>, s: &StructureType) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");
    let heading = format!(" Current content for new {} note template ", s);
    println!("{:-^163}", heading);
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let mut current_i = 100; // arbitrary value not equal to zero to allow first iteration 
    for (i, cont) in length_adjusted_vec {
      let display_i = if i == current_i {
        String::from("   ")
      } else {
        let d_i = i + 1;
        format!(" {} ", d_i)
      };
      println!("{:-^20} | {: <140}", display_i, cont);
      current_i = i;
    }
    println!("{:-^163}", "-");
  }
  pub fn generate_display_content_string(&self) -> String {
    let mut content_slice = self.content.clone();
    for b in Blank::iterator() {
      content_slice = content_slice.replace(&format!("{}", b), &b.display_to_user_empty());
    }
    content_slice.clone()
  }
  pub fn get_display_content_vec(&self) -> Vec<(usize, String)> {
    let (display_content_string, formatting) = self.generate_display_content_string_with_blanks(None, None);
    Self::get_display_content_vec_from_string(display_content_string, Some(formatting)).iter()
      .map(|(u, s, o)| (*u, s.to_string()) )
      .collect::<Vec<(usize, String)>>()
  }
  pub fn display_content(&self) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");
    let display_custom = if self.custom { "Custom" } else { "Default" };

    let heading = if self.structure == CustomStructure {
      String::from(" Custom template ")
    } else {
      format!(" {} {} template ", display_custom, self.structure)
    };
    println!("{:-^163}", heading);
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let mut prev_i = 100;
    for (i, cont) in self.get_display_content_vec() {
      let display_i = if i == prev_i {
        String::from("   ")
      } else {
        let d_i = i + 1;
        format!(" {} ", d_i)
      };
      println!("{:-^20} | {: <140}", display_i, &format!(" {} ", cont));
      prev_i = i;
    }
    println!("{:-^163}", "-");
  }
  pub fn display_edit_content(&self, blank_focus_id: Option<u32>, content_focus_id: Option<u32>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{:-^163}", "-");
    let heading = format!(" Edit custom {} template ", self.structure);
    println!("{:-^163}", heading);
    println!("{:-^163}", "-");
    println!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println!("{:-^163}", "-");
    let (display_content_string, formatting) = self.generate_display_content_string_with_blanks(blank_focus_id, content_focus_id);
    println!("{}", display_content_string);
    let mut prev_i = 100; // 0 is the actual index
    for (i, cont, f) in NoteTemplate::get_display_content_vec_from_string(display_content_string, Some(formatting)) {
      println!("{}, {}, {:?}", i, cont, f);
      // f is Option<Vec<(String, usize, usize)>>
      let display_i = if i == prev_i {
        String::from("   ")
      } else {
        let display_i = i + 1;
        format!(" {} ", display_i)
      };
      prev_i = i;
      match f {
        None => println!("{:-^20} | {:-^140}", display_i, ANSIString::from(&cont)),
        Some(f_vec) => {
          print!("{:-^20} | ", display_i);
          if f_vec.len() == 0 {
            print!("{}", Style::new().paint(&cont));
          } else {
            for (s, idx1, idx2) in f_vec {
              let to_format = &cont[idx1..idx2];
              match &s[..] {
                "HIGHLIGHTED CONTENT" => {
                  print!("{: <140}", Black.on(Cyan).italic().paint(to_format));
                },
                "UNHIGHLIGHTED CONTENT" => {
                  print!("{: <140}", Cyan.on(Blue).paint(to_format));
                },
                "CONTENT" => {
                  print!("{: <140}", Style::new().paint(to_format));
                },
                "HIGHLIGHTED BLANK" => {
                  print!("{: <140}", Black.on(Yellow).bold().paint(to_format));
                },
                "UNHIGHLIGHTED BLANK" => {
                  print!("{: <140}", Black.on(White).paint(to_format));
                },
                "BLANK" => {
                  print!("{}", Black.on(White).paint(to_format));
                },
                _ => (),
              }
            }
          }
        }
      }
      print!("\n");
    }
    println!("{:-^163}", "-");
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
    if self.content.len() > 95 {
      &self.content[0..95]
    } else {
      &self.content[..]
    }
  }


  pub fn get_content_vec_from_string(display_content: String) -> Vec<(usize, String)> {
    let display_content_vec: Vec<String> = display_content.split(". ").map(|s| s.to_string() ).collect();
    let mut length_adjusted_vec = vec![];
    for (i, sent) in display_content_vec.iter().enumerate() {
      if sent.chars().count() > 0 {
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
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
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
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    let mut blank_content = self.content.clone();
    let mut blanks: Vec<Blank> = vec![];
    while RE_BLANK.is_match(&blank_content) {
      let m = RE_BLANK.find(&blank_content).unwrap();
      blanks.push(Blank::get_blank_from_str(&blank_content[m.start()..m.end()]));
      blank_content = RE_BLANK.replace(&blank_content, "X").to_string();
    }
    blanks
  }
  pub fn get_empty_blanks_and_indexes(&self) -> Vec<(u32, Blank)> {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    let num_total_blanks = RE_BLANK.find_iter(&self.content).count();
    let ordered_blanks = self.get_blank_types();
    let mut blanks: Vec<(u32, Blank)> = vec![];
    for i in 0..num_total_blanks {
      let idx = i as u32 + 1;
      match self.blanks.get(&idx) {
        Some(b_tup) => (),
        None => {
          blanks.push((idx, ordered_blanks[i as usize]));
        }
      }
    }
    blanks
  }
  pub fn has_unfilled_blanks(&self) -> bool {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    RE_BLANK.find_iter(&self.content[..]).count() < self.blanks.len()
  }
  pub fn number_of_blanks(&self) -> usize {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    RE_BLANK.find_iter(&self.content[..]).count()
  }
  pub fn display_content(&self) {
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
        let d_i = i + 1;
        format!(" {} ", d_i)
      } else {
        String::from("   ")
      };
      let cont = format!(" {} ", cont);
      println!("{:-^20} | {: <140}", display_i, cont);
      current_i = i;
    }
    println!("{:-^163}", "-");
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
          _ => panic!("Failed to read Blank type from string: {}", &format!("{}{}{}", components[0], components[1], components[2])),
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
        format!("Possessive determiner pronouns of the person in blank #{} (his, her, their)", b_id)
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
  pub fn display_to_user_empty(&self) -> String {
    match self {
      CurrentUser => String::from("[ Current user ]"),
      CurrentClientName => String::from("[ Name of client ]"),
      Collaterals => String::from("[ One or more collaterals ]"),
      AllCollaterals => String::from("[ All collaterals for the current client ]"),
      Pronoun1ForBlank(_) => {
        format!("[ Subject pronoun of the person in another blank (he, she, they) ]")
      },
      Pronoun2ForBlank(_) => {
        format!("[ Object pronoun of the person in another blank (him, her, them) ]")
      },
      Pronoun3ForBlank(_) => {
        format!("[ Possessive determiner of the person in another blank (his, her, their) ]")
      },
      Pronoun4ForBlank(_) => {
        format!("[ Possessive pronoun of the person in another blank (his, hers, theirs) ]")
      },
      Pronoun1ForUser => String::from("[ Subject pronoun of the current user ]"),
      Pronoun2ForUser => String::from("[ Object pronoun of the current user ]"),
      Pronoun3ForUser => String::from("[ Possessive determiner of the current user ]"),
      Pronoun4ForUser => String::from("[ Possessive pronoun of the current user ]"),
      Pronoun1ForClient => String::from("[ Subject pronoun of the current client ]"),
      Pronoun2ForClient => String::from("[ Object pronoun of the current client ]"),
      Pronoun3ForClient => String::from("[ Possessive determiner of the current client ]"),
      Pronoun4ForClient => String::from("[ Possessive pronoun of the current client ]"),
      TodayDate => String::from("[ Today's date ]"),
      NoteDayDate => String::from("[ The date of the current note ]"),
      InternalDocument => String::from("[ Internal document ]"),
      ExternalDocument => String::from("[ External document ]"),
      InternalMeeting => String::from("[ Wraparound meeting title ]"),
      ExternalMeeting => String::from("[ External meeting title ]"),
      Action => String::from("[ General action ]"),
      Phrase => String::from("[ Other phrase ]"),
      CustomBlank => String::from("[ Custom input ]"),
    }
  }
}

impl fmt::Display for Blank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(---{}---)", self.abbreviate())
  }
}