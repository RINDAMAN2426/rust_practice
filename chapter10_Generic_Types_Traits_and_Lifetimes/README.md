# Generic Types, Traits, and Lifetimes

## Removing Duplication by Extracting a Function

```rust
fn main() {
    let number_list = vec![34,50,25,100,65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

위의 코드를 토대로 중복하여 다시 가장 큰 숫자를 구하는 아래 예제

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

두 번째 예제에서 중복되는 부분은 함수로 추상화

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

변경하는 과정은 아래와 같음

1. 중복된 코드를 판단
2. 중복된 코드를 함수의 본문으로 이동, 함수 시그니처에 입력과 리턴 타입에 대해 명시
3. 중복된 코드를 함수 호출로 교체

## Generic Data Types

### In Function Definitions

제네릭을 사용하는 함수를 정의할 때는 특장 타입의 매개변수와 리턴 타입을 사용하는 함수의 시그니처에 사용

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    print!("The largest char is {}", result);
}
```

위 예제는 이전 예제에서 추가로 문자 슬라이스로부터 가장 큰 문자를 찾음

두 함수의 본문은 같기 때문에 이 중복을 제거하기 위하여 `Generic Type`을 사용함

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    print!("The largest char is {}", result);
}
```

위 코드를 컴파일 하면 `error[E0369]: binary operation > cannot be applied to type T` 에러가 발생하게 됨

help를 참고해보면 `fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T` 라고 언급되는데, 타입 T의 값은 반드시 정렬이 가능해야함. 표준 라이브러리는 비교 연산을 수행할 타입들에 대해서 `std::cmp::PartialOrd` 트레이트를 구현할 것을 요구함. 후에 서술할 내용.

### In Struct Definitions

`Struct`에서도 `Generic Type parameter`는 사용 가능

```rust
struct  Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0};
}
```

`x`와 `y`는 같은 타입으로 선언이 되어있기 때문에 다른 타입으로 인스턴스를 생성하려 하면 컴파일 에러 발생

서로 다른 타입으로 선언하고 싶다면 다중으로 파라미터를 선언하면 됨

```rust
struct  Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0};
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

만일 파라미터가 너무 많이 늘어난다면 이는 코드를 더 작게 리팩토링해야한다는 뜻

### In Enum Definitions

`Enum` 또한 사용 가능

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### In Method Definitions

`Struct` 혹은 `Enum`의 메소드에서도 활용 가능함

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

`impl` 키워드 다음 `<T>`를 사용한다면 그 다음에 오는 `Point<T>`의 타입 `T`는 `Generic Type`으로 인식함.
또한 `impl Point<f32>` 와 같이 특정 타입의 인스턴스에만 적용하게끔 구현도 가능함

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0_f32, y: 4.0 };

    println!("disatnce = {}", p.distance_from_origin());
}
```

`Sturct`의 정의에 사용된 `Generic Type`은 메소드 시그니처에 사용한 타입과 다를 수 있음

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {} p3.y = {}", p3.x, p3.y);
}
```

### Performance of Code Using Generics

`Rust`에서 `Generic`을 사용한다고하여 런타임 성능이 떨어지진 않음

`monomorphzation`을 하기 때문인데, 컴파일 시점에 `Generic` 코드를 실제로 사용하는 구체와된 타입으로 변환함

이 과정에서 컴파일러는 우리가 정의할 때 사용했던 것과 반대의 단계를 수행함. `Generic`이 호출되는 부분을 모두 찾아 호출에 사용된 구체화된 타입을 사용하는 코드로 생성함

```rust
let integer = Some(5);
let float = Some(5.0);
```

위의 코드를 컴파일할 때 `monomorphzation`이 수행됨. 컴파일러는 `Option<T>` 인스턴스에 사용된 값들을 읽어 두 종류가 사용되고 있음을 인지함. 그 후 `Option<T>` 의 정의를 `Option_i32`와 `Option_f64`로 확장하고 이로 교체함

`Generic` 코드를 특정 타입으로 사용하는 코드로 컴파일 하기 때문에 별도의 런타임 비용은 들지 않음. 또한 이 과정은 함수로 중복된 코드를 정의하는 것과 같은 성능을 발휘하기 때문에 런타임에 있어서 매우 효율적임

## Traits: Defining Shared Behavior

`trait`는 컴파일러에게 특정 타입이 어떤 기능을 실행할 수 있으며, 어떤 타입과 이 기능을 공유할 수 있는지 알려줌

`trait`는 공유 가능한 행위를 추상화된 방식으로 정의하는 방법임. `Generic`을 결합하여 모든 타입에 특정 행위를 공유할 수도 있음

> 다른 언어의 `interface` 기능과 유사함

### Defining a Trait

타입의 행위는 해당 타입에 대해 호출할 수 있는 메소드로 구성. 이때 다른 타입에 같은 메소드를 호출할 수 있다면, 이 행위는 타입 간에 공유가 가능

`trait`는 어떤 목적을 이루는 데 필요한 행위들을 정의하고, 여러 타입에 적용할 메소드 시그니처를 그룹화 하는 것

특정 구조체들의 인스턴스에 저장된 데이터를 요약하는 `Summary trait`를 선언한다고 하면 아래와 같음

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

`trait`의 메소드 시그니처는 본문 대신 `semi colon`을 덧붙임. 이 `trait`를 구현하는 각 타입들이 본문에 자신의 행위를 구현하면 됨. 하나의 `trait`에 여러 개의 메소드 정의도 가능함

### Implementing a Trait on a Type

`Summary trait`를 토대로 `NewsArticle`과 `Tweet`의 타입에 트레이트를 구현하는 것은 아래와 같음.

`impl`블록에서 `for` 키워드 사용과 메소드 시그니처를 추가함.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: horse_ebooks: of course, as you probably already know, people
}
```

해당 `Struct`와 `trait`를 별도의 `lib.rs`에 선언하였다하면, 다른 누군가 별개의 라이브러리 scope에 정의된 구조체에 `Summary trait`를 구현하려한다면, `xxx::Summary`와 같이 `trait`를 해당 구조체와 같은 scope로 가져와야 함.
또한 `Summary trait`는 공개로 선언해야 다른 `crates`에서도 구현이 가능함

`trait` 구현에 있어서 한 가지 제약은 `trait` 혹은 `trait`를 구현할 타입이 현재 `crates`의 로컬 타입이어야 함. `Display`와 같은 `crates`를 `Tweet`같은 타입에 구현할 수 있는 이유는 `Tweet` 타입이 `crates`의 로컬 타입이기 때문. `Summary trait`는 `crates`의 로컬 타입이기 때문에 `Vec<T>` 타입에 `Summary trait`를 구현할 수도 있음.

단 외부 타입에 외부 `trait`를 구현할 수는 없음. `crate` 안에서 `Vec<T>`타입에 `Display trait`는 구현이 불가능함 `Display`와 `Vec<T>` 모두 표준 라이브러리의 타입, 즉 `crate`의 로컬 타입이 아니기 때문. 이 제약은 `coherence`라고 부르는 특성이며, `orphan rule`이라고도 함. 즉 부모 타입이 존재하지 않는다는 말. 이 제약의 이유는 다른 이의 코드로 인하여 내가 작성한 코드에 문제가 생기거나 반대 경우가 생기는 일이 없도록 하기 위함. 이 제약이 없다면 같은 타입의 같은 트레이트가 두 개의 `crate`에서 구현될 수 있으며, `Rust`는 어떤 것을 사용해야할 지 모르게 되기 때문

### Default Implementations

기본 동작을 구현하고, 사용하려면 `trait`의 본문을 작성하고, 해당 타입의 `impl` 블록은 비우면 가능함. 기본 구현의 재정의는 이전 예시와 동일함.

```rust
// lib.rs
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}

// main.rs
use chapter10_Generic_Types_Traits_and_Lifetimes::*;

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize()); // New article available! (Read more...)

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: horse_ebooks: of course, as you probably already know, people
}
```

기본 구현은 같은 `trait`의 다른 메소드를 호출할 수도 있음. 다른 메소드가 기본 구현을 제공하지 않아도 가능.

```rust
// lib.rs
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}


// main. rs
use chapter10_Generic_Types_Traits_and_Lifetimes::*;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: (Read more from @horse_ebooks...)
}
```

### Traits as Parameters

`Summary trait`를 구현하는 타입을 전달할 수 있는 `item` 매개변수의 `summarize` 메소드를 호출하는 `notify`함수 작성

```rust
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```

매개변수의 타입은 실제 타입이 아닌 `impl` 키워드와 `trait`의 이름을 이용. 해당 `trait`를 구현하는 모든 타입이 인수로 전달될 수 있음. 즉 함수 본문내 `summarize`와 같이 `Summarize trait`에 정의된 메소드는 무엇이든 호출이 가능함. 또한 매개변수로 `NewsArticle`이나 `Tweet`같은 타입의 인스턴스도 전달 가능. 반대로 말하면 `String`, `i32`와 같이 `Summarize trait`가 없는 타입은 전달 불가능.

#### Trait Bound Syntax

`impl Trait` 문법은 간단한 경우에는 잘 동작하지만, `trait bound`라고 부르는 긴 문법을 간단히 표현하기 위한 편의 장치. `trait bound`는 아래와 같음.

```rust
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}
```

`trait bound`는 `Generic Type`에 `colon`을 이용하여 지정함.

`impl Trait` 문법은 훨씬 편리하고 깔끔한 코드를 작성할 수 있으며, `trait bound` 문법은 복잡한 경우를 대처할 수 있음

만일 `item1`과 `item2`의 타입이 다른 타입을 전달할 수 있도록 한다면 아래와 같이 표현하며

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

두 타입이 같은 타입이라면 `trait bound`를 <strong>이용해야만(only)</strong> 가능함

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

#### Specifying Multiple Trait Bounds with the + Syntax

하나 이상의 `trait bound`를 정의하는 것 또한 가능함. 만일 `item` 매개변수 값에 출력 형식을 적용하는 동시에 `summarize` 메소드를 호출한다면 `notify`는 `Display`,`Summary` 두 `trait`를 모두 구현해야함. 이는 `+` 문법을 이용해 명시가 가능함.

```rust
pub fn notify(item: &(impl Summary + Display)) {

pub fn notify<T: Summary + Display>(item: &T) {
```

#### Clearer Trait Bounds with where Clauses

너무 많은 `trait bound`는 좋지 않음. 각각의 `Generic`은 각자의 `trait bound`를 가지고 있기 때문.

여러 개의 `Generic Type`을 갖는 함수는 함수 시그니처의 가독성이 떨어짐.

이를 해결하기 위해 `where`를 이용하여 선언이 가능함

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

위의 함수는 아래와 같이 수정 가능함

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

#### Returning Types that Implement Traits

`impl Trait` 문법은 특정 `trait`를 구현하는 타입을 리턴 값으로 사용할 때도 활용이 가능함

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

리턴 타입이 `impl Summary`이기 때문에 `returns_summarizable` 함수는 실제 타입 이름을 사용하지 않더라도 `Summarize trait`가 구현된 타입이라면 어떤 타입도 리턴으로 가질 수 있음. 위의 케이스는 `Tweet`을 리턴하지만 이 함수를 호출하는 코드는 실제 리턴 타입을 알지 못함.

````rust
    let tweet = returns_summarizable(); // tweet type is impl Summary

    println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: (Read more from @horse_ebooks...)
    ```
````

리턴 타입이 구현해야할 `trait`를 명시하는 방법은 후에 서술될 `closures`와 `iterators`에서 더 유용함.
두 가지는 컴파일러만 알고 있는 타입을 사용하거나 이름이 굉장히 긴 타입을 생성하기 때문. `impl Trait`를 사용하면 긴 타입을 명시하지 않아도 `Iterator trait`를 구현하는 타입을 리턴하는 함수를 쉽게 정의 가능함.

`impl Trait`는 하나의 타입을 리턴하는 경우만 사용가능 함. 해당 함수가 아래와 같이 `NewsArticle`이나 `Tweet` 타입을 리턴한다면 동작하지 않음. 이는 컴파일러가 해당 문법을 구현하는 방법의 제약 때문인데 이 또한 후에 서술하도록 함

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

#### Fixing the largest Function with Trait Bounds

앞서 보았던 `largest` 함수로 되돌아 간다면 해당 함수는 `>` operator를 통해 `T`타입 간의 비교를 행함.
이 연산자는 표준 라이브러리 `std::cmp::PartialOrd`의 메소드에 정의되어 있으므로 `T` 타입에 `PartialOrd`의 `trait bound`를 지정하여 해당 함수가 실제로 비교할 수 있는 타입의 슬라이스만 처리하도록 해야함.

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T { // error[E0508]: cannot move out of type `[T]`, a non-copy slice
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

`error[E0508]: cannot move out of type [T], a non-copy slice` 에러가 발생하게 되는데 `i32`와 `char`의 경우 크기가 정해진 타입들은 스택에 저장되므로 `Copy trait`를 구현하고 있지만, 위의 함수는 `Generic Type`을 사용하고 있기 때문에 `Copy trait`이 구현되지 않을 가능성이 있음. 따라서 타입 `T`는 `Copy trait`을 구현되어 있어야하므로 `Copy trait`을 `trait bound`에 추가해주어야 한다.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

`Copy trait` 대신 `Clone`으로 선언해도 가능함. `largest`함수가 `ownership`을 갖게 되었을 때 각 슬라이스 값을 복제하면 됨. 다만 `String`처럼 heap 데이터를 사용하는 타입은 더 많은 heap 메모리를 할당하며, 이는 많은 양의 데이터를 처리할 때는 속도가 떨어질 수 있음.

`largest`를 구현하는 또 다른 방법은 슬라이스에 저장된 타입 `T`의 참조를 리턴하면 가능함. 리턴 타입을 `T`에서 `&T`로 변경하고 함수의 본문도 참조를 리턴하도록 수정하면 `Clone trait`나 `Copy trait` 의 경계 선언이 필요 없으며 힙 메모리의 할당 또한 필요하지 않음

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    &largest
}
```

#### Using Trait Bounds to Conditionally Implement Methods

`Generic Type`을 사용하는 `impl` 블록에 `trait bound`를 사용하면 타입이 특정 `trait`를 구현하는 지에 따라 메소드를 구현할 수 있음

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

또한 타입이 원하는 `trait`를 구현하는 경우에만 다른 `trait`를 조건적으로 구현하게 할수 있음.

타입에 `trait bound`를 만족하는 `trait`를 구현하는 것을 `blanket implementations`라고 하며, 표준 라이브러리에서 매우 빈번하게 사용하는 기법.

예를 들어 `Display trait`를 구현하는 타입에는 `ToString trait`도 함께 구현함.

```rust
impl<T: Display> ToString for T {
  // --snip--
}
```

이를 통해 `Display trait`를 구현한 모든 타입에 대해 `ToString trait` 가 정의한 `to_string` 메소드도 호출 가능함

```rust
let s = 3.to_string();
```

`blanket implementations`에 대해서는 `Implementors`를 참고.

## Validating References with Lifetimes

`lifetime`은 우리가 원하는 동작을 실행하도록 보장하는 것이 아닌, 타입에 대한 참조가 원하는 시점까지 유효하도록 보장.

`Rust`의 모든 참조에는 `lifetime`이 있음. 타입이 대부분 `inferred`에 의해 결정되는 것처럼 `lifetime` 또한 마찬가지.

하나 이상의 타입이 바인딩될 수 있을 때에는 `type annotation`을 붙여야 하는 것처럼 참조의 수명이 달라질 수 있을 때에는 `lifetime annotation`을 붙여야함. `Rust`는 런타임에 실제 사용되는 참조가 유효한지 확인하기 위하여 `Generic Lifetime` 매개변수를 이용해 관계를 설명해달라 요구함.

### Preventing Dangling References with Lifetimes

```rust
    {
        let r;
        {
            let x = 5;
            r = &x; // error[E0597]: `x` does not live long enough
        }

        println!("r: {}", r);
    }
```

바깥쪽 scope의 변수 `r`은 초기값이 없고 `x`는 5로 초기화가 된 상태. 안쪽 scope에서 변수 `r`에 변수 `x`에 대한 참조를 대입. 그 이후 다시 바깥쪽 scope에서 변수 `r`을 출력하려함. 그러나 이 변수 `r`은 사용하기 전에 이미 범위를 벗어난 값을 참조하고 있기 때문에 해당 코드는 컴파일 되지 않음. 이를 컴파일러가 알 수 있는 이유는 바로 `borrow checker`를 이용하기 때문.

### The Borrow Checker

```rust
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+}
```

변수 `x`의 수명인 `b`블록은 `a`블록 보다 훨씬 작음. `Rust`는 컴파일 타임에 두 `lifetime`의 크기를 비교하여 변수 `r`이 더 작은 `lifetime`을 가진 변수의 메모리를 참조하고 있음을 알아냄. 이를 통해 `b`가 `a`보다 작기 때문에 컴파일을 허락하지 않음.

### Generic Lifetimes in Functions

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

위와 같은 예제 코드에서 `longest` 함수 작성의 주의점은 매개변수의 `ownership`을 갖지 않도록 해야함.
하지만 아래와 같은 에러로 컴파일에 실패함

```rust
// error[E0106]: missing lifetime specifier
fn longest(x: &str, y: &str) -> &str { // expected named lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

컴파일러 입장에서는 리턴값이 `x`인지 `y`인지 알 수 없기 때문에 리턴 타입에 `Generic Lifetime` 매개변수를 지정해야 한다고 설명. 이를 수정하기 위해선 `borrow checker`가 수명 분석을 할 수 있도록 해야함.

### Lifetime Annotation Syntax

`lifetime annotation`은 참조의 유효기간을 변경하진 않음. `generic lifetime`을 지정하면, 어떤 수명의 참조도 전달할 수 있음. `lifetime annotation`은 `lifetime`에 영향을 주지 않으면서 참조 간의 `lifetime`의 관계를 서술할 수 있음.

문법은 `apostrophe(')`와 소문자로 구성. 일반적으로 `'a`를 많이 사용함

```rust
&i32 // 참조
&'a i32 // 명시적인 수명을 가진 참조
&'a mut i32 // 명시적인 수명을 가진 가변 참조
```

`lifetime annotation`은 그 자체로 많은 의미를 갖지는 않음. `generic lifetime`이 지정된 각 참조가 어떤 관계인지 만을 명시할 뿐임.

### Lifetime Annotations in Function Signatures

`longest`함수의 모든 파라미터와 리턴 값은 같은 `lifetime`을 가져야하므로 아래와 같이 수정

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result); // The longest string is abcd
}
```

기억해야할 점은 `generic lifetime`은 `lifetime`을 변경하거나 하지 않고, 그저 `borrow checker`가 이 제약에 일치하지 않는 값을 사용하지 못하게 하는 것.

`lifetime annotation`은 함수의 본문이 아닌 시그니처에 정의해야함.

함수가 함수의 외부로부터 들어오거나, 외부로 나가는 값의 참조를 가졌을 경우에는 `Rust`가 매개 변수나 리턴값 자체의 `lifetime`을 확인하기는 거의 불가능하기 때문에 (이 `lifetime`은 함수가 호출 될 때마다 바뀔 수 있음), 이럴 때에는 직접 `lifetime`을 추가해주어야 함.

`longest` 함수의 `'a`는 변수 `x`의 범위이면서도 변수 `y`의 범위와 겹쳐지는 부분을 표현하기 때문에 실제 `x`,`y`의 수명보다 작은 범위의 수명을 갖게 됨. 리턴 값 또한 `'a`를 적용했기 때문에 마찬가지로 작은 범위를 갖게 됨.

```rust
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
```

`string1`은 바깥쪽에서 유효하며, `string2`는 안쪽에서 유효하다. `result` 역시 안쪽에서 유효하다. 이 코드를 실행한다면 `borrow checker`는 유효하다고 판단, `longest`함수의 return 값은 `long string is long`이 됨.

```rust
// error[E0597]: `string2` does not live long enough
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
```

`println!`에서 사용되는 `result`가 유효하기 위해선 `string2`가 바깥쪽 scope에서도 유효해야 가능함.
`Rust`가 이를 알 수 있는 이유는 `'a`를 이용하여 매개변수와 리턴값이 `lifetime`이 같은 범위내에 있어야 한다고 지정했기 때문.

### Thinking in Terms of Lifetimes

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

`longest` 함수의 리턴값이 항상 첫 번째 매개변수를 리턴한다면 `y`의 `lifetime`은 지정할 필요가 없음.
이게 가능한 이유는 `x`와 리턴값이 `y`의 `lifetime`과는 관계가 없기 때문

함수가 참조를 리턴한다면 매개변수 중 하나의 매개변수의 `lifetime`과는 일치해야함.
만일 리턴값이 함수 매개 변수 중 하나를 참조하는게 아니라면 이는 함수 내에서 선언된 값을 참조한다는 의미일테고, 이 참조는 함수의 scope를 벗어나는 순간 유효하지 않은 죽은 참조가 된다.

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

`Rust`는 죽은 참조가 발생하는 상황은 절대 허락하지 않으며, 만일 그럼에도 이 형태로 고쳐야 한다면 `onwership`을 리턴하여 호출하는 쪽에서 값을 직접 해제할 수 있도록 해야함.

### Lifetime Annotations in Struct Definitions

`Struct`에 참조를 저장하려면 모든 참조에 `lifetime annotation`을 추가해야 함.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

`part` 필드는 참조 타입의 문자열 슬라이스를 저장. `Generic Lifetime` 매개변수를 구조체에 지정하였음.
이 `annotation`이 의미하는 바는 해당 `struct`의 인스턴스는 `part`필드에 저장한 참조의 `lifetime`을 벗어날 수 없음을 의미함.

### Lifetime Elision

아래 함수는 `lifetime annotation`이 없음에도 컴파일이 되는 이유가 무엇일까.

```rust
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}
```

초기버전에서는 모든 참조가 `lifetime`이 있어야 하므로 해당 시그니처의 형태는 아래와 같음.

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

특정 상황에서 같은 `lifetime annotation`이 발생함을 발견하였고, 이런 상황은 충분히 예측할 수 있으며 몇 가지 결정적인 패턴을 따름. 이 패턴을 컴파일러의 코드에 추가하여 `borrow checker`가 해당 상황에서는 `lifetime`을 `inferred`하여 명시적인 `annotation`이 필요하지 않도록 수정함.

해당 패턴은 `lifetime elision rules`라고 불리우며 이는 프로그래머가 준수해야할 룰이 아닌, 컴파일러가 고려해야할 상황을 의미함.

`elision rules`은 완벽한 추론을 제공하진 않음. 이런 모호한 케이스는 에러를 띄워서 `lifetime annotation`을 요구함

함수, 메소드의 매개변수에 적용되는 `lifetime`은 `input lifetimes`, 리턴값은 `output lifetimes`라고 함.

컴파일러는 3가지 규칙을 이용하여 어떤 `lifetime`을 적용할 것인지 판단.

#### elision rules

1. `input lifetime`을 적용 -> 각 참조 매개변수는 각각 `lifetime` 매개변수가 있어야 함.
2. `output litfetime`을 적용 -> 명시적으로 하나의 `input lifetime`이 있으면, 모든 `output lifetime`에 적용
3. `output litfetime`을 적용 -> `input lifetime`가 하나 이상이며 메소드로 선언되어서 매개변수 중 하나가 `&self` 혹은 `&mut self`라면, `self` 변수의 `lifetime`을 모든 `output`에 적용함.

그럼에도 판단할 수 없으면 에러를 발생시킴.

위 규칙대로 `first_word` 함수를 변화시켜보면 아래와 같음

```rust
fn first_word(s: &str) -> &str {

fn first_word<'a>(s: &'a str) -> &str {

fn first_word<'a'>(s: &'a str) -> &'a str {
```

`input`이 두개인 경우는 아래와 같음 (1번 규칙까지 적용한 상태)

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

`input lifetimes`가 2개이므로 2번 규칙은 제외함. 또한 메소드가 아니기 때문에 3번 규칙도 제외함.

따라서 `output lifetimes`의 `lifetime`을 알 수 없기 때문에 컴파일러가 에러를 내어 명시적인 `lifetime`을 요구함.

### Lifetime Annotations in Method Definitions

`struct`의 필드의 `lifetime`은 `impl` 다음에 선언하며 `struct` 이름 다음에 명시해야 함. 수명은 `sturct`의 타입 일부이기 때문.

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

1. `lifetime` 매개변수를 `impl` 다음에 선언하며 `struct` 다음에도 지정을 하였음.
2. 첫번째 생략 규칙 덕분에 `self` 매개변수에 수명을 지정할 필요가 없음

아래는 세번째 생략 규칙이 적용되는 사례

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

### The Static Lifetime

`static lifetime('static)`은 전체 프로그램에 적용됨. 모든 문자열 리터럴은 `static lifetime`

```rust
let s: &'static str = "I have a static lifetime.";
```

이것은 프로그램의 바이너리에 직접 저장되며 사용할 수 있음. 종종 참조에 `'static`를 적용하라는 메세지가 있는데, 적용하는 것은 고려를 해야할 사항. 대부분은 죽은 참조 혹은 `lifetime` 불일치이기 때문에 이런 부분에서 체크를 해봐야 함.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

`Generic Type T`의 `ann`이라는 매개변수 추가.

매개변수에 앞서 `Generic` 타입의 매개변수 값을 출력. 따라서 `Display trait bound`가 필요. 해당 매개 변수는 `Display trait`를 구현한 타입이라면 어떤 것이든 가능.

`lifetime` 또한 `Generic`. 그러므로 함수 시그니처의 `angle brackets`에 같이 추가하도록 함.
