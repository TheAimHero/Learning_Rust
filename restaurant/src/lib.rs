#![allow(dead_code)]

mod back_of_house;
pub use crate::back_of_house::{cleaning, cooking};

mod front_of_house;
pub use crate::front_of_house::{hosting, serving};

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  serving::take_order();
  cleaning::clean_kitchen();
}
