# Writing Automated Tests

## How to writes test

1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

### The Anatomy of a test function

테스트는 `test attributes`를 이용한 `Rust`의 함수. 테스트 함수로의 전환을 위해선 `fn` 키워드를 사용한 코드 위 `#[test]`를 적용하면 됨. 모듈내 테스트를 위한 함수가 있을 수도 있기 때문에 검증이 필요한 함수에 적용해야함.

`cargo test` 명령어를 이용해 테스트를 실행하면 `test attributes`가 적용된 함수들을 실행하는 `test runner binary`를 빌드하고 각 테스트의 성공여부를 보고

```sh
$ cargo new adder --lib
$ cd adder
```

```rust
// src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

- `#[test] annotation` 을 통해 이 함수가 테스트 함수임을 가리키며 `test runner`는 이를 통해 어떤 함수가 테스트 함수인지 판단
- `assert_eq!` macro를 통해 `2+2 = 4`인지를 확인하고 있음

`cargo test` 명령어를 실행하면 프로젝트 안의 모든 테스트를 실행

- `measured` 의 경우는 benchmark test를 위한 부분. https://doc.rust-lang.org/unstable-book/library-features/test.html 참조

##### 실패 케이스 테스트

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

result:

```sh
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test failed', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

테스트를 실행할 때, `failures`의 테스트 이름을 지정하면, 해당 테스트만 다시 실행하여 디버깅을 더 수월하게 할 수 있음

### Checking Results with the `assert!` Macro

`assert!` macro는 표준 라이브러리가 제공. `boolean`으로 평가되는 표현식을 인수로 전달.

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

- `use super::*` - `tests` module은 `inner module`이므로 `outer module`의 테스트 코드를 `inner module`의 scope내로 가져와야함
- `assert!(larger.can_hold(&smaller));` - 결과값이 `true`이므로, 테스트가 성공

```rust
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(!smaller.can_hold(&larger));
    }
```

- `assert!(!smaller.can_hold(&larger));` - 마찬가지로 결과값이 `true`이므로, 테스트 성공

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

`assert!` macro에 `==` operator를 이용한 표현식을 전달하여 테스트를 할 수도 있음. 그러나 표준 라이브러리에 이를 위한 `assert_eq!`와 `assert_ne!`를 제공. 이 두 가지 macro는 검증이 실패하면 테스트가 왜 실패했는지 알 수 있도록 두 값을 출력해줌.

##### assert_eq!

```rust
#[cfg(test)]

mod tests {
    use super::*;

    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.51s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

##### assert_eq! failed test

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.43s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

- `---- tests::it_adds_two stdout ----` - 이하의 내용을 통해 어떤 값들로 인해 테스트가 실패했는지 알 수 있음

- `'assertion failed: (left == right)` - `expected`와 `actual`에 대해서 `left`, `right`로 표현

`assert_ne!` macro는 반대로 같지 않으면 성공이고, 같으면 실패

내부적으로 두 macro는 각각 `==`와 `=!` operator를 사용. 검증이 실패하면 디버그 형식을 이용해 전달된 인수를 출력함. 따라서 매크로에 전달될 값들은 `PartialEq`와 `Debug` trait가 구현되어야 함. `primitive types`, 표준 라이브러리는 두 traits를 구현하고 있으나 직접 선언한 `struct`와 `enum`은 `PartialEq`, `Debug` trait를 구현해야 동등 비교 및 값 출력을 할 수 있음. 해당 traits들은 상속이 가능하므로 선언 시 `#[derive(PartialEq, Debg)]` annotation을 통해 적용가능

### Adding Custom Failure Messages

`assert!`, `assert_eq!`, `assert_ne!` macro의 선택형 인수를 이용하면 실패 메세지에 사용자 정의 메세지 추가가 가능함. 해당 필수 매개변수 다음에 `format!` macro를 통해 가능함

##### 어떤 문자열을 return 하고 있는지 커스텀 메세지를 통해서 확인하기

```rust
pub fn greeting(_name: &str) -> String {
    String::from("Hello")
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    }
}
```

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello`', src/lib.rs:13:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### Checking for Panics with `should_panic`

코드가 에러 상황을 올바르게 처리하고 있는지에 대한 테스트 또한 중요. `should_panic` attribute를 이용하여, 함수 내 코드가 패닉이 발생하면 테스트를 성공, 발생하지 않으면 실패하도록 처리

###### 패닉 테스트

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
warning: field is never read: `value`
 --> src/lib.rs:2:5
  |
2 |     value: u32,
  |     ^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `adder` (lib) generated 1 warning
warning: `adder` (lib test) generated 1 warning (1 duplicate)
    Finished test [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

위 코드에서 `panic`을 발생시키는 조건문을 삭제하면 아래와 같이 결과가 나옴.

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
warning: field is never read: `value`
 --> src/lib.rs:2:5
  |
2 |     value: u32,
  |     ^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `adder` (lib) generated 1 warning
warning: `adder` (lib test) generated 1 warning (1 duplicate)
    Finished test [unoptimized + debuginfo] target(s) in 0.48s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

`should_panic` 테스트의 경우 의도와는 다르더라도 패닉이 발생하면 성공. 구체적으로 정의하기 위해서 매개변수를 추가해야함.

##### 특정 패닉 메세지를 포함하는 패닉 발생 조건 테스트

```rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
warning: field is never read: `value`
 --> src/lib.rs:2:5
  |
2 |     value: u32,
  |     ^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `adder` (lib) generated 1 warning
warning: `adder` (lib test) generated 1 warning (1 duplicate)
    Finished test [unoptimized + debuginfo] target(s) in 0.48s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

두 패닉 메세지를 바꿔서 의도적으로 실패 테스트를 해보면 아래와 같은 결과가 나옴

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
warning: field is never read: `value`
 --> src/lib.rs:2:5
  |
2 |     value: u32,
  |     ^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `adder` (lib) generated 1 warning
warning: `adder` (lib test) generated 1 warning (1 duplicate)
    Finished test [unoptimized + debuginfo] target(s) in 0.45s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'tests::greater_than_100' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 200."`,
 expected substring: `"Guess value must be less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

## Controlling How Tests Are Run

`cargo run` 명령이 코드를 컴파일하고 결과 바이너리를 실행하는 것과 마찬가지로 `cargo test` 명령은 테스트 모드의 코드를 컴파일하고 결과 테스트 바이너리를 실행. 명령줄 옵션을 적용하여 기본 동작을 변경할 수 있음

옵션의 일부는 `cargo test`명령에 적용되며, 나머지는 결과 테스트 바이너리 생성에 적용. `cargo test --help`는 `cargo test` 명령에 적용가능한 리스트를, `cargo test -- --help`는 테스트 바이너리에 적용되는 옵션들을 설명한다. 구분자는 `--`.

### Running Tests in Parallel or Consecutively

여러 테스트 실행은 기본적으로 threads를 사용한 `parallel`로 실행됨. 동시에 실행되는 만큼 각 테스트는 `shared state`에 대해 독립적이어야함. 따라서 테스트 실행 순서에 의존성이 있다면 다른 방법으로 해결해야함.

`parallel`로 실행되길 원하지 않거나, 사용되는 thread의 개수에 대해 `fine-grained control`이 필요한 경우 `--test-threads`를 이용하여 개수를 지정할 수 있음

##### `parallelism`을 사용하지 않도록 설정

```sh
$ cargo test -- --test-threads=1
```

### Showing Function Output

기본적으로, 테스트 성공에 대해 테스트 라이브러리는 표준 출력에 아무것도 출력하지 않음. 반대로 실패의 경우는 `println` macro와 같은 출력에 대해 실행됨.

```rust
pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

- `---- tests::this_test_will_fail stdout ---- I got the value 8` - failed case에 대해 메세지 확인이 가능. 성공 케이스는 출력되지 않음.

`--show-output`, `--nocapture` flag 를 통해서 성공 케이스도 확인 가능

##### --show-output

```sh
$ cargo test -- --show-output
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

##### nocapture

```sh
$ cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 2 tests
I got the value 8
I got the value 4
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

두 flag에 대한 `--help`에서의 설명은 아래와 같음

- `--nocapture` - don't capture stdout/stderr of each task, allow printing directly
- `--show-output` - Show captured stdout of successful tests

`--nocapture`는 태스크 실행 순서 대로 표준 출력/에러에 대하여 있는 그대로 출력되고, `--show-output`은 `captured`된 성공 케이스의 표준 출력을 보여줌

https://github.com/rust-lang/book 여기랑 https://github.com/rust-lang/cargo/tree/master/src/doc/src 여기 비교해보면 옵션에 대해 예시가 서로 다른데.. 그냥 편한대로 쓰면 되는 것 같음..

### Running a Subset of Tests by Name

##### 특정 이름의 테스트 함수만 실행

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

shell:

```sh
$ cargo test one_hundred
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.29s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

테스트 함수 이름의 일부만 지정하여 일치하는 테스트 실행도 가능

##### 여러 테스트 실행

shell:

```sh
cargo test add
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

두 케이스를 보면 `filtered out`를 통해 필터링 된 케이스 개수 확인 가능

### Ignoring Some Tests Unless Specifically Requested

오래 걸리는 테스트는 무시하도록 할 수 있음

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
```

result:

```sh
$ cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.36s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 2 tests
test tests::expensive_test ... ignored
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `test tests::expensive_test ... ignored` - ignored 된 테스트 함수 표시
- `test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s` - ignored 된 개수 표시

ignored 된 테스트만을 실행하고 싶다면 `--ignored` flag를 이용

```sh
cargo test -- --ignored
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Test Organization

unit tests, integration tests에 대해 `Rust` 커뮤니티 또한 고민 중.

### Unit Tests

unit tests의 목적은 각 코드의 유닛들을 독립적으로 테스트, 코드가 의도대로 동작하는지 빠르게 판단. `src` 디렉토리 파일들에 테스트할 코드와 함께 테스트를 작성함. 컨벤션은 `cfg(test)` annotation을 적용한 `tests`라는 모듈 선언 후 그 안에 테스트 작성.

#### The Tests Module and `#[cfg(test)]`

`#[cfg(test)]`는 `Rust`에게 `cargo test` 명령어 실행의 경우 코드를 컴파일하고 실행하기 위함. 그 외에는 테스트 모듈의 코드는 컴파일되지 않음. integration tests의 경우는 다른 디렉토리에 작성하기 때문에 이 annotation은 필요하지 않음.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

위 코드는 `cargo`가 기본적으로 생성해주는 테스트 모듈. `cfg` attribute를 이용하여 `cargo`는 테스트 명령어 실행시에만 테스트 코드를 컴파일한다.

모듈 이름은 바꿔도 가능.

##### 모듈이름 바꾸기

```rust

#[cfg(test)]
mod custom_named_test_module {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

result:

```sh
cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.35s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test custom_named_test_module::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `test custom_named_test_module::it_works ... ok` - 바꾼 모듈이름으로 반영해줌.

#### Testing Private Functions

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}
```

result:

```sh
cargo test
   Compiling adder v0.1.0 (/Users/brouk.develop/playground/rust_practice/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests (target/debug/deps/adder-66126a49892b93fa)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`Rust's privacy rules`에 의해 `private` 함수 또한 가져와 쓸 수 있음. 이념따라 판단하면 됨.

### Integration Tests

integration tests는 라이브러리의 외부에서 실행됨. 다른 코드들과 마찬가지로, 라이브러리의 public API만 사용하여 실행함. integration tests는 unit tests에서 통과된 코드들이 통합적으로 실행될 때 문제 유발에 대한 테스트이므로 coverage도 상당히 중요함.

#### The `tests` Directory

> `Cargo`는 특별히 `tests` 디렉터리에 대하여 `cargo test` 명령을 실행할 때만 컴파일 하도록 취급함. `tests` 디렉터리의 각 파일은 별개의 crates로 컴파일 됨.

(여기부터 디렉토리 바꾸느라 콘솔값이 좀 달라진게 있음)

##### tests/integration_test.rs

```rust
use chapter11_Writing_Automated_Tests as adder;

#[test]
fn it_adds_two() {
  assert_eq!(4, adder::add_two(2));
}
```

result:

```sh
$ cargo test
   Compiling chapter11_Writing_Automated_Tests v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter11_Writing_Automated_Tests)
    Finished test [unoptimized + debuginfo] target(s) in 0.33s
     Running unittests (target/debug/deps/chapter11_Writing_Automated_Tests-7e4fb620c8f759e8)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-66dda6ff42ea7ae0)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests chapter11_Writing_Automated_Tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `Running unittests (target/debug/deps/chapter11_Writing_Automated_Tests-7e4fb620c8f759e8)` - unit tests 실행 표시
- `Running tests/integration_test.rs (target/debug/deps/integration_test-66dda6ff42ea7ae0)` - `tests` 디렉토리 테스트 실행 표시.

integration tests의 경우는 `--test` flag에 인수로 파일 이름을 지정하면 해당 integration test 함수 실행.

```sh
$ cargo test --test integration_test
```

#### Submodules in Integration Tests

integration tests 양의 너무 많아질 경우 `tests` 디렉터리에 하나 이상의 파일을 생성하는 것이 좋음.

각 파일은 별개의 crates로 컴파일 되므로, 각 파일이 공유할 helper functions들을 정의할 때 주의해야함.

```rust
// tests/common.rs
pub fn setup() {
  // setup code specific to your library's tests would go here
}
```

result:

```sh
$ cargo test
   Compiling chapter11_Writing_Automated_Tests v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter11_Writing_Automated_Tests)
    Finished test [unoptimized + debuginfo] target(s) in 0.51s
     Running unittests (target/debug/deps/chapter11_Writing_Automated_Tests-7e4fb620c8f759e8)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/common.rs (target/debug/deps/common-5a482402cd32ea29)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-66dda6ff42ea7ae0)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests chapter11_Writing_Automated_Tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `Running tests/common.rs (target/debug/deps/common-5a482402cd32ea29)` - `setup` 함수에 대하여 별다른 호출이 없음에도 테스트 결과에 포함되는 것을 확인할 수 있음

따라서 `common.rs`의 섹션이 빠지게 하기 위해선 `tests/common/mod.rs` 의 형태로 구성. `common` 디렉토리명은 바뀌어도 상관없음. `mod.rs`만 유지.

```rust
// tests/common/mod.rs
pub fn setup() {
  println!("called setup");
}

// tests/integration_test.rs
use chapter11_Writing_Automated_Tests as adder;

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, adder::add_two(2));
}
```

result:

```sh
$ cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests (target/debug/deps/chapter11_Writing_Automated_Tests-7e4fb620c8f759e8)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-66dda6ff42ea7ae0)

running 1 test
called setup
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests chapter11_Writing_Automated_Tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `called setup` - `setup` 함수 호출 확인

#### Integration Tests for Binary Crates

`src/main.rs`를 가진 binary crates 라면, `src/main.rs`의 함수들에 대한 integration tests는 불가능함. library crates의 함수들만 외부에서 가져다 사용가능함. `src/main.rs`의 코드량이 적고 잘 동작하면 해당 코드는 굳이 테스트 안해도 됨.
