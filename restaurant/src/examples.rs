mod front_of_house { // mod keyword 를 이용하여 module의 이름을 지정 및 정의
  pub mod hosting { // module내 다른 module을 정의할 수 있음
      pub fn add_to_waitlist() {} // module에는 sturct, enums, constants, traits, functions 를 추가 가능

      fn seat_at_table() {}
  }

  mod serving {
      fn take_order() {}

      fn serve_order() {}

      fn take_payment() {}
  }
}

pub fn eat_at_restaurant_with_path () {
  // absolute path
  crate::front_of_house::hosting::add_to_waitlist();
  // relative path
  front_of_house::hosting::add_to_waitlist();
}

pub fn eat_at_restaurant_with_struct() {
  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant_with_enum() {
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

//pub use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;

pub fn eat_at_restaurant_with_use() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}



fn serve_order () {}

mod back_of_house {
  pub enum Appetizer {
      Soup,
      Salad,
  }

  pub struct Breakfast {
      pub toast: String,
      seaonal_fruit: String,
  }

  impl Breakfast {
      pub fn summer(toast: &str) -> Breakfast {
          Breakfast {
              toast: String::from(toast),
              seaonal_fruit: String::from("peaches")
          }
      }
  }

  fn cook_order() {}

  fn fix_incorrect_order() {
      cook_order();
      super::serve_order();
  }
}