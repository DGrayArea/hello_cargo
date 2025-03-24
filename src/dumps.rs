use std::io;
use std::cmp::Ordering;
use std::fmt::Display;
use rand::Rng;
use std::collections::HashMap;

mod back_of_house {
  pub struct Breakfast {
      pub toast: String,
      seasonal_fruit: String,
  }

  impl Breakfast {
      pub fn summer(toast: &str) -> Breakfast {
          Breakfast {
              toast: String::from(toast),
              seasonal_fruit: String::from("peaches"),
          }
      }
  }
}

pub fn eat_at_restaurant() {
  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");
}

fn main () {


}

pub fn add(left: u64, right: u64) -> u64 {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      let result = add(2, 2);
      assert_eq!(result, 4);
  }

  #[test]
  fn another() {
      panic!("Make this test fail");
  }
}


fn garbage() {
  eat_at_restaurant();

  let mut v = Vec::new();
  v.push(3);

  let vecce = vec![1, 2, 3, 4, 5];
  println!("The value of vecce is: {}", vecce.get(2).unwrap());

  let number :i32 = 98;
  let result = add_on(number);

  if result < 90 {
    println!("This is the result: {result}");
  } else {
    println!("This is not the result")
  }

  let x = [1,2,3,4,5,7];

  let mut index = 0;

  // while index < 5 {
  //   println!("the current number is: {}", x[index]);
  //   index+=1;
  // }

  for element in x {
    println!("The element is {element}")
  }

  for number in (1..10).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");

let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

let user = generate_user(String::from("John Doe"),String::from("rustup@gmail.com"));

// user.email = String::from("rutlanf@yahoo.com");

let rect1 = Rectangle{
  width:32,
  height:32,
};

let result = area(&rect1);

// println!("rect1 is {rect1:#?}");
dbg!(&rect1);

println!(
  "The area of the rectangle is {} square pixels.",  rect1.area()
);

  let rect1 = Rectangle {
    width: 30,
    height: 50,
};
let rect2 = Rectangle {
    width: 10,
    height: 40,
};
let rect3 = Rectangle {
    width: 60,
    height: 45,
};

let sqr = Rectangle::square(30);
dbg!(&sqr);
println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

let address = IpAddress {
  kind:IpAddressKind::V4(String::from("234.452.46")),
  address:String::from("192.168.0.0")
};

let some_num = Some(5);
let some_char = Some('e');

// let absent_num: Option<i32> = None;

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2} {s1}");

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

println!("The score is {scores:?}");

for (key, value) in &scores {
  println!("{key}: {value}");
}
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}

fn generate_user(email: String, username: String) -> User {
  User{
    username,
    email,
    sign_in_count:1,
    active:true,
  }
}
fn add_on(y:i32) -> i32 {
    y+1
}

// fn dangle () -> &String {
//   let s: String = String::from("Hello Dangling!!!");

//   &s
// }

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height:u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other:&Rectangle) -> bool {
      self.width > other.width && self.height > other.height
  }

  fn square(size:u32) -> Self {
    Self {
      width: size,
      height:size
    }
  }
}

enum IpAddressKind {
  V4(String),
  V6
}

enum Option<T> {
  None,
  Some(T)
}
struct IpAddress {
  kind: IpAddressKind,
  address: String
}

fn area (dimensions: &Rectangle) -> u32 {
  dimensions.width * dimensions.height
}

fn guessing_game() {
    println!("Guess the number!!.");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

   loop { 
    
    println!("Input your guess:");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Guess too small ..."),
        Ordering::Equal => {
            println!("You guessed correct ...");
            break;
        },
        Ordering::Greater => println!("Guess to big ...")
    }
  }

}