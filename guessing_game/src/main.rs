#![allow(unused_variables)]
#![allow(dead_code)]
use rand::Rng;
use std::cmp::Ordering;
use std::{io, print, println};

// The main guessing game function
fn _guessing_game() {
  println!("Guess a number! \nEnter a number");
  let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
  println!("Your secrent number is :{secret_number}");

  loop {
    let mut guess: String = String::new();
    println!("Enter a number:");
    io::stdin()
      .read_line(&mut guess)
      // .expect("Failed to read line");
      .expect("Failed to read line");
    print!("You guessed: {guess}");

    //When a function returns a result (a type of enum) it can be handled
    //using a except or using a match that compares the the individual values
    //inside the enum and proceeds with the execution accordingly
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        continue;
      }
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}

// Splits the given string by spaces
fn _split_string(s: &mut String) -> Vec<&str> {
  let bytes = s.trim().as_bytes();
  let mut vector: Vec<&str> = Vec::new();
  let mut prev: usize = 0;
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      vector.push(&s[prev..(i + 1)]);
      prev = i + 1
    }
  }
  vector.push(&s[prev..]);
  if vector.len() == 0 {
    return vector;
  }
  return vector;
}

//Trying stuff
#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
  email: String,
}

impl Person {
  fn update(&mut self) {
    self.name = String::from("Not Vedant");
  }

  fn display_struct(&self) {
    println!("Name: {}", self.name);
    println!("Age: {}", self.age);
    println!("Email: {}", self.email);
    println!()
  }
}

fn main() {
  let mut p = Person {
    name: String::from("Vedant"),
    age: 20,
    email: String::from("a@b.com"),
  };
  let mut vector: Vec<i32> = Vec::new();
  vector.push(9);
  println!("{:#?}", &p);
  p.display_struct();
  p.update();
  p.display_struct();
}
