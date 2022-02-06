fn main() {
  let s = String::from("hello");

  let len = s.len();
  // 아래 둘은 동일
  let slice = &s[0..2];
  let slice = &s[..2];

  // 아래 둘도 동일
  let slice = &s[3..len];
  let slice = &s[3..];

  // 아래 둘도 동일
  let slice = &[0..len];
  let slice = &[..];
}