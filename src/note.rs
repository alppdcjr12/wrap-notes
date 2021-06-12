use std::fmt;
use std::collections::HashMap;
use std::convert::TryFrom;
use ansi_term::Colour::{Black, Yellow, White, RGB};
use ansi_term::{Style};
use chrono::{NaiveDate, Datelike, Weekday};

// bold, dimmed, italic, underline, blink, reverse, hidden, strikethrough, on

use crate::constants::*;

use lazy_static::lazy_static;
use regex::Regex;

use std::format_args;

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
// print unhighlighted content
pub fn print_unhighlighted_content(s: String) {
  print!("{}", RGB(25, 225, 225).on(RGB(0, 0, 255)).paint(s));
}
macro_rules! print_unhighlighted_content {
    ($($arg:tt)*) => (print_unhighlighted_content(format!("{}", format_args!($($arg)*))));
}
// print highlighted content
pub fn print_highlighted_content(s: String) {
  print!("{}", Black.on(RGB(25, 225, 225)).paint(s));
}
macro_rules! print_highlighted_content {
    ($($arg:tt)*) => (print_highlighted_content(format!("{}", format_args!($($arg)*))));
}
// print unfocused blank
pub fn print_unfocused_blank(s: String) {
  print!("{}", Black.on(RGB(160, 160, 160)).paint(s));
}
macro_rules! print_unfocused_blank {
    ($($arg:tt)*) => (print_unfocused_blank(format!("{}", format_args!($($arg)*))));
}

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
  pub fn new(
    id: u32,
    structure: StructureType,
    custom: bool,
    content: String,
    user_ids: Vec<u32>,
  ) -> NoteTemplate {
    let foreign_key: HashMap<String, u32> = HashMap::new();
    let foreign_keys: HashMap<String, Vec<u32>> = [
      (String::from("note_ids"), vec![]),
      (String::from("user_ids"), user_ids),
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
  pub fn clean_spacing(&mut self) {
    self.content = self.content.split(". ").map(|s| s.trim().to_string() ).collect::<Vec<String>>().join(". ");
    if self.content.len() >= 1 {
      if &self.content[self.content.len()-1..] == "." {
        self.content = format!("{}{}", &self.content[..self.content.len()-1].trim(), ".");
      }
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
    let mut prev_end_idx = 0;
    
    loop {
      let find_match_string = content_string.clone();
      let m = RE_BLANK.find(&find_match_string);
      let m = match m {
        None => break,
        Some(m) => m,
      };

      let mut display_blank = String::new();
      for _i in m.start()..m.end() {
        display_blank.push_str("X");
      }

      content_string = format!(
        "{}{}{}",
        &content_string[..m.start()],
        display_blank,
        &content_string[m.end()..]
      );

      typed_content_indices.push((prev_end_idx, m.start()));
      prev_end_idx = m.end();
    }
    if prev_end_idx < content_string.len() {
      typed_content_indices.push((prev_end_idx, content_string.len()))
    }
    typed_content_indices
  }
  pub fn get_blanks_with_new_ids(mut blanks: Vec<Blank>, position: u32) -> Vec<Blank> {
    let copied_blanks = blanks.clone();

    for (i, b) in copied_blanks.iter().enumerate() {
      match b {
        Pronoun1ForBlank(id) => {
          if id.unwrap() + 1 >= position {
            blanks = blanks.iter().enumerate().map(|(oi, _ob)| 
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
            blanks = blanks.iter().enumerate().map(|(oi, _ob)| 
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
            blanks = blanks.iter().enumerate().map(|(oi, _ob)| 
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
            blanks = blanks.iter().enumerate().map(|(oi, _ob)| 
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
  pub fn get_num_content_sections(&self) -> usize {
    let mut num_contents = 0;
    let typed_content_indices = self.get_typed_content_indices();
    for (i1, i2) in typed_content_indices {
      let sentence_indices = if i2 == self.content.len() {
        NoteTemplate::get_sentence_end_indices(0, String::from(&self.content[i1..i2]))
      } else {
        NoteTemplate::get_sentence_end_indices(0, String::from(&self.content[i1..i2+1]))
      };
      for _idcs in sentence_indices {
        num_contents += 1;
      }
    }
    num_contents
  }
  pub fn get_content_section_indices(&self) -> Vec<(usize, usize)> {
    let mut num_contents: Vec<(usize, usize)> = vec![];
    let typed_content_indices = self.get_typed_content_indices();
    for (i1, i2) in typed_content_indices {
      let sentence_indices = if i2 == self.content.len() {
        NoteTemplate::get_sentence_end_indices(i1, String::from(&self.content[i1..i2]))
      } else {
        NoteTemplate::get_sentence_end_indices(i1, String::from(&self.content[i1..i2+1]))
      };
      for idcs in sentence_indices {
        num_contents.push(idcs);
      }
    }
    num_contents
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
    if num_chars > 0 {
      output_vec.push((current_idx + offset, (current_idx + offset + num_chars) - 1 ));
    } else if output_vec.len() == 0 {
      // the only content would be the empty content at the beginning of the empty string, since no other indices
      output_vec.push((current_idx, current_idx));
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
                let sentence_indices = NoteTemplate::get_sentence_end_indices(0, find_match_string.clone());
                let last_tuple = sentence_indices[sentence_indices.len()-1].clone();
                for (idx1, idx2) in sentence_indices {
                  let mut idx2_updated = idx2+1;
                  if idx1 == last_tuple.0 && idx2 == last_tuple.1  {
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
                let sentence_indices = NoteTemplate::get_sentence_end_indices(0, find_match_string.clone());
                let last_tuple = sentence_indices[sentence_indices.len()-1];
                for (idx1, idx2) in sentence_indices {
                  let mut idx2_updated = idx2+1;
                  if idx1 == last_tuple.0 && idx2 == last_tuple.1 {
                    idx2_updated += 2;
                  }
                  format_vec.push((String::from("CONTENT"), idx1, idx2_updated));
                }
              },
            }
            break;
          } else {
            if prev_end_idx < find_match_string.chars().count() {
              match content_focus_id {
                Some(focus_id) => {
                  let sentence_indices = NoteTemplate::get_sentence_end_indices(
                    prev_end_idx,
                    format!("{}", &content_string[prev_end_idx..]),
                  );
                  let last_tuple = sentence_indices[sentence_indices.len()-1];
                  for (idx1, idx2) in sentence_indices.clone() {
                    let num_to_add = if idx1 == idx2 && idx1 == 0 {
                      0
                    } else {
                      1
                    };
                    let mut adjust_last_index = 0;
                    if (idx1, idx2) == last_tuple {
                      match &content_string[idx2..] {
                        "." => if sentence_indices.len() > 1 {
                          adjust_last_index += 1;
                        }
                        _ => adjust_last_index += 1,
                      }
                    }
                    let display_content = format!("[{}]: {}", cont_i, &content_string[idx1..idx2+num_to_add]);
                    let cidx1 = content.chars().count();
                    content.push_str(&display_content);
                    let cidx2 = content.chars().count();
                    if focus_id == cont_i {
                      format_vec.push((String::from("HIGHLIGHTED CONTENT"), cidx1, cidx2+adjust_last_index));
                    } else {
                      format_vec.push((String::from("UNHIGHLIGHTED CONTENT"), cidx1, cidx2+adjust_last_index));
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
                  let last_tuple = sentence_indices[sentence_indices.len()-1].clone();
                  content.push_str(&end_string);
                  for (idx1, idx2) in sentence_indices {
                    let num_to_add = match &end_string[end_string.len()-1..] {
                      "." => 2,
                      _ => {
                        if (idx1, idx2) == last_tuple {
                          2
                        } else {
                          1
                        }
                      },
                    };
                    format_vec.push((String::from("CONTENT"), idx1, idx2+num_to_add));
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

          let cidx1 = content.chars().count();
          content.push_str(&display_content);
          let cidx2 = content.chars().count();

          if cidx1 != cidx2 {
            let sentence_indices = NoteTemplate::get_sentence_end_indices(
              cidx1,
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
          for (idx1, idx2) in sentence_indices.clone() {
            let num_to_add = if idx1 == idx2 && idx1 == 0 {
              0
            } else {
              1
            };
            let display_content = format!("[{}]: {}", cont_i, &String::from(&content_string[idx1..idx2+num_to_add]));
            let cidx1 = content.chars().count();
            content.push_str(&display_content);
            let cidx2 = content.chars().count();
            if f_id == cont_i {
              format_vec.push((String::from("HIGHLIGHTED CONTENT"), cidx1, cidx2));
            } else {
              format_vec.push((String::from("UNHIGHLIGHTED CONTENT"), cidx1, cidx2));
            }
            // format_vec.push((String::from(&content_string[last_tuple.0..last_tuple.1]), last_tuple.0, last_tuple.1));
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
            match content_focus_id {
              Some(_) => format_vec.push((String::from("UNFOCUSED BLANK"), bidx1, bidx2)),
              None => format_vec.push((String::from("BLANK"), bidx1, bidx2)),
            }
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
  pub fn num_blanks_before_idx(&self, idx: usize) -> usize {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    RE_BLANK.find_iter(&self.content[..idx]).count()
  }
  pub fn get_display_content_vec_from_string(display_content: String, color_formatting: Option<Vec<(String, usize, usize)>>) -> Vec<(usize, String, Option<Vec<(String, usize, usize)>>)> {
    let display_content_vec: Vec<String> = display_content.split(". ").map(|s| s.to_string() ).collect();
    let mut length_adjusted_vec = vec![];
    let mut current_idx = 0;
    for (i, sent) in display_content_vec.iter().enumerate() {
      if sent.chars().count() > 0 {
        let mut sentence = sent.clone();
        if i != display_content_vec.len() - 1 && sentence != String::from("") {
          sentence.push_str(". ");
        }
        if sentence.chars().count() < 140 {
          match color_formatting.clone() {
            None => length_adjusted_vec.push((i, sentence.clone(), None)),
            Some(f) => {
              let sf1: Vec<(String, usize, usize)> = f.iter()
                .filter(|(_, i1, i2)| i1 >= &current_idx && i2 <= &(sentence.chars().count() + current_idx + 1) )
                .map(|(s, i1, i2)| (s.to_string(), i1-current_idx, i2-current_idx) )
                .collect();

              let only_one = sf1.len() == 1;
              
              let sf2 = if i == display_content_vec.len() - 1 {
                sf1.iter()
                  .map(|(s, i1, i2)| if i == display_content_vec.len() - 1 {
                      if (&sentence[i2-1..] == ". " || only_one) && !String::from("UNHIGHLIGHED BLANK UNFOCUSED BLANK").contains(s) {
                        // if 
                        (s.to_string(), *i1, i2-1)
                      } else {
                        (s.to_string(), *i1, *i2)
                      }
                    } else {
                      (s.to_string(), *i1, *i2)
                    }
                  )
                  .collect::<Vec<(String, usize, usize)>>()
              } else {
                sf1.clone()
              };


              length_adjusted_vec.push((i, sentence.clone(), Some(sf2)));
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
                        let last_divider_idx = f.iter().find(|(_s, i1, i2)| i1 > &current_idx && i2 >= &(current_idx+140) );
                        match last_divider_idx {
                          None => {
                            let sentence_formatting: Vec<(String, usize, usize)> = f.iter()
                              .filter(|(_s, i1, i2)| i1 > &current_idx && i2 <= &(current_idx+140+1) )
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
                              .filter(|(_s, i1, i2)| i1 >= &current_idx && i2 <= &(current_idx+pos) )
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
                          .filter(|(_s, i1, i2)| i1 > &current_idx && i2 <= &(current_idx+140) )
                          .map(|(s, i1, i2)|
                            if i2 > &(current_idx+spc) {
                              (s.to_string(), i1-current_idx, spc)
                            } else {
                              (s.to_string(), i1-current_idx, i2-current_idx)
                            }
                          )
                          .collect();
                        length_adjusted_vec.push((i, String::from(&long_sent[..spc]), Some(sentence_formatting)));
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
                .filter(|(_, i1, i2)| i1 >= &current_idx && i2 <= &(long_sent.chars().count() + current_idx + 1) )
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
  pub fn display_content(&self, blank_focus_id: Option<u32>, content_focus_id: Option<u32>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^163}", "-");
    let display_custom = if self.custom { "Custom" } else { "Default" };
    let heading = if self.structure == CustomStructure {
      String::from(" Custom template ")
    } else {
      format!(" {} {} template ", display_custom, self.structure)
    };
    println_on_bg!("{:-^163}", heading);
    println_on_bg!("{:-^163}", "-");
    println_on_bg!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println_on_bg!("{:-^163}", "-");
    let (display_content_string, formatting) = self.generate_display_content_string_with_blanks(blank_focus_id, content_focus_id);
    let mut prev_i = 100; // 0 is the actual index
    let display_content_vec = NoteTemplate::get_display_content_vec_from_string(display_content_string, Some(formatting));
    for (i, cont, f) in display_content_vec {
      let num_chars = cont.chars().count();
      let num_to_add = if num_chars < 140 { 140-num_chars-1 } else { 0 };
      // f is Option<Vec<(String, usize, usize)>>
      let display_i = if i == prev_i {
        String::from("   ")
      } else {
        let display_i = i + 1;
        format!(" {} ", display_i)
      };
      prev_i = i;
      match f {
        None => println_on_bg!("{:-^20} | {:-^140}", display_i, &cont),
        Some(f_vec) => {
          print_on_bg!("{:-^20} |  ", display_i);
          if f_vec.len() == 0 {
            print_on_bg!("{: <140}", &cont);
          } else {
            for (s, idx1, idx2) in f_vec {
              let to_format = if idx2 >= cont.len() {
                &cont[idx1..]
              } else {
                &cont[idx1..idx2]
              };
              match &s[..] {
                "HIGHLIGHTED CONTENT" => {
                  print_highlighted_content!("{}", to_format);
                },
                "UNHIGHLIGHTED CONTENT" => {
                  print_unhighlighted_content!("{}", to_format);
                },
                "CONTENT" => {
                  print_on_bg!("{}", to_format);
                },
                "HIGHLIGHTED BLANK" => {
                  print!("{}", Black.on(Yellow).bold().paint(to_format));
                },
                "UNHIGHLIGHTED BLANK" => {
                  print!("{}", Black.on(White).paint(to_format));
                },
                "UNFOCUSED BLANK" => {
                  print_unfocused_blank!("{}", to_format);
                },
                "BLANK" => {
                  print!("{}", Black.on(White).paint(to_format));
                },
                _ => (),
              }
            }
            for _ in 0..num_to_add {
              print_on_bg!(" ");
            }
          }
        }
      }
      print!("\n");
    }
    println_on_bg!("{:-^163}", "-");
  }
  pub fn get_content_indices(&self) -> Vec<(usize, usize)> {
    let (_, formatting) = self.generate_display_content_string_with_blanks(None, None);
    formatting.iter().filter(|(s, _, _)| !String::from("UNHIGHLIGHTED BLANK UNFOCUSED BLANK").contains(s) ).map(|(_, u1, u2)| (*u1, *u2) ).collect::<Vec<(usize, usize)>>()
  }
  pub fn display_edit_content(&self, blank_focus_id: Option<u32>, content_focus_id: Option<u32>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println_on_bg!("{:-^163}", "-");
    let heading = format!(" Edit custom {} template ", self.structure);
    println_on_bg!("{:-^163}", heading);
    println_on_bg!("{:-^163}", "-");
    println_on_bg!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println_on_bg!("{:-^163}", "-");
    let (display_content_string, formatting) = self.generate_display_content_string_with_blanks(blank_focus_id, content_focus_id);
    let mut prev_i = 100; // 0 is the actual index
    let display_content_vec = NoteTemplate::get_display_content_vec_from_string(display_content_string, Some(formatting));
    for (i, cont, f) in display_content_vec {
      let num_chars = cont.chars().count();
      let num_to_add = if num_chars < 140 { 140-num_chars-1 } else { 0 };
      // f is Option<Vec<(String, usize, usize)>>
      let display_i = if i == prev_i {
        String::from("   ")
      } else {
        let display_i = i + 1;
        format!(" {} ", display_i)
      };
      prev_i = i;
      match f {
        None => println_on_bg!("{:-^20} | {:-^140}", display_i, &cont),
        Some(f_vec) => {
          print_on_bg!("{:-^20} |  ", display_i);
          if f_vec.len() == 0 {
            print_on_bg!("{: <140}", &cont);
          } else {
            for (s, idx1, idx2) in f_vec {
              let to_format = if idx2 >= cont.len() {
                &cont[idx1..]
              } else {
                &cont[idx1..idx2]
              };
              match &s[..] {
                "HIGHLIGHTED CONTENT" => {
                  print_highlighted_content!("{}", to_format);
                },
                "UNHIGHLIGHTED CONTENT" => {
                  print_unhighlighted_content!("{}", to_format);
                },
                "CONTENT" => {
                  print_on_bg!("{}", to_format);
                },
                "HIGHLIGHTED BLANK" => {
                  print!("{}", Black.on(Yellow).bold().paint(to_format));
                },
                "UNHIGHLIGHTED BLANK" => {
                  print!("{}", Black.on(White).paint(to_format));
                },
                "UNFOCUSED BLANK" => {
                  print!("{}", Black.on(RGB(160, 160, 160)).paint(to_format));
                },
                "BLANK" => {
                  print!("{}", Black.on(White).paint(to_format));
                },
                _ => (),
              }
            }
            for _ in 0..num_to_add {
              print_on_bg!(" ");
            }
          }
        }
      }
      print!("\n");
    }
    println_on_bg!("{:-^163}", "-");
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
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
      static ref RE_PIPE: Regex = Regex::new(r#" \| "#).unwrap();
      static ref RE_L: Regex = Regex::new(r#"\(---"#).unwrap();
      static ref RE_R: Regex = Regex::new(r#"---\)"#).unwrap();
    }
    let find_match_string = self.content.clone();
    let matches = RE_BLANK.find_iter(&find_match_string);
    
    let mut changed_content = self.content.clone();
    for m in matches {
      let mut replacement = String::new();
      for _i in m.start()..m.end() {
        replacement.push_str("X");
      }
      changed_content = format!(
        "{}{}{}",
        &changed_content[..m.start()],
        &replacement,
        &changed_content[m.end()..]
      );
    }
    let match_pipe = RE_PIPE.find_iter(&changed_content);
    let match_l = RE_L.find_iter(&changed_content);
    let match_r = RE_R.find_iter(&changed_content);
    
    let mut content = self.content.clone();
    for m in match_pipe {
      content = format!(
        "{}{}{}",
        &content[..m.start()],
        " / ",
        &content[m.end()..]
      );
    }
    for m in match_l {
      content = format!(
        "{}{}{}",
        &content[..m.start()],
        "(-- ",
        &content[m.end()..]
      );
    }
    for m in match_r {
      content = format!(
        "{}{}{}",
        &content[..m.start()],
        " --)",
        &content[m.end()..]
      );
    }
    write!(
      f,
      "{} | {} | {} | {} | {}\n",
      &self.id,
      &self.structure,
      &content,
      &self.foreign_keys["user_ids"]
        .iter()
        .map(|id| format!("{}", id))
        .collect::<Vec<String>>()
        .join("#"),
      &self.foreign_keys["note_ids"]
        .iter()
        .map(|id| format!("{}", id))
        .collect::<Vec<String>>()
        .join("#"),
    )
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Eq)]
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

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum FPIntervention {
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
}

impl FPIntervention {
  pub fn iterator() -> impl Iterator<Item = FPIntervention> {
    [
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
    ].iter().copied()
  }
}

impl fmt::Display for FPIntervention {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      FaceToFaceContact => "Face to face contact",
      CollateralContact => "Collateral contact",
      CrisisSupport => "Crisis support",
      TelephoneSupport => "Telephone support",
      DirectTimeWithProviders => "Direct time with providers",
      EducatingCoachingModelingAndGuiding => "Educating, coaching, modeling and guiding",
      EngageParentCaregiverInAddressingGoals => "Engage parent/caregiver in addressing goals",
      TeachAdvocacyGuideLinkageToResources => "Teach advocacy, guide linkage to resources",
      TeachNetworkingInCommunityAndWithProviders => "Teach networking in community and with providers",
      ProviderOutreachToPerson => "Provider outreach to person",
      MemberTransportationByStaff => "Member transportation by staff",
      NoShowLateCancellation => "No show/late cancellation",
      FPInterventionDocumentation => "Documentation",
      Other => "Other",
    };
    write!(f, "{}", display)
  }
}

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

impl PartialEq for FPIntervention {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&FaceToFaceContact, &FaceToFaceContact) => true,
      (&CollateralContact, &CollateralContact) => true,
      (&CrisisSupport, &CrisisSupport) => true,
      (&TelephoneSupport, &TelephoneSupport) => true,
      (&DirectTimeWithProviders, &DirectTimeWithProviders) => true,
      (&EducatingCoachingModelingAndGuiding, &EducatingCoachingModelingAndGuiding) => true,
      (&EngageParentCaregiverInAddressingGoals, &EngageParentCaregiverInAddressingGoals) => true,
      (&TeachAdvocacyGuideLinkageToResources, &TeachAdvocacyGuideLinkageToResources) => true,
      (&TeachNetworkingInCommunityAndWithProviders, &TeachNetworkingInCommunityAndWithProviders) => true,
      (&ProviderOutreachToPerson, &ProviderOutreachToPerson) => true,
      (&MemberTransportationByStaff, &MemberTransportationByStaff) => true,
      (&NoShowLateCancellation, &NoShowLateCancellation) => true,
      (&FPInterventionDocumentation, &FPInterventionDocumentation) => true,
      (&Other, &Other) => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum FPNoteCategory {
  DescriptionOfIntervention(Option<FPIntervention>),
  ResponseToIntervention(Option<FPIntervention>),
  Functioning,
  PlanAdditionalInformation,
}

use FPNoteCategory::{
  DescriptionOfIntervention,
  ResponseToIntervention,
  Functioning,
  PlanAdditionalInformation,
};

impl FPNoteCategory {
  pub fn iterator() -> impl Iterator<Item = FPNoteCategory> {
    [
      DescriptionOfIntervention(None),
      ResponseToIntervention(None),
      Functioning,
      PlanAdditionalInformation,
    ].iter().copied()
  }
  pub fn iterator_of_descriptions() -> impl Iterator<Item = FPNoteCategory> {
    [
      DescriptionOfIntervention(Some(FaceToFaceContact)),
      DescriptionOfIntervention(Some(CollateralContact)),
      DescriptionOfIntervention(Some(CrisisSupport)),
      DescriptionOfIntervention(Some(TelephoneSupport)),
      DescriptionOfIntervention(Some(DirectTimeWithProviders)),
      DescriptionOfIntervention(Some(EducatingCoachingModelingAndGuiding)),
      DescriptionOfIntervention(Some(EngageParentCaregiverInAddressingGoals)),
      DescriptionOfIntervention(Some(TeachAdvocacyGuideLinkageToResources)),
      DescriptionOfIntervention(Some(TeachNetworkingInCommunityAndWithProviders)),
      DescriptionOfIntervention(Some(ProviderOutreachToPerson)),
      DescriptionOfIntervention(Some(MemberTransportationByStaff)),
      DescriptionOfIntervention(Some(NoShowLateCancellation)),
      DescriptionOfIntervention(Some(FPInterventionDocumentation)),
      DescriptionOfIntervention(Some(Other)),
    ].iter().copied()
  }
  pub fn iterator_of_responses() -> impl Iterator<Item = FPNoteCategory> {
    [
      ResponseToIntervention(Some(FaceToFaceContact)),
      ResponseToIntervention(Some(CollateralContact)),
      ResponseToIntervention(Some(CrisisSupport)),
      ResponseToIntervention(Some(TelephoneSupport)),
      ResponseToIntervention(Some(DirectTimeWithProviders)),
      ResponseToIntervention(Some(EducatingCoachingModelingAndGuiding)),
      ResponseToIntervention(Some(EngageParentCaregiverInAddressingGoals)),
      ResponseToIntervention(Some(TeachAdvocacyGuideLinkageToResources)),
      ResponseToIntervention(Some(TeachNetworkingInCommunityAndWithProviders)),
      ResponseToIntervention(Some(ProviderOutreachToPerson)),
      ResponseToIntervention(Some(MemberTransportationByStaff)),
      ResponseToIntervention(Some(NoShowLateCancellation)),
      ResponseToIntervention(Some(FPInterventionDocumentation)),
      ResponseToIntervention(Some(Other)),
    ].iter().copied()
  }
}

impl fmt::Display for FPNoteCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display = match self {
      ResponseToIntervention(fp_intervention) | DescriptionOfIntervention(fp_intervention) => {
        let main_display = match self {
          DescriptionOfIntervention(_) => String::from("Description of"),
          ResponseToIntervention(_) => String::from("Response to"),
          _ => panic!("Other FP Note Category patterns should already have been filtered out."),
        };
        match fp_intervention {
          Some(intervention) => {
            match intervention {
              FaceToFaceContact => format!("{} 'face to face contact'", main_display),
              CollateralContact => format!("{} 'collateral contact'", main_display),
              CrisisSupport => format!("{} 'crisis support'", main_display),
              TelephoneSupport => format!("{} 'telephone support'", main_display),
              DirectTimeWithProviders => format!("{} 'direct time with providers'", main_display),
              EducatingCoachingModelingAndGuiding => format!("{} 'educating, coaching, modeling and guiding'", main_display),
              EngageParentCaregiverInAddressingGoals => format!("{} 'engage parent/caregiver in addressing goals'", main_display),
              TeachAdvocacyGuideLinkageToResources => format!("{} 'teach advocacy, guide linkage to resources'", main_display),
              TeachNetworkingInCommunityAndWithProviders => format!("{} 'teach networking in community and with providers'", main_display),
              ProviderOutreachToPerson => format!("{} 'provider outreach to person'", main_display),
              MemberTransportationByStaff => format!("{} 'member transportation by staff'", main_display),
              NoShowLateCancellation => format!("{} 'no show/late cancellation'", main_display),
              FPInterventionDocumentation => format!("{} 'documentation'", main_display),
              Other => format!("{} 'other'", main_display),
            }
          },
          None => {
            format!("{} interventions", main_display)
          },
        }
      }
      Functioning => String::from("Functioning"),
      PlanAdditionalInformation => String::from("Plan/additional information"),
    };
    write!(f, "{}", &display)
  }
}

impl PartialEq for FPNoteCategory {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&Functioning, &Functioning) => true,
      (&PlanAdditionalInformation, &PlanAdditionalInformation) => true,
      (&DescriptionOfIntervention(int_a), &DescriptionOfIntervention(int_b)) | (&ResponseToIntervention(int_a), &ResponseToIntervention(int_b)) => {
        if int_a == int_b {
          true
        } else {
          false
        }
      },
      _ => false,
    }
  }
}

#[derive(Clone)]
pub struct Note {
  pub id: u32,
  pub date: NaiveDate,
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
    date: NaiveDate,
    category: NoteCategory,
    structure: StructureType,
    content: String,
    user_id: u32,
    client_id: u32,
    collateral_ids: Vec<u32>
  ) -> Note {
    let blanks = HashMap::new();
    let foreign_key: HashMap<String, u32> = [
      (String::from("user_id"), user_id),
      (String::from("client_id"), client_id),
    ].iter().cloned().collect();
    let foreign_keys: HashMap<String, Vec<u32>> = [
      (String::from("collateral_ids"), collateral_ids),
    ].iter().cloned().collect();
    Note {
      id,
      date,
      category,
      structure,
      content,
      blanks,
      foreign_key,
      foreign_keys,
    }
  }
  pub fn fmt_date(&self) -> String {
    self.date.format("%Y-%m-%d").to_string()
  }
  pub fn heading_date(&self) -> String {
    let wd = match self.date.weekday() {
      Weekday::Mon => "Monday",
      Weekday::Tue => "Tuesday",
      Weekday::Wed => "Wednesday",
      Weekday::Thu => "Thursday",
      Weekday::Fri => "Friday",
      Weekday::Sat => "Saturday",
      Weekday::Sun => "Sunday",
    };
    format!("{} {}/{}", wd, self.date.month(), self.date.day())
  }
  pub fn get_num_content_sections(&self) -> usize {
    self.get_content_section_indices().iter().count()
  }
  pub fn get_typed_content_indices(&self) -> Vec<(usize, usize)> {
    let mut content_string = self.content.clone();

    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    
    let mut typed_content_indices: Vec<(usize, usize)> = vec![];
    let mut prev_end_idx = 0;
    
    loop {
      let find_match_string = content_string.clone();
      let m = RE_BLANK.find(&find_match_string);
      let m = match m {
        None => break,
        Some(m) => m,
      };

      let mut display_blank = String::new();
      for _ in m.start()..m.end() {
        display_blank.push_str("X");
      }

      content_string = format!(
        "{}{}{}",
        &content_string[..m.start()],
        display_blank,
        &content_string[m.end()..]
      );

      typed_content_indices.push((prev_end_idx, m.start()));
      prev_end_idx = m.end();
    }
    if prev_end_idx < content_string.len() {
      typed_content_indices.push((prev_end_idx, content_string.len()))
    }
    typed_content_indices
  }
  pub fn get_content_section_indices(&self) -> Vec<(usize, usize)> {
    let mut num_contents: Vec<(usize, usize)> = vec![];
    let typed_content_indices = self.get_typed_content_indices();
    for (i1, i2) in typed_content_indices {
      let sentence_indices = if i2 == self.content.len() {
        NoteTemplate::get_sentence_end_indices(i1, String::from(&self.content[i1..i2]))
      } else {
        NoteTemplate::get_sentence_end_indices(i1, String::from(&self.content[i1..i2+1]))
      };
      for idcs in sentence_indices {
        num_contents.push(idcs);
      }
    }
    num_contents
  }
  pub fn add_blank(&mut self, blank: Blank) {
    if self.content.len() == 0 {
      self.content.push_str(&blank.to_string())
    } else {
      match &self.content[self.content.len()-1..] {
        " " => {
          self.content.push_str(&blank.to_string())
        },
        _ => {
          self.content.push_str(&format!(" {}", blank.to_string()));
        }
      }
    }
  }
  pub fn remove_blanks_after_content_index(&mut self, idx: usize) {
    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
    }
    let matches = RE_BLANK.find_iter(&self.content);
    for (i, m) in matches.enumerate() {
      if m.start() > idx {
        self.blanks.remove(&(i as u32));
      }
    }
  }
  pub fn preview(&self) -> &str {
    if self.content.len() > 95 {
      &self.content[0..95]
    } else {
      &self.content[..]
    }
  }
  pub fn clean_spacing(&mut self) {
    self.content = self.content.split(". ").map(|s| s.trim().to_string() ).collect::<Vec<String>>().join(". ");
    if self.content.len() >= 1 {
      if &self.content[self.content.len()-1..] == "." {
        self.content = format!("{}{}", &self.content[..self.content.len()-1].trim(), ".");
      }
    }
  }
  pub fn delete_associated_pronouns(&mut self, i: &u32) {
    for (idx, blank_tup) in &self.blanks.clone() {
      match blank_tup.0 {
        Pronoun1ForBlank(id_opt) | Pronoun2ForBlank(id_opt) | Pronoun3ForBlank(id_opt) | Pronoun4ForBlank(id_opt) => {
          match id_opt {
            None => (),
            Some(pronoun_blank_id) => {
              if pronoun_blank_id == *i {
                self.blanks.remove(&idx);
              }
            }
          }
        },
        _ => (),
      }
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
    let mut current_blank: u32 = 0;
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
                let sentence_indices = NoteTemplate::get_sentence_end_indices(0, find_match_string.clone());
                let last_tuple = sentence_indices[sentence_indices.len()-1].clone();
                for (idx1, idx2) in sentence_indices {
                  let mut idx2_updated = idx2+1;
                  if idx1 == last_tuple.0 && idx2 == last_tuple.1  {
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
                let sentence_indices = NoteTemplate::get_sentence_end_indices(0, find_match_string.clone());
                let last_tuple = sentence_indices[sentence_indices.len()-1];
                for (idx1, idx2) in sentence_indices {
                  let mut idx2_updated = idx2+1;
                  if idx1 == last_tuple.0 && idx2 == last_tuple.1 {
                    idx2_updated += 2;
                  }
                  format_vec.push((String::from("CONTENT"), idx1, idx2_updated));
                }
              },
            }
            break;
          } else {
            if prev_end_idx < find_match_string.chars().count() {
              match content_focus_id {
                Some(focus_id) => {
                  let sentence_indices = NoteTemplate::get_sentence_end_indices(
                    prev_end_idx,
                    format!("{}", &content_string[prev_end_idx..]),
                  );
                  let last_tuple = sentence_indices[sentence_indices.len()-1];
                  for (idx1, idx2) in sentence_indices.clone() {
                    let num_to_add = if idx1 == idx2 && idx1 == 0 {
                      0
                    } else {
                      1
                    };
                    let mut adjust_last_index = 0;
                    if (idx1, idx2) == last_tuple {
                      match &content_string[idx2..] {
                        "." => if sentence_indices.len() > 1 {
                          adjust_last_index += 1;
                        }
                        _ => adjust_last_index += 1,
                      }
                    }
                    let display_content = format!("[{}]: {}", cont_i, &content_string[idx1..idx2+num_to_add]);
                    let cidx1 = content.chars().count();
                    content.push_str(&display_content);
                    let cidx2 = content.chars().count();
                    if focus_id == cont_i {
                      format_vec.push((String::from("HIGHLIGHTED CONTENT"), cidx1, cidx2+adjust_last_index));
                    } else {
                      format_vec.push((String::from("UNHIGHLIGHTED CONTENT"), cidx1, cidx2+adjust_last_index));
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
                  let last_tuple = sentence_indices[sentence_indices.len()-1].clone();
                  content.push_str(&end_string);
                  for (idx1, idx2) in sentence_indices {
                    let num_to_add = match &end_string[end_string.len()-1..] {
                      "." => 2,
                      _ => {
                        if (idx1, idx2) == last_tuple {
                          2
                        } else {
                          1
                        }
                      },
                    };
                    format_vec.push((String::from("CONTENT"), idx1, idx2+num_to_add));
                  }
                }
              }
            }
            break;
          }
        }
        Some(m) => m,
      };
      current_blank += 1;

      let num_chars = m.end() - m.start();
      let mut replacement = String::new();
      for _ in 0..num_chars {
        replacement.push_str("X");
      }
      
      let b: Blank = Blank::get_blank_from_str(&content_string[m.start()..m.end()]);
      content_string = format!("{}{}{}", &content_string[..m.start()], &replacement, &content_string[m.end()..]);

      let display_blank = match self.blanks.get(&current_blank) {
        Some(b_tup) => {
          match blank_focus_id {
            None => format!("{}", b_tup.1),
            Some(_) => format!("[{}]: {}", i, b_tup.1),
          }
        },
        None => {
          match blank_focus_id {
            None => format!("{}", b.display_to_user()),
            Some(_) => format!("[{}]: {}", i, b.display_to_user()),
          }
        }
      };

      match content_focus_id {
        None => {
          let display_content = String::from(&content_string[prev_end_idx..m.start()]);

          let cidx1 = content.chars().count();
          content.push_str(&display_content);
          let cidx2 = content.chars().count();

          if cidx1 != cidx2 {
            let sentence_indices = NoteTemplate::get_sentence_end_indices(
              cidx1,
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
          // let last_tuple = if sentence_indices.len() > 0 {
          //   sentence_indices[sentence_indices.len()-1]
          // } else {
          //   (0, 0)
          // };
          for (idx1, idx2) in sentence_indices.clone() {
            let num_to_add = if idx1 == idx2 && idx1 == 0 {
              0
            } else {
              1
            };
            let display_content = format!("[{}]: {}", cont_i, &String::from(&content_string[idx1..idx2+num_to_add]));
            let cidx1 = content.chars().count();
            content.push_str(&display_content);
            let cidx2 = content.chars().count();
            if f_id == cont_i {
              format_vec.push((String::from("HIGHLIGHTED CONTENT"), cidx1, cidx2));
            } else {
              format_vec.push((String::from("UNHIGHLIGHTED CONTENT"), cidx1, cidx2));
            }
            // format_vec.push((String::from(&content_string[last_tuple.0..last_tuple.1]), last_tuple.0, last_tuple.1));
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
            match content_focus_id {
              Some(_) => format_vec.push((String::from("UNFOCUSED BLANK"), bidx1, bidx2)),
              None => format_vec.push((String::from("BLANK"), bidx1, bidx2)),
            }
          }
        }
      }

      prev_end_idx = m.end();

      i += 1;
    }
    (content, format_vec)
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
        Some(_b_tup) => (),
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
  pub fn display_content(&self, blank_focus_id: Option<u32>, content_focus_id: Option<u32>) {
    println_on_bg!("{:-^20} | {:-^140}", " Sentence ID ", " Content ");
    println_on_bg!("{:-^163}", "-");
    let (display_content_string, formatting) = self.generate_display_content_string_with_blanks(blank_focus_id, content_focus_id);
    let mut prev_i = 100; // 0 is the actual index
    let display_content_vec = NoteTemplate::get_display_content_vec_from_string(display_content_string, Some(formatting));
    for (i, cont, f) in display_content_vec {
      let num_chars = cont.chars().count();
      let num_to_add = if num_chars < 140 { 140-num_chars-1 } else { 0 };
      // f is Option<Vec<(String, usize, usize)>>
      let display_i = if i == prev_i {
        String::from("   ")
      } else {
        let display_i = i + 1;
        format!(" {} ", display_i)
      };
      prev_i = i;
      match f {
        None => println_on_bg!("{:-^20} | {:-^140}", display_i, &cont),
        Some(f_vec) => {
          print_on_bg!("{:-^20} |  ", display_i);
          if f_vec.len() == 0 {
            print_on_bg!("{: <140}", &cont);
          } else {
            for (s, idx1, idx2) in f_vec {
              let to_format = if idx2 >= cont.len() {
                &cont[idx1..]
              } else {
                &cont[idx1..idx2]
              };
              match &s[..] {
                "HIGHLIGHTED CONTENT" => {
                  print_highlighted_content!("{}", to_format);
                },
                "UNHIGHLIGHTED CONTENT" => {
                  print_unhighlighted_content!("{}", to_format);
                },
                "CONTENT" => {
                  print_on_bg!("{}", to_format);
                },
                "HIGHLIGHTED BLANK" => {
                  print!("{}", Black.on(Yellow).bold().paint(to_format));
                },
                "UNHIGHLIGHTED BLANK" => {
                  print!("{}", Black.on(White).paint(to_format));
                },
                "UNFOCUSED BLANK" => {
                  print_unfocused_blank!("{}", to_format);
                },
                "BLANK" => {
                  print!("{}", Black.on(White).paint(to_format));
                },
                _ => (),
              }
            }
            for _ in 0..num_to_add {
              print_on_bg!(" ");
            }
          }
        }
      }
      print!("\n");
    }
    println_on_bg!("{:-^163}", "-");
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
      let mut b_string = blanks_tup.1.replace("/#/", "#");
      b_string = b_string.replace("/%/", "%");
      let blanks_string = format!(
        "{}/%/{}/%/{}/%/{}",
        order,
        blanks_tup.0,
        &b_string,
        blanks_tup.2.iter().map(|id| id.to_string() ).collect::<Vec<String>>().join("-")
      );
      formatted_blanks.push(blanks_string);
    }
    let blanks_str: String = formatted_blanks.join("/#/");

    lazy_static! {
      static ref RE_BLANK: Regex = Regex::new("[(]---[a-zA-Z0-9_]*@?[0-9]*@?---[)]").unwrap();
      static ref RE_PIPE: Regex = Regex::new(r#" \| "#).unwrap();
      static ref RE_L: Regex = Regex::new(r#"\(---"#).unwrap();
      static ref RE_R: Regex = Regex::new(r#"---\)"#).unwrap();
    }
    let find_match_string = self.content.clone();
    let matches = RE_BLANK.find_iter(&find_match_string);
    
    let mut changed_content = self.content.clone();
    for m in matches {
      let mut replacement = String::new();
      for _i in m.start()..m.end() {
        replacement.push_str("X");
      }
      changed_content = format!(
        "{}{}{}",
        &changed_content[..m.start()],
        &replacement,
        &changed_content[m.end()..]
      );
    }
    let match_pipe = RE_PIPE.find_iter(&changed_content);
    let match_l = RE_L.find_iter(&changed_content);
    let match_r = RE_R.find_iter(&changed_content);
    
    let mut content = self.content.clone();
    for m in match_pipe {
      content = format!(
        "{}{}{}",
        &content[..m.start()],
        " / ",
        &content[m.end()..]
      );
    }
    for m in match_l {
      content = format!(
        "{}{}{}",
        &content[..m.start()],
        "(-- ",
        &content[m.end()..]
      );
    }
    for m in match_r {
      content = format!(
        "{}{}{}",
        &content[..m.start()],
        " --)",
        &content[m.end()..]
      );
    }
    write!(
      f,
      "{} | {}-{}-{} | {} | {} | {} | {} | {} | {} | {}\n",
      &self.id,
      &self.date.year(),
      &self.date.month(),
      &self.date.day(),
      &self.category,
      &self.structure,
      &content,
      blanks_str,
      &self.foreign_key["user_id"],
      &self.foreign_key["client_id"],
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
  PartnerICCOrFP,
  CurrentClientName,
  Collaterals,
  AllCollaterals,
  PrimaryContact,
  Guardian,
  CarePlanTeam,
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
}

use Blank::{
  CurrentUser,
  PartnerICCOrFP,
  CurrentClientName,
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

impl Blank {
  pub fn has_pronouns(&self) -> bool {
    match self {
      CurrentUser | PartnerICCOrFP | CurrentClientName | Collaterals | AllCollaterals | PrimaryContact | Guardian | CarePlanTeam => true,
      _ => false,
    }
  }
  pub fn iterator() -> impl Iterator<Item = Blank> {
    [
      CurrentUser,
      PartnerICCOrFP,
      CurrentClientName,
      Collaterals,
      AllCollaterals,
      PrimaryContact,
      Guardian,
      CarePlanTeam,
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
    ].iter().copied()
  }
  pub fn vec_of_fillables() -> Vec<Blank> {
    vec![
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
      CustomBlank,
    ]
  }
  pub fn vector_of_variants() -> Vec<Blank> {
    vec![
      CurrentUser,
      PartnerICCOrFP,
      CurrentClientName,
      Collaterals,
      AllCollaterals,
      PrimaryContact,
      Guardian,
      CarePlanTeam,
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
    ]
  }
  pub fn abbreviate(&self) -> String {
    match self {
      CurrentUser => String::from("u"),
      PartnerICCOrFP => String::from("p"),
      CurrentClientName => String::from("c"),
      Collaterals => String::from("co"),
      AllCollaterals => String::from("allco"),
      PrimaryContact => String::from("pc"),
      Guardian => String::from("g"),
      CarePlanTeam => String::from("cpt"),
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
      Appearance => String::from("ap"),
      SupportedParent => String::from("sp"),
      ParentingSkill => String::from("ps"),
      CarePlanningTopic => String::from("cpto"),
      YouthTopic => String::from("yt"),
      ContactMethod => String::from("cm"),
      ContactPurpose => String::from("cp"),
      FulfilledContactPurpose => String::from("fcp"),
      Service => String::from("s"),
      MeetingMethod => String::from("mm"),
      SignatureMethod => String::from("sm"),
      CustomBlank => String::from("cu"),
    }
  }
  pub fn get_blank_from_str(s: &str) -> Blank {
    match &s[..] {
      "(---u---)" => CurrentUser,
      "(---p---)" => PartnerICCOrFP,
      "(---c---)" => CurrentClientName,
      "(---co---)" => Collaterals,
      "(---allco---)" => AllCollaterals,
      "(---pc---)" => PrimaryContact,
      "(---g---)" => Guardian,
      "(---cpt---)" => CarePlanTeam,
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
      "(---ap---)" => Appearance,
      "(---ap---)" => SupportedParent,
      "(---ps---)" => ParentingSkill,
      "(---cpto---)" => CarePlanningTopic,
      "(---yt---)" => YouthTopic,
      "(---cm---)" => ContactMethod,
      "(---cp---)" => ContactPurpose,
      "(---fcp---)" => FulfilledContactPurpose,
      "(---s---)" => Service,
      "(---mm---)" => MeetingMethod,
      "(---sm---)" => SignatureMethod,
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
      CurrentUser => String::from("[ Current user ]"),
      PartnerICCOrFP => String::from("[ Partner ICC or FP ]"),
      CurrentClientName => String::from("[ Name of client ]"),
      Collaterals => String::from("[ One or more collaterals ]"),
      AllCollaterals => String::from("[ All collaterals for the current client ]"),
      PrimaryContact => String::from("[ Current client's primary contact ]"),
      Guardian => String::from("[ Current client's guardian ]"),
      CarePlanTeam => String::from("[ Current client's Care Plan Team ]"),
      Pronoun1ForBlank(b_id_option) => {
        let b_id = b_id_option.unwrap();
        format!("[ Subject pronouns of the person in blank #{} (he, she, they) ]", b_id)
      },
      Pronoun2ForBlank(b_id_option) => {
        let b_id = b_id_option.unwrap();
        format!("[ Object pronouns of the person in blank #{} (him, her, them) ]", b_id)
      },
      Pronoun3ForBlank(b_id_option) => {
        let b_id = b_id_option.unwrap();
        format!("[ Possessive determiner pronouns of the person in blank #{} (his, her, their) ]", b_id)
      },
      Pronoun4ForBlank(b_id_option) => {
        let b_id = b_id_option.unwrap();
        format!("[ Possessive pronouns of the person in blank #{} (his, hers, theirs) ]", b_id)
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
      Appearance => String::from("[ Appearance/affect ]"),
      SupportedParent => String::from("[ Supported... ]"),
      ParentingSkill => String::from("[ Parenting skills ]"),
      CarePlanningTopic => String::from("[ Care Planning topic ]"),
      YouthTopic => String::from("[ Youth mental health topic ]"),
      ContactMethod => String::from("[ Contact method ]"),
      ContactPurpose => String::from("[ Contact purpose ]"),
      FulfilledContactPurpose => String::from("[ Contact purpose (past tense) ]"),
      Service => String::from("[ Service ]"),
      MeetingMethod => String::from("[ Meeting method ]"),
      SignatureMethod => String::from("[ Signature method ]"),
      CustomBlank => String::from("[ Custom input ]"),
    }
  }
  pub fn display_to_user_empty(&self) -> String {
    match self {
      CurrentUser => String::from("Current user"),
      PartnerICCOrFP => String::from("Partner ICC or FP"),
      CurrentClientName => String::from("Name of client"),
      Collaterals => String::from("One or more collaterals"),
      AllCollaterals => String::from("All collaterals for the current client"),
      PrimaryContact => String::from("Current client's primary contact"),
      Guardian => String::from("Current client's guardian"),
      CarePlanTeam => String::from("Current client's Care Plan Team"),
      Pronoun1ForBlank(_) => {
        format!("Subject pronoun of the person in another blank (he, she, they)")
      },
      Pronoun2ForBlank(_) => {
        format!("Object pronoun of the person in another blank (him, her, them)")
      },
      Pronoun3ForBlank(_) => {
        format!("Possessive determiner of the person in another blank (his, her, their)")
      },
      Pronoun4ForBlank(_) => {
        format!("Possessive pronoun of the person in another blank (his, hers, theirs)")
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
      Appearance => String::from("Appearance/affect"),
      SupportedParent => String::from("Supported..."),
      ParentingSkill => String::from("Parenting skills"),
      CarePlanningTopic => String::from("Care Planning topic"),
      YouthTopic => String::from("Youth mental health topic"),
      ContactMethod => String::from("Contact method"),
      ContactPurpose => String::from("Contact purpose"),
      FulfilledContactPurpose => String::from("Contact purpose (past tense)"),
      Service => String::from("Service"),
      MeetingMethod => String::from("Meeting method"),
      SignatureMethod => String::from("Signature method"),
      CustomBlank => String::from("Custom input"),
    }
  }
}

impl fmt::Display for Blank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(---{}---)", self.abbreviate())
  }
}