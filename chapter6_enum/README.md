# Enum

## Defining an Enum

```rust
enum IpAddrKind {
  V4,
  V6,
}
```

`V4`,`V6`은 `variants`

### Enum values

```rust
  let four = IpAddrKind::V4;
  let six = IpAddrKind:V6;

  fn route(ip_type: IpAddrKind) { }

  route(IpAddrKind::V4);
  route(IpAddrKind::V6);
```

값을 표현할 땐 `double colon (::)` 을 사용

열거자를 매개변수로 가질 수 있음

#### ip 주소 종류와 데이터를 저장하는 예시

```rust
enum IpAddrKind { // 열거자 타입 정의
    V4,
    V6,
}

struct IpAddr { // 구조체 정의
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr { // 인스턴스 생성
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

위 예시는 열거자만을 이용해서 더 간편하게 수정 가능

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

열거자의 값에 직접 데이터를 저장 가능

구조체에 대비하여 이점은 열거자에 나열된 각각의 데이터는 서로 다른 타입과 다른 수를 보유할 수 있음

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

#### 표준 라이브러리 예시

```rust
struct Ipv4Addr {
  ..
}

struct IpvADdr {
  ..
}

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}
```

예시와 같이 열거자의 값은 어떠한 타입도 가능

해당 라이브러리 타입을 코드의 범위로 가져오지 않는 한 같은 타입의 이름을 재정의할 수 있음

[enum_with_different_type.rs](./src/enum_with_different_type.rs) 참조

#### 각기 다른 타입으로 정의한 Message

```rust
enum Message {
    Quit, // 연관 데이터 없음
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String), // String
    ChangeColor(i32, i32, i32), // 3개의 i32
}
```

이와 다르게 각각의 `variant`를 서로 다른 타입의 `struct`로 구현을 하게 되면 여러 종류의 `Message`를 매개변수로 받는 함수를 정의하기 어려움

`impl`을 이용하여 구조체와 마찬가지로 열거자도 메서드 정의가 가능

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
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
```

### Option 열거자를 Null 대신 사용할 때의 장점

러스트는 `null값`이라는 개념이 없다. `null`과 `not-null`인 데이터는 사방에 존재하기 때문에 에러를 유발하기 쉽다.

`null값`은 없지만, 어떤 값의 존재 여부를 표현하는 열거자를 정의

```rust
enum Option<T> {
  Some(T),
  None,
}
```

`Option<T>`는 `prelude`에도 포함되어 있음. 명시적으로 범위로 가져올 필요가 없음

`Option::` 접두어 없이, `Some`,`None` 값을 직접 사용 가능

`<T>`는 `generic type` 매개 변수를 의미, 이를 통해 `Some`은 어떤 타입의 데이터도 저장 가능

```rust
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
```

`None`의 경우 열거자의 타입이 어떤 타입인지 알려주어야 함. 명시하지 않으면 컴파일러가 Some 값이 어떤 타입인지 모르기 때문에 `Option<{unknown}>` 처리. 에러 발생함.

```rust
consider giving `absent_number`the explicit type`Option<T>`, where the type parameter `T` is specified
```

`Option<T>`와 `T`는 다른 타입이기 때문에 컴파일러는 값이 명확히 존재할 때는 `Option<T>` 값을 사용하는 걸 허락하지 않는다.

```rust
// error[E0277]: cannot add `Option<i8>` to `i8`
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; //no implementation for `i8 + Option<i8>`
}
```

이를 위해선 `T`타입에 대한 수행 전 `Option<T>`타입을 `T`로 변환해주어야 한다. `Option<T>`는 여러 상황에서 사용할 수 있는 다양한 메서드를 제공한다.

https://doc.rust-lang.org/std/option/enum.Option.html

`match` 표현식은 열거자의 값에 따라 다른코드를 실행 할 수 있게 해주는 `control flow operator`

## The match Control Flow Operator

일련의 패턴과 일치하는 코드를 실행

패턴의 종류는 `literal`, `variable names`, `wildcards` 등등 다양한 값

`match` as being like `a coin-sorting machine`, 처음으로 일치하는 패턴을 찾으면 해당 패턴의 코드 블록을 실행

```rust
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
```

### Patterns that Bind to Values

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main(){
    value_in_cents(Coin::Quarter(UsState::Alaska)); // State quarter from Alaska!
}
```

### Matching with Option<T>

```rust
fn main(){
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None); // 첫 번째 패턴과 일치하기 때문에 나머지 패턴과의 비교는 실행하지 않음
}
```

`bind`와 함께 사용해서, 열거값에 저장된 데이터를 변수에 바인딩한 후 연관된 코드를 실행하는 패턴이 잦다.

### Matches are exhaustive

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
```

[coin.rs](./src/coin.rs)의 `quarter` 주석 참조

패턴 매칭은 `exhaustive` 해야함, 특히 `Option<T>`의 경우 `None`에 해당하는 경우를 명시적으로 처리하도록 강제

### Catch-all Patterns and the `_ PlaceHolder`

모든 경우를 다 처리하고 싶지 않을 때 사용하는 패턴

```rust
let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => println!("not one or three"),
        // other => println!("not one or three, value is {}", other),
    }
```

값을 사용해야할 경우 `other`를 사용한다.

## Concise Control Flow with `if let`

일반적인 `match` 사용 케이스

```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("max is {}", max),
        _ => (),
    }
```

`if let` 사용 케이스

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max is {}", max);
    }
```

`if let`을 사용하면, 코드의 양이 줄어들겠지만 `match`가 제공하듯 모든 케이스를 검사하도록 강요할 수는 없다.

`if let`은 `syntax sugar`

`if let`은 `else`를 포함 할 수 있다. `_ placeholder`의 코드 블록에 해당

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState)
}

fn main(){
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);

    match coin {
        Coin::Quarter(state) => println!("{:?} state's quarter", state),
        _ => count +=1,
    }

}
```

위의 `match`는 아래와 같이 수정 가능

```rust
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
```
