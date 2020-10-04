#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use chrono::{NaiveDate, Datelike, TimeZone, Utc, Local};

use wrap_notes::*;

// User-related tests

#[test]
fn can_write_users() {
  let u1 = User::new(1, String::from("Pete"), String::from("Smith"), ICC, vec![]);
  let u2 = User::new(2, String::from("Johana"), String::from("Smith"), FP, vec![]);

  let client_string_1 = u1
    .clients
    .iter()
    .map(|i| i.to_string())
    .collect::<Vec<String>>()
    .join("#");

  let client_string_2 = u2
    .clients
    .iter()
    .map(|i| i.to_string())
    .collect::<Vec<String>>()
    .join("#");

  NoteArchive::new()
    .write_users(vec![u1, u2], "test_write_users.txt")
    .unwrap();
  let file = File::open("test_write_users.txt").unwrap();
  let reader = BufReader::new(file);

  let lines: Vec<String> = reader.lines().map(|item| item.unwrap()).collect::<Vec<_>>();

  // remove unneeded file
  fs::remove_file("test_write_users.txt").unwrap();

  assert_eq!(
    lines,
    vec![
      String::from("##### users #####"),
      format!("{} | {} | {} | {} | {}", 1, "Pete", "Smith", "ICC", client_string_1),
      format!("{} | {} | {} | {} | {}", 2, "Johana", "Smith", "FP", client_string_2),
      String::from("##### users #####"),
    ]
  );
}

#[test]
fn can_read_users() {
  let mut lines = String::from("##### users #####\n");
  lines.push_str("1 | Pete | Peteson | ICC | 1#2#3#4\n");
  lines.push_str("2 | Vivian | Vivianson | FP | 5#6#7#8\n");
  lines.push_str("3 | Sabrina | Sabrinason | FP | 9#10#11#12\n");
  lines.push_str("4 | Dave | Davidson | ICC | 13#24#35#46\n");
  lines.push_str("##### users #####");

  let mut file = File::create("test_read_users.txt").unwrap();
  file.write_all(lines.as_bytes()).unwrap();

  assert_eq!(
    NoteArchive::read_users("test_read_users.txt"),
    vec![
      User::new(1, String::from("Pete"), String::from("Peteson"), ICC, vec![1, 2, 3, 4],),
      User::new(2, String::from("Vivian"), String::from("Vivianson"), FP, vec![5, 6, 7, 8],),
      User::new(3, String::from("Sabrina"), String::from("Sabrinason"), FP, vec![9, 10, 11, 12],),
      User::new(4, String::from("Dave"), String::from("Davidson"), ICC, vec![13, 24, 35, 46],),
    ]
  );
  // remove unneeded file
  fs::remove_file("test_read_users.txt").unwrap();
}

#[test]
fn creates_unique_new_user() {
  let user_1 = User::new(1, String::from("Pete"), String::from("Peteson"), ICC, vec![1, 2, 3, 4]);
  let user_2 = User::new(2, String::from("Sandy"), String::from("Sandyson"), FP, vec![5, 6, 7, 8]);
  let users = vec![user_1, user_2];
  let mut notes = NoteArchive::new();
  notes.write_users(users, "test_unique_users.txt").unwrap();

  let new_user_attempt = notes.generate_unique_new_user(
    String::from("Carl"),
    String::from("Carlson"),
    ICC,
    "test_unique_users.txt",
  );

  let new_user = match new_user_attempt {
    Ok(user) => user,
    Err(_) => panic!("Failed to generate user."),
  };

  assert_eq!(new_user, User::new(3, String::from("Carl"), String::from("Carlson"), ICC, vec![]));

  fs::remove_file("test_unique_users.txt").unwrap();
}

#[test]
fn saves_user_to_file() {
  {
    let mut notes = NoteArchive::new();
    let user_1 = User::new(1, String::from("Pete"), String::from("Peteson"), ICC, vec![1, 2, 3, 4],);
    let user_2 = User::new(2, String::from("Sandy"), String::from("Sandyson"), FP, vec![5, 6, 7, 8],);
    let user_3 = User::new(3, String::from("Carl"), String::from("Carlson"), ICC, vec![]);
    notes.save_user(user_1, "test_save_user.txt");
    notes.save_user(user_2, "test_save_user.txt");
    notes.save_user(user_3, "test_save_user.txt");

    assert_eq!(
      NoteArchive::read_users("test_save_user.txt"),
      vec![
        User::new(1, String::from("Pete"), String::from("Peteson"), ICC, vec![1, 2, 3, 4],),
        User::new(2, String::from("Sandy"), String::from("Sandyson"), FP, vec![5, 6, 7, 8],),
        User::new(3, String::from("Carl"), String::from("Carlson"), ICC, vec![])
      ]
    );
  }
  fs::remove_file("test_save_user.txt").unwrap();
}

// Client-related tests

#[test]
fn can_write_clients() {
  let c1 = Client::new(1, String::from("John"), String::from("Doe"), NaiveDate::from_ymd(2015, 01, 14), vec![]);
  let c2 = Client::new(2, String::from("Jane"), String::from("Doe"), NaiveDate::from_ymd(2011, 05, 3), vec![]);

  let collaterals_string_1 = c1.collaterals.iter()
    .map(|i| i.to_string()).collect::<Vec<String>>()
    .join("#");

  let collaterals_string_2 = c2.collaterals.iter()
    .map(|i| i.to_string()).collect::<Vec<String>>()
    .join("#");

  NoteArchive::new()
    .write_clients(vec![c1, c2], "test_write_clients.txt")
    .unwrap();
  let file = File::open("test_write_clients.txt").unwrap();
  let reader = BufReader::new(file);

  let lines: Vec<String> = reader.lines().map(|item| item.unwrap()).collect::<Vec<_>>();

  // remove unneeded file
  fs::remove_file("test_write_clients.txt").unwrap();

  assert_eq!(
    lines,
    vec![
      String::from("##### clients #####"),
      format!("{} | {} | {} | {} | {}", 1, "John", "Doe", "2015-1-14", collaterals_string_1),
      format!("{} | {} | {} | {} | {}", 2, "Jane", "Doe", "2011-5-3", collaterals_string_2),
      String::from("##### clients #####"),
    ]
  );
}

#[test]
fn can_read_clients() {
  let mut lines = String::from("##### clients #####\n");
  lines.push_str("1 | John | Doe | 2005-3-14 | 1#2#3#4\n");
  lines.push_str("2 | Jane | Doe | 2006-3-14 | 5#6#7#8\n");
  lines.push_str("3 | Joe | Biden | 2009-6-21 | 9#10#11#12\n");
  lines.push_str("4 | Donald | Trump | 2010-7-1 | 13#24#35#46\n");
  lines.push_str("##### clients #####");

  let mut file = File::create("test_read_clients.txt").unwrap();
  file.write_all(lines.as_bytes()).unwrap();

  assert_eq!(
    NoteArchive::read_clients("test_read_clients.txt"),
    vec![
      Client::new(1, String::from("John"), String::from("Doe"), NaiveDate::from_ymd(2005, 3, 14), vec![1, 2, 3, 4]),
      Client::new(2, String::from("Jane"), String::from("Doe"), NaiveDate::from_ymd(2006, 3, 14), vec![5, 6, 7, 8]),
      Client::new(3, String::from("Joe"), String::from("Biden"), NaiveDate::from_ymd(2009, 6, 21), vec![9, 10, 11, 12]),
      Client::new(4, String::from("Donald"), String::from("Trump"), NaiveDate::from_ymd(2010, 7, 1), vec![13, 24, 35, 46]),
    ]
  );
  // remove unneeded file
  fs::remove_file("test_read_clients.txt").unwrap();
}

#[test]
fn creates_unique_new_client() {
  let client_1 = Client::new(1, String::from("Pete"), String::from("McLastName"), NaiveDate::from_ymd(2006, 1, 2), vec![1, 2, 3, 4]);
  let client_2 = Client::new(2, String::from("Sandy"), String::from("O'Lastnymn"), NaiveDate::from_ymd(2007, 2, 3), vec![5, 6, 7, 8]);
  let clients = vec![client_1, client_2];
  let mut notes = NoteArchive::new();
  notes.write_clients(clients, "test_unique_clients.txt").unwrap();

  let new_client_attempt = notes.generate_unique_new_client(
    String::from("Carl"),
    String::from("Carlson"),
    NaiveDate::from_ymd(2008, 3, 4),
    "test_unique_clients.txt",
  );

  let new_client = match new_client_attempt {
    Ok(user) => user,
    Err(_) => panic!("Failed to generate client."),
  };

  assert_eq!(new_client, Client::new(3, String::from("Carl"), String::from("Carlson"), NaiveDate::from_ymd(2008, 3, 4), vec![]));

  fs::remove_file("test_unique_clients.txt").unwrap();
}

#[test]
fn saves_client_to_file() {
  {
    let mut notes = NoteArchive::new();
    let client_1 = Client::new(1, String::from("John"), String::from("Doe"), NaiveDate::from_ymd(2005, 3, 14), vec![1, 2, 3, 4],);
    let client_2 = Client::new(2, String::from("Jane"), String::from("Doe"), NaiveDate::from_ymd(2006, 3, 14), vec![5, 6, 7, 8],);
    let client_3 = Client::new(3, String::from("Joe"), String::from("Biden"), NaiveDate::from_ymd(2009, 6, 21), vec![9, 10, 11, 12]);
    let client_4 = Client::new(4, String::from("Donald"), String::from("Trump"), NaiveDate::from_ymd(2010, 7, 1), vec![13, 24, 35, 46]);
    notes.save_client(client_1, "test_save_client.txt");
    notes.save_client(client_2, "test_save_client.txt");
    notes.save_client(client_3, "test_save_client.txt");
    notes.save_client(client_4, "test_save_client.txt");
    
    assert_eq!(
      NoteArchive::read_clients("test_save_client.txt"),
      vec![
        Client::new(1, String::from("John"), String::from("Doe"), NaiveDate::from_ymd(2005, 3, 14), vec![1, 2, 3, 4],),
        Client::new(2, String::from("Jane"), String::from("Doe"), NaiveDate::from_ymd(2006, 3, 14), vec![5, 6, 7, 8],),
        Client::new(3, String::from("Joe"), String::from("Biden"), NaiveDate::from_ymd(2009, 6, 21), vec![9, 10, 11, 12]),
        Client::new(4, String::from("Donald"), String::from("Trump"), NaiveDate::from_ymd(2010, 7, 1), vec![13, 24, 35, 46]),
      ]
    );
  }
  fs::remove_file("test_save_client.txt").unwrap();
}
