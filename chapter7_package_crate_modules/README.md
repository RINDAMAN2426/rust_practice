# Managing Growing Projects with `Packages`, `Crates`, andpub mod hosting;

// front_of_house/hosting.rs
pub fn add_to_waitlist() {}ules`

패키지는 `multiple binary crates`를 포함. `library crate`도 추가 가능

`cargo`는 패키지 간에 연관이 많은 대형 프로젝트를 위한 `Cargo workspace`를 제공한다.

`module system` 은 아래와 같다

- `Packages` A Cargo feature that lets you build, test, and share crates
- `Crates` A tree of modules that produces a library or executable
- `Modules and use` Let you control the organization, scope, and privacy of paths
- `Paths` A way of naming an item, such as a struct, function, or module

## `Packages` and `Crates`

`crate`는 `binary`, `library`

`crate root`는 `Rust compiler`가 컴파일을 시작해서 `root module of crate`를 만들어 내는 소스 파일

`package`는 기능을 제공하는 하나 이상의 `crates`

`Cargo.toml`를 통해 `crates`를 어떻게 빌드할 지 서술한다.

```sh
$ cargo new my-project
```

위 명령어를 실행하면 `Cargo.toml`을 생성해서 패키지를 만듬

이 때 `Cargo.toml`에는 `src/main.rs`에 대한 언급이 없는데, `Cargo`는 `src/main.rs`가 패키지와 같은 이름을 갖는 `binary crate`의 `root crate`라는 컨벤션을 따르기 때문

마찬가지로, `sr/lib.rs`가 있다면 해당 패키지는 패키지와 같은 이름의 `library crates`를 가진다고 판단, `src/lib.rs`를 `root crate`로 인식

`Cargo`는 `library`나 `binary`를 빌드 시에 `rustc`에게 해당 `root crate`를 전달

위 명령어를 통해 생성한 `src/main.rs`만 있는 패키지는 `my-project`라는 이름의 `binary crate`를 포함한다는 의미

`src/main.rs`와 `src/lib.rs` 모두 존재할 경우, `library`와 `binary` 두 `crate`를 가진다는 의미(이름은 package name과 동일)

`package`는 `src/bin` 디렉토리에 파일을 둠으로써 여러개의 `binary crates`를 가질 수 있으며, 이 때 해당 파일들은 각각의 `binary crates`로 분리

`crate`는 관련된 기능들을 하나의 `scope`로 그룹화 함으로써 해당 기능을 여러 프로젝트간에 공유하기 편하게 함

`crate`는 기능을 해당 `crate`안에서 구현하면, 작성 중인 `crate`에서 정의된 기능인지 다른 `crate`에서 정의된 기능인지 명확히 구분이 가능 하며 이는 잠재적인 이름 충돌을 해결

## Defining `Modules` to Control `Scope` and `Privacy`

`path`는 아이템의 이름을 결정하며, `use`는 해당 경로를 `scope`안으로 가져옴

`pub`은 아이템을 외부에 공개

`module`은 아이템의 `privacy`를 조절함. 외부의 공개 가능하게 하거나(`public`), 혹은 외부에서 사용할 수 없는 상세구현(`internal implementation detail`)이거나(`private`)

```rust
//../restaurant/src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

`src/main.rs`, `src/lib.rs` 는 `crate root`

두 파일의 콘텐츠는 `crate`라는 이름의 모듈로 구성되며, `module tree`에서 `root`역할을 하기 때문

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

`hosting module`이 `front of house` 모듈에 있듯이, 일부 모듈은 다른 모듈에 중첩 가능

`hostring module`과 `serving module`처럼 같은 수준에 중첩 가능

전체 모듈 트리의 최 상단에는 `crate`가 존재

## Paths for Referring to an Item in the Module Tree

모듈 트리 내에서 아이템을 찾으려면 경로를 이용

- `absolute path` - `crate`의 이름이나 리터럴을 이용해 `crate root`부터 시작하는 경로
- `relative path` - 현재 모듈로부터 시작하며 `self`,`super` 아니면 현재 모듈의 식별자로부터 시작

경로는 하나 혹은 그 이상의 식별자로 구성되며, 각 식별자는 `double colons(::)`로 구분

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() { // eat_at_restaurant 는 `library crate`의 공개 API
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```

`pub` keyword를 누락하면 `error[E0603]: module `hosting` is private` 에러 발생

`Rust`에서 모든 아이템의 `privacy`는 기본적으로 비공개

부모 모듈은 자식 모듈 안의 비공개 아이템을 사용할 수 없지만, 자식 모듈은 부모 모듈의 아이템을 사용 가능함

### Exposing Paths with the `pub` Keyword

```rust
mod front_of_house {
    pub mod hosting { // Add pub keyword
        pub fn add_to_waitlist() {} // module이 공개되어도 아이템은 공개되지 않는다.

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```

`privacy rules`은 `module` 뿐 아니라 `sturct`, `enums`, `functions`, `methods` 들 전부 적용

`front_of_house`는 `eat_at_restaurant`와 같은 모듈 내 정의되어 있으므로, `eat_at_restaurant`가 정의되어 있는 곳부터 시작(상대경로)해도 접근 가능

### Starting Relative Paths with `super`

`relative path`는 `super`를 사용하여 부모 모듈로부터 시작할 수 있음 (filesystem의 `..` syntax와 동일)

```rust

fn serve_order () {}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
}
```

`fix_incorrect_order`는 `back_of_house`에 정의되어 있기 때문에 `super` 키워드를 이용하여 `back_of_house`의 부모 모듈인 `crate`에 접근 가능

`back_of_house`와 `serve_order` 함수는 서로 같은 관계에 있으므로, `module tree`를 재구성할 때도 함께 이동 -> 따라서 `super`를 이용하면 코드를 다른 모듈로 이동해도 수정해야할 코드를 최소화

### Making `Structs` and `Enums` Public

`struct`를 `pub`으로 공개해도 `struct`의 `field`는 공개되지 않음, 필요에 따라 각 `field`를 공개하거나 비공개로 유지

```rust

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seaonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seaonal_fruit: String::from("peaches")
            }
        }
    }
}
```

`back_of_house::Breakfast`의 toast `filed`는 공개이므로 `eat_at_restaurant` 함수에서 `read`와 `write`가 가능

`seasonal_fruit`의 경우 비공개이기 때문에 접근 불가

`back_of_house::Breakfast`는 `private field`가 있기 때문에 `public associated function`를 제공해야함. 해당 함수가 없다면 `seasonal_fruit`의 값을 설정할 수 없기 때문에, 인스턴스 생성이 불가

대조적으로 `enum`의 경우, `enum`을 공개하면, 그에 해당하는 `variants`도 공개

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // can use soup and salad, because Appetizer enum is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## Bringing Paths into Scope with the `use` Keyword

절대 경로나 상대 경로 모두 `add_to_waitlist`를 호출하려면 `front_of_house`, `hosting` 모듈을 지정해주어야 함 -> `use` keyword를 통해서 이를 해결

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // `use`를 이용해서 `crate::front_of_house::hosting`를 `eat_at_restaurant`의 `scope`로 가져옴

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

`use`의 사용은 `filesystem`의 `symbolic link`와 유사

```rust
use self::front_of_house::hosting;
```

`use`에 상대경로를 지정하는 방법은 현재 범위의 이름부터가 아닌 `self`를 이용하여 사용

### Creating Idiomatic `use` Paths

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

위의 방식은 보다는 이전 예시가 좀 더 `idiomatic way`. 위의 방식은 `add_to_waitlist`의 정의가 어디서 되어있는지 명확하지 못함

아래 코드와 같이 `struct`, `enums` 혹은 다른 아이템을 `use`로 가져올 경우 이는 또 `full path`를 지정하는게 `idiomatic`

특별한 이유는 없고, 그냥 자리잡힌 컨벤션

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

예외적으로 같은 이름을 가진 두 아이템을 현재 범위로 가져올 경우는 `Rust`에서 지원하지 않음

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

`use std::fmt::Result`, `use std::io::Result` 의 형태라면 `Result`의 타입의 부모 모듈이 어느 것을 가리키는지 이해할 수 없기 때문

### Providing New Names with the `as` Keyword

위의 예시와 같은 부분을 해결하기 위해 `as`사용

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

두 예시 모두 `idiomatic`하니까 선택의 영역

### Re-exporting Names with `pub` `use`

`use`를 이용해 `scope`내로 가져오면, 새로운 `scope`에서 다시 비공개됨

이를 다른 `scope`에서 접근 가능하게 하기 위해선 `pub use`를 이용하여 re-exporting

```rust
pub use crate::front_of_house::hosting;
```

### Using External Packages

Chapter2와 같이 외부 패키지를 사용하려면 `Cargo.toml`에 해당 코드를 추가

```rust
// Cargo.toml
[dependencies]
rand = "0.5.5"
```

이를 추가하면 이 패키지와 `rand`패키지 사용을 위한 모든 의존성 패키지를 https://crates.io/ 에서 내려받음

필요한 `crate`는 `Cargo.toml`에 나열하고 `use`를 이용해 해당 `scope`로 가져오면 됨

`Standard Library(std)` 또한 `external crate`. 다만 `Rust`언어와 함께 제공되기 때문에 `Cargo.toml`에 추가할 필요는 없음

### Using Nested Paths to Clean Up Large `use` Lists

같은 패키지 내의 여러 아이템을 사용하기 위해 여러 줄을 사용하면 공간 낭비

```rust
use std::io;
use std::cmp::Ordering;
```

이는 Nested Paths를 사용해서 개선 가능

```rust
use std::{io, cmp::Ordering};
```

중첩 경로는 어디서도 사용가능

```rust
use std::io;
use std::Write;
```

이 구문은 `self`를 사용하면 개선 가능

```rust
use std::io::{self, Write}
```

### The `Glob` Operator

```rust
use std::collections::*;
```

`Glob Operator(*)`를 사용하면 모든 공개 아이템을 가져올 수 있음. 다만 어떤 이름들을 가져왔고, 어디에 정의되어 있는지 모르기 때문에 주의

테스트 코드에서 `tests` 모듈에 정의된 모든 아이템을 가져올 때 주로 사용함

`prelude` 패턴에서도 간간히 사용. https://doc.rust-lang.org/std/prelude/index.html#other-preludes

## Separating Modules into Different Files

```rust
// lib.rs
mod front_of_house

pub use create::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// front_of_house.rs
pub mod hosting;

// front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

`use`는 `crate`의 일부로 컴파일되는 위치가 변경되어도 아무 영향을 받지 않음

`mod` 키워드는 모듈을 선언하며, `Rust`는 모듈이름과 같은 이름의 파일에서 모듈의 컨텐츠를 가져옴
