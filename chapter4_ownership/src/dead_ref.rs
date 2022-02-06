fn main() {
  let ref_to_nothing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");

  &s // 변수 s는 dangle 스코프내에 생성되었기 때문에, 실행 후에 메모리는 해제되게 된다.
     // 해제된 메모리를 계속하여 참조하려 시도하기 때문에 에러를 유발할 수 있으므로, 컴파일에서 허용되지 않는다.
}