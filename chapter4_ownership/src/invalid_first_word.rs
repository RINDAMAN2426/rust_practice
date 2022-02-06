fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s); // word = 5;

  s.clear();

  // 이 경우 word의 5값은 실제로는 아무런 쓸모가 없기 때문에 이를 이용하여 clear된 s에서 추출하려 하면 버그를 유발한다.
  // 그렇다 하여 word의 값을 s의 변화에 따라 조정하는 것은 유용하지도 않고 에러를 유발하기 쉽다.
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();                         // String 타입을 bytes 배열로 return

  for (i, &item) in bytes.iter().enumerate() {  // iter로 bytes배열의 iterator 생성
                                                         // enumerates는 (index, &item) 을 리턴
    if item == b' ' {                                    // b' '는 공백 표현의 바이트 리터럴 문법
      return i;
    }
  }
  s.len()
}