fn main(){
  let some_u8_value = 0u8;
  match some_u8_value {
      1 => println!("one"),
      3 => println!("three"),
      _ => println!("not one or three"),
      // other => println!("not one or three, value is {}", other),
  }
}