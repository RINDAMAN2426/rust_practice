fn main() {
  let mut s = String::from("hello");

  // Error
  // let r1 = &mut s;
  // let r2 = &mut s;

  change(&mut s);

  println!("{}", s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}