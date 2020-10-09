// use std::fs::File;
// use std::io::prelude::*;
use wrap_notes::note_archive::*;

pub const USR_FL: &str = "users.txt";
pub const CLT_FL: &str = "clients.txt";
pub const PRN_FL: &str = "pronouns.txt";

fn main() {
  let mut a = NoteArchive::new(
    String::from(USR_FL),
    String::from(CLT_FL),
    String::from(PRN_FL),
  );
  a.run();
}
