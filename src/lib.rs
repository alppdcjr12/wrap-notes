#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod user;
pub use user::*;
pub use crate::EmployeeRole::{FP, ICC};

pub mod note_archive;
pub use note_archive::*;

pub mod client;
pub use client::*;

pub mod pronouns;
pub use pronouns::*;