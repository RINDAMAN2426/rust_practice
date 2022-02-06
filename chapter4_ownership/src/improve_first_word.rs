fn main() {
  let my_string = String::from("hello world");

  let word = first_word(&my_string[..]);

  let my_string_literal = "hello world";

  let word = first_word(&my_string_literal[..]);

  let word = first_word(my_string_literal);
}

// String -> &str으로 변경함으로써 같은 기능이지만
// 더 보편적이고 유용한 API를 디자인할 수 있다.
// literal은 그대로 전달하면 되고, String은 전체 슬라이스를 전달하면 된다.
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}