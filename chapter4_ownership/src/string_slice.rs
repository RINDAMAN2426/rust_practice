fn main() {
  let s = String::from("hello world");

  let hello = &s[0..5];   // String에 대한 참조 뿐 아니라 지정된 부분에 대한 참조만을 얻게 된다.
  let world = &s[6..11];

  println!("{} {}", hello, world);
}