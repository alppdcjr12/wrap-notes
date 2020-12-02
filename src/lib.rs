#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use] extern crate lazy_static;

pub mod user;
pub use user::*;
pub use crate::EmployeeRole::{FP, ICC};
pub use crate::SupportType::{Natural, Formal};
pub use crate::StructureType::{
  CarePlan,
  CarePlanVerbose,
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
};

pub mod note_archive;
pub use note_archive::*;

pub mod client;
pub use client::*;

pub mod collateral;
pub use collateral::*;

pub mod pronouns;
pub use pronouns::*;

pub mod note_day;
pub use note_day::*;

pub mod note;
pub use note::*;

pub mod utils;
pub use utils::*;

pub mod constants;
pub use constants::*;

pub mod blank_enums;
pub use blank_enums::*;