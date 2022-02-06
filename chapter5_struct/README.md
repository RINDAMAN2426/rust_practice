# 구조체

## 구조체 정의, 인스턴스 생성

구조체는 튜플과 유사, 튜플과 마찬가지로 각기 다른 타입으로 구성된다.

튜플과 다른 점은 각 데이터에 별개의 이름을 부여할 수 있다. 따라서 인덱스에 의존할 필요가 없다.

```rust
struct User {
  // fields
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}
```

구조체를 정의한 후 사용하기 위해선 각 필드에 저장할 값을 명시하여 인스턴스를 생성하면 된다. 인스턴스의 구조는 `key: value` 이다.

```rust
let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someone"),
  active: true,
  sign_in_count: 1,
};
```

인스턴스가 가변 인스턴스라면 새로운 값을 대입할 수 있다.

```rust
let mut user2 = User {
    email: String::from("someone2@example.com"),
    username: String::from("someone2"),
    active: true,
    sign_in_count: 1
  };

  user2.email = String::from("another@exapmle.com");
```

주의할 점은 러스트의 구조체는 몇몇 필드만을 가변 데이터로 표시하는 것을 지원하지 않는다.

함수를 이용해서 인스턴스를 생성할 수 있으며, 필드 이름과 같은 변수이름을 사용하면 아래와 같이 생성할 수 있다.

```rust
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
```

또한 존재하는 인스턴스에서 몇가지 값만 수정하여 인스턴스를 새로 만들어 낼 수도 있다. `struct update syntax`와 `..`를 이용해서 아래와 같이 새로 만들어 낼 수 있다.

```rust
  let user3 = User {
    email: String::from("user3@example.com"),
    username: String::from("user3"),
    ..user1
  };
```

### 이름 없는 필드를 가진 튜플 구조체로 다른 타입 생성하기

튜플과 유사하게 생긴 구조체를 선언할 수도 있다.`tuple structs`

구조체에는 이름을 부여하지만, 필드에는 이름을 부여하지 않고 타입만 지정한다.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0,0,0);
let origin = Point(0,0,0);
```

데이터 타입이 같다하더라도 다른 구조체 타입이다. Color타입을 매개변수로 사용하는 함수에 Point 타입을 전달할 수 없다. 그 외에는 튜플과 동일하게 동작한다.

### 필드가 없는 유사 유닛 구조체

필드가 하나도 없는 구조체를 선언할 수 있다. `unit-like structs`

이 구조체는 유닛타입`()`과 유사하게 동작한다.

유사 유닛 구조체는 어떤 타입의 `trait`를 구현해야하지만, 타입에 저장할 데이터는 없을 때 활용할 수 있다.

#### 구조체 데이터의 소유권

```rust
struct USer {
  username: &str,
  email: &str,
  sign_in_count: u64,
  active: bool
}

fn main() {
  let user1 = User {
    email: "email@example.com",
    username: "user",
    active: true,
    sign_in_count: 1,
  };
}
// Error: missing lifetime specifier
```

`User` 구조체에서 `String`타입을 사용한 이유는 구조체가 데이터의 소유권을 갖게 하기 위함이다. 다른 변수가 소유한 데이터의 참조를 저장할 수 있지만 그렇게 하기 위해선 `lifetimes`를 사용해야한다. 위 코드처럼 수명을 지정하지 않는다면 에러가 발생하게 된다.

## 구조체 예제 프로그램

```rust
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };

  println!(
    "사각형의 면적: {} 제곱 픽셀",
    area(&rect1)
  );
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
```

### 트레이트를 상속해서 기능 추가하기

```rust
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };

  println!("react: {}", rect1);
}
```

위의 코드는 display format에 관련해서 에러가 발생하게된다. 구조체는 어떠한 형태로 출력을 해야할지 결정하기 어렵기 때문이다.

해당 구조체가 디버깅 정보를 출력할 수 있도록 `Debug trait`를 상속하면 해결할 수 있다.

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };

  println!("rect: {:?}", rect1);
}
```

## 메서드 문법

메서드는 구조체의 `context` 내에 정의하며 첫번째 매개변수는 항상 구조체 인스턴스를 표현하는 `self`여야 한다.

```rust
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
```

### 더 많은 매개변수를 갖는 메서드

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };
  let rect2 = Rectangle { width: 10, height: 40 };
  let rect3 = Rectangle { width: 60, height: 45 };

  println!("rect1 > rect2 ? {}", rect1.can_hold(&rect2));
  println!("rect1 > rect3 ? {}", rect1.can_hold(&rect3));
}
```

### 연관함수

self 매개변수를 사용하지 않는 `associated functions`도 정의할 수 있다. 구조체의 인스턴스를 전달받지 않기 때문에 메서드가 아닌 함수이다. `String::from`과 같은 함수가 연관함수이다.

연관함수는 새 인스턴스를 리턴하는 생성자를 구현할 때 자주 사용한다.

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}

fn main() {
  let square1 = Rectangle::square(3);

  println!("{:?}", square1);
}
```

이 함수는 해당 구조체에 대해서만 사용할 수 있다. `:: 문법`은 연관 함수의 호출, 모듈이 생성하는 `namespace`를 정의 할때 모두 사용된다.

### 여러 개의 impl

각 구조체는 여러 `impl` 블록을 선언할 수 있다. 별개의 `impl`블록에 선언하더라도 동일하게 동작한다. 유용하게 쓰이는 예는 제네릭 타입과 트레이트인데 이는 뒤에서 서술한다.
