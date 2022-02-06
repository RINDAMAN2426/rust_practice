fn main() {
  let s = String::from("hello world");

  let word = first_word(&s);

  // 아래 코드는 이제 정상적으로 컴파일 에러를 표시한다.
  s.clear();

  println!("the first word is : {}", word);
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}