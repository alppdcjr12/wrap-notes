use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::{BufRead, BufReader};
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockCipher, NewBlockCipher};
use aes::Aes256;

pub fn make_ascii_titlecase(s: &mut str) {
  if let Some(r) = s.get_mut(0..1) {
    r.make_ascii_uppercase();
  }
}

pub fn decrypt_file(data_fp: &str, pw: &str) -> Result<(), Error> {

  let mut key = password_to_bytes(pw.to_string());

  let cipher_key = GenericArray::from_mut_slice(&mut key);
  let cipher = Aes256::new(&cipher_key);

  let mut file = File::open(data_fp)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  let mut all_blocks = vec![];

  while buffer.len() > 0 {
    let mut block = vec![];
    for byte in &buffer[0..16] {
      block.push(*byte);
    }
    buffer.drain(0..16);
    let mut cipher_block = GenericArray::from_mut_slice(&mut block);
    cipher.decrypt_block(&mut cipher_block);

    let mut decrypted_block: Vec<u8> = vec![];

    if buffer.is_empty() {
      if cipher_block.iter().any(|byte| byte > &0 && byte < &16) {
        let mut idx = 0;
        for byte in cipher_block.clone() {
          if byte > 0 && byte < 16 {
            if cipher_block[idx+1..].to_vec().iter().all(|b| *b == 0) {
              decrypted_block.append(&mut cipher_block[..idx].to_vec());
            }
          }
          idx += 1;
        }
      }
    } else {
      decrypted_block.append(&mut cipher_block.to_vec());
    }

    all_blocks.append(&mut decrypted_block.to_vec());
  }

  let mut file = File::create(data_fp)?;

  file.write_all(&all_blocks)?;

  Ok(())

}

pub fn encrypt_file(data_fp: &str, pw: &str) -> Result<(), Error> {

  let mut key = password_to_bytes(pw.to_string());

  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(data_fp).unwrap();

  let reader = BufReader::new(file);
  let lines: Vec<std::io::Result<String>> = reader.lines().collect();
  let mut data = lines.iter().filter(|l| l.is_ok() ).map(|l| l.as_ref().unwrap().clone() ).collect::<Vec<String>>().join("\n").as_bytes().to_vec();

  let cipher_key = GenericArray::from_mut_slice(&mut key);
  let cipher = Aes256::new(&cipher_key);

  let mut all_blocks = vec![];
  
  while data.len() > 0 {
    let mut block = vec![];
    if data.len() > 15 {
      for byte in &data[0..16] {
        block.push(*byte);
      }
      data.drain(0..16);
    } else {
      for byte in &data[0..] {
        block.push(*byte);
      }
      let num_zeros = 15-data.len();
      block.push(num_zeros as u8);
      for _n in 0..num_zeros {
        block.push(0 as u8);
      }
      data.clear();
    }
    let mut cipher_block = GenericArray::from_mut_slice(&mut block);
    cipher.encrypt_block(&mut cipher_block);
    all_blocks.append(&mut cipher_block.to_vec());
  }

  let mut file = File::create(data_fp)?;

  file.write_all(&all_blocks)?;

  Ok(())
}

fn password_to_bytes(password: String) -> Vec<u8> {
  let mut password_bytes = password.as_bytes().to_vec();
    
  while password_bytes.len() < 32 {
    let mut new_bytes = password_bytes.clone();
    password_bytes.append(&mut new_bytes);
  }
  
  let mut xord_bytes: Vec<u8> = vec![0; 32];
  
  while password_bytes.len() > 0 {
    for (i, num) in xord_bytes.clone().iter().enumerate() {
      if password_bytes.len() > 0 {
        xord_bytes[i] = num ^ password_bytes.pop().unwrap();
      }        
    }        
  }

  xord_bytes

}