use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};


struct User {
  id: u32,
  name: String,
  clients: Vec<String>,
}

impl User {
  fn new(id: u32, name: String) -> User {
    User {
      id,
      name,
      clients: vec![],
    }
  }
  fn add_client(&mut self, client: String) {
    self.clients.push(client);
  } 
  fn write_users(users: Vec<&User>, filepath: String) {
    let mut lines = String::from("##### users #####\n");
    for u in users {
      lines.push_str(&format!("{} | {} | {:?}\n",
        &u.id,
        &u.name[..],
        &u.clients[..]
      ));
    }
    lines.push_str("##### users #####");
    let mut file = File::create(filepath).unwrap();
    file.write_all(lines.as_bytes()).unwrap();
  }
  // fn load_users() -> std::io::Result<()> {

  // }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn new_users() {
    let u1 = User::new(
      1,
      String::from("Carol"),
    );
    let u2 = User::new(
      2,
      String::from("Kerri"),
    );
    let test_vec: Vec<String> = vec![]; 
    assert_eq!(u1.id, 1);
    assert_eq!(u1.name, String::from("Carol"));
    assert_eq!(u1.clients, test_vec);
    assert_eq!(u2.id, 2);
    assert_eq!(u2.name, String::from("Kerri"));
    assert_eq!(u2.clients, test_vec);
  }
  #[test]
  fn adding_clients() {
    let mut u1 = User::new(
      1,
      String::from("Carol"),
    );
    u1.add_client(String::from("Client1"));
    u1.add_client(String::from("Client2"));
    u1.add_client(String::from("Client3"));
    assert_eq!(u1.clients, [
      String::from("Client1"),
      String::from("Client2"),
      String::from("Client3"),
    ]);
  }
  #[test]
  fn can_write_users() {
    let u1 = User::new(
      1,
      String::from("Pete"),
    );
    let u2 = User::new(
      2,
      String::from("Johana"),
    );
    User::write_users(vec![&u1, &u2], String::from("test_users.txt"));
    let file = File::open("test_users.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
      .map(|item| item.unwrap())
      .collect::<Vec<_>>();

    // remove unneeded file
    // fs::remove_file("test_users.txt").unwrap();

    assert_eq!(
      lines,
      vec![
        String::from("##### users #####"),
        format!("{} | {} | {:?}", 1, "Pete", u1.clients),
        format!("{} | {} | {:?}", 2, "Johana", u1.clients),
        String::from("##### users #####"),
      ]
    );

  }
}