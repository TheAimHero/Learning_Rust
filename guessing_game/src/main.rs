#![allow(unused_variables)]
#![allow(dead_code)]
use rand::Rng;
use std::{cmp::Ordering, io, print, println};

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
    //using a except or using a match (match can run diffrent code depending
    //on the result) that compares the the individual values
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

// fibonacci series
fn _fibonacci(n: u32) -> u32 {
  if n <= 2 {
    return n - 1;
  }
  return _fibonacci(n - 1) + _fibonacci(n - 2);
}

//Trying stuff
#[derive(Debug)]
struct _Person {
  name: String,
  age: u8,
  email: String,
}

impl _Person {
  fn update(&mut self) {
    self.name = String::from("Not Vedant");
  }
}

#[derive(Debug)]
enum _Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl _Message {
  fn call(&self) {
    println!("{:?}", self);
  }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
  Alabama,
  Alaska,
}

#[derive(Debug)]
enum Coin {
  Penny(UsState, i32),
  Nickel(UsState, i32),
  Dime(UsState, i32),
  Quarter(UsState, i32),
}

fn _value_in_cents(coin: Coin) -> (UsState, i32) {
  match coin {
    Coin::Penny(state, ammount) => {
      println!("{:?} {:?}", state, ammount);
      (state, ammount)
    }
    Coin::Nickel(state, ammount) => {
      println!("{:?} {:?}", state, 5 * ammount);
      (state, 5 * ammount)
    }
    Coin::Dime(state, ammount) => {
      println!("{:?} {:?}", state, 10 * ammount);
      (state, 10 * ammount)
    }
    Coin::Quarter(state, ammount) => {
      println!("{:?} {:?}", state, 25 * ammount);
      (state, 25 * ammount)
    }
  }
}

fn main() {
  let th: Coin = Coin::Penny(UsState::Alabama, 5);
  println!("{:?}", th);
}
