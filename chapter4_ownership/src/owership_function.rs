fn main() {
  let s = String::from("hello"); // 변수 s가 main 스코프 내에 생성된다.

  takes_ownership(s); // s의 값은 함수 내로 이동하고 이 시점부터 변수 s는 더이상 유효 하지 않다.

  println!("{}", s); // Error 따라서 s를 호출할 수 없다.

  let x = 5; // 변수 x가 범위 내에 생성된다.

  makes_copy(x); // 변수 x의 값이 함수 내로 이동한다.
                 // i32값은 copy를 수행하므로 이 시점 이후로도 여전히 휴효하다.

  println!("{}", x); // 5
} // 이 시점에서 변수 x가 스코프를 벗어난 후, 변수 s도 범위를 벗어나지만 변수 s의 값은 함수 내로 이동했기 때문에 아무런 일도 일어나지 않는다.

fn takes_ownership(some_string: String) { // some_string 변수가 범위 내에 생성된다.
  println!("{}", some_string);
} // 이 시점에서 some_string 변수가 범위를 벗어나며 'drop'이 호출되고 some_string의 메모리가 해제된다.

fn makes_copy(some_integer: i32) { // some_integer 변수가 범위 내에 생성된다.
  println!("{}", some_integer);
} // 이 시점에서 some_integer 변수가 범위를 벗어나지만 아무런 일도 일어나지 않는다.