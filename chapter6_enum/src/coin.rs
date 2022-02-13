enum Coin {
  Penny,
  Nickle,
  Dime,
  Quarter
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
      Coin::Penny => { // 코드가 짧을 경우 curly brackets 사용
          println!("Lucky penny!");
          1
      },
      Coin::Nickle => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25, // 누락되면 error[E0004]: non-exhaustive patterns: `Quarter` not covered 에러 발생
  }
}

fn main(){
  println!("hello")
}