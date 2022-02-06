#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 { // 값을 읽기만 할 뿐 쓰지않기 때문에 소유권이 필요 없다.
                          // 만일 변경하는 함수라면 &mut self와 같이 선언해야 한다.
    self.width * self.height
  }
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };

  println!("사각형의 면적: {} 제곱 픽셀", rect1.area());
}