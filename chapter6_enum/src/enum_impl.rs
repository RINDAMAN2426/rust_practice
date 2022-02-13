
enum Message {
  Quit, // 연관 데이터 없음
  Move { x: i32, y: i32 }, // anonymous struct
  Write(String), // String
  ChangeColor(i32, i32, i32), // 3개의 i32
}


impl Message {
  fn call(&self) {
      //
  }
}

fn main(){
  let m = Message::Write(String::from("hello"));

  m.call();
}