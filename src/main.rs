// use std::fs::File;
// use std::io::prelude::*;
use wrap_notes::note_archive::*;

pub const USR_FL: &str = "users.txt";
pub const CLT_FL: &str = "clients.txt";
pub const COL_FL: &str = "collaterals.txt";
pub const PRN_FL: &str = "pronouns.txt";
pub const ND_FL: &str = "note_days.txt";
pub const NT_FL: &str = "note_templates.txt";

fn main() {
  let filepaths = [
    (String::from("user_filepath"), String::from(USR_FL),),
    (String::from("client_filepath"), String::from(CLT_FL),),
    (String::from("collateral_filepath"), String::from(COL_FL),),
    (String::from("pronouns_filepath"), String::from(PRN_FL),),
    (String::from("note_day_filepath"), String::from(ND_FL),),
    (String::from("note_template_filepath"), String::from(NT_FL),),
  ].iter().cloned().collect();
  let mut a = NoteArchive::new(filepaths);
  a.run();
}
