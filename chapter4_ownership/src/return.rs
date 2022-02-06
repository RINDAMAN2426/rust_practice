fn main() {
  let s1 = gives_ownership(); // gives_ownership 의 리턴값이 변수 s1으로 옮겨진다.

  let s2 = String::from("hello"); // 변수 s2가 범위 내 생성된다.

  let s3 = takes_and_gives_back(s2); // 변수 s2는 taks_and_gives_back으로 옮겨간 후 리턴값은 s3으로 옮겨진다.
} // 이 시점에서 변수 s1, s3는 drop함수가 호출된다.
  // 변수 s2또한 스코프를 벗어나지만 takes_and_gives_back로 옮겨갔기 때문에 아무런 일도 일어나지 않는다.

fn gives_ownership() -> String {          // gives_ownership 함수의 리턴값은 호출한 함수로 옮겨진다.
  let some_string = String::from("hello");// some_string이 스코프 내에 생성된다.

  some_string                             // 리턴되고 나면 호출한 함수로 옮겨진다.
}

fn takes_and_gives_back(a_string: String) -> String { // String을 전달받아 다시 리턴한다.
  a_string                                            // a_string이 스코프내에 생성된다.
}                                                     // 리턴하고 나면 호출한 함수로 옮겨진다.