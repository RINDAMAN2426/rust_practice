# An I/O Project: Building a Command Line Program

## Accepting Command Line Arguments

파일명과 검색할 문자열 등 두 개의 명령줄 arguments를 처리가 목표

```sh
$ cargo run searchstring example-file.txt
```

### Reading the Argument Values

`std::anv::args` 함수를 사용. 이 함수는 arguments의 iterator를 리턴함.

```rust
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
}
```

- `std::env::args` 함수는 argument에 유효하지 않은 유니코드가 포함되어 있으면 패닉 발생. 만일 유효하지 않은 유니코드를 허용해야 한다면 `std::env::args_os`로 대신해서 사용해야함. `OsString` 값의 iterator를 리턴하는데, `OsString`은 플랫폼마다 다르기 때문에 `args` 함수보다 복잡함
- `env::args().collect()` - `collect` 메소드를 통해 iterator의 값들을 `Vector`로 변환.

result:

```sh
cargo run needle haystack
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.06s
     Running `target/debug/chapter12_minigrep needle haystack`
["target/debug/chapter12_minigrep", "needle", "haystack"]
```

- 첫 번째 값은 binary의 이름. 실행 중인 프로그램의 이름이 필요하거나, 프로그램의 동작을 변경해야하는 경우 유용함.

### Saving the Argument Values in Variables

```rust
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let query = &args[1];
  let filename = &args[2];

  println!("Searching for \"{}\"", query);
  println!("In file \"{}\"", filename);
}
```

result:

```sh
$ cargo run test sample.txt
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/chapter12_minigrep test sample.txt`
Searching for "test"
In file "sample.txt"
```

## Reading a File

##### 인수로 전달된 파일 읽기

```rust
use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let _query = &args[1];
  let filename = &args[2];

  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

  println!("With text: \n{}", contents);
}
```

result:

```sh
$ cargo run the poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/chapter12_minigrep the poem.txt`
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

위 예시의 문제점

1. 함수가 복잡함. 함수가 하나의 동작을 수행하는 편이 더 깔끔하고 이해하기 쉬움
2. configuration variables는 하나의 `struct`에 모아서 목적을 명확히 하는게 관리하기 좋음
3. 파일을 열지 못했을 때 적합한 에러 처리를 하고 있지 않음
4. 다른 종류의 에러 처리를 위해 `expect` 함수가 반복적으로 사용하면, 사용자가 프로그램에 필요한 arguments를 구체적으로 명시하지 않으면 인덱스를 벗어나는 에러가 발생함. 에러 처리 로직을 한 곳에 모아두면 의미 있는 에러 메세지 출력에도 도움이 됨

## Refactoring to Improve Modularity and Error Handling

### Separation of Concerns for Binary Projects

`main`함수의 크기가 증감에 따라 관심을 분리하기 위한 지침

1. 프로그램은 `main.rs`와 `lib.rs` 파일로 구분. 로직은 `lib.rs` 파일로 옮김
2. 파싱 로직이 작다면 `main.rs` 에 남겨둠.
3. 파싱 로직이 복잡해진다면, `lib.rs` 파일로 옮김

이 지침을 수행한 후의 `main`함수의 역할은 아래와 같게 된다.

- arguments를 이용한 파싱 로직 호출
- 다른 설정 적용
- `lib.rs`의 `run` 함수 호출
- `run` 함수가 에러를 리턴할 경우 이에 대한 처리

#### Extracting the Argument Parser

```rust
use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let (_query, filename) = parse_config(&args);

  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

  println!("With text: \n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
  let query = &args[1];
  let filename = &args[2];

  (query, filename)
}
```

#### Grouping Configuration Values

튜플을 리턴한 후에, 이 값들을 개별 변수로 다시 대입하고 있다는 것은 적절한 추상화를 적용하지 않았다는 반증이기도 함. 두 값을 하나의 `struct`로 정의하여, 필드에 의미있는 네이밍을 하는 편이 좋음.

> complex type이 적합한 상황에서 오히려 primitive type 을 사용하는 것은 primitive obsession 이라는 안티 패턴.

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

- `clone()` - `clone`을 통해서 `String` 값을 직접 소유하는 인스턴스를 리턴하도록 함. 참조보다 시간과 메모리 소비는 크지만 참조 수명을 관리할 필요가 없기 때문에 직관적으로 코드 작성이 가능.

> 런타임 비용 때문에 데이터 복제를 통해 `ownership` 문제를 해결하는 것은 피하려는 경향이 있음. 13장에서 효율적으로 관리할 수 있는 방법을 설명. 예제 케이스의 경우 아주 작은 데이터이기 때문에 일단 사용함

#### Creating a Constructor for Config

`parse_config` 함수의 목적은 `Config` 인스턴스 생성. 따라서 `sturct`의 연관 함수인 `new` 함수로 바꿀 수 있음 (보편적인 패턴).

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config {
      let query = args[1].clone();
      let filename = args[2].clone();

      Config { query, filename }
    }
}
```

### Fixing the Error Handling

`args vector`가 3개 이하의 아이템을 가지고 있을 때, 인덱스 참조에 의해서 패닉이 발생.

```sh
$ cargo run
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/chapter12_minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:23:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

#### Improving the Error Message

##### arguments 개수 체크 로직 추가

```rust
// --snip--
fn new(args: &[String]) -> Config {
      if args.len() < 3 {
        panic!("not enough arguments")
      }
      // --snip--
```

result:

```sh
$ cargo run
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/chapter12_minigrep`
thread 'main' panicked at 'not enough arguments', src/main.rs:24:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

지정한 panic 메세지 확인 가능. `panic!`은 프로그램 사용상의 문제와는 적합하지 않으므로 `Result` 타입을 리턴하는게 더 적합함.

#### Returning a `Result` from new Instead of Calling `panic!`

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
        return Err("not enough arguments")
      }

      let query = args[1].clone();
      let filename = args[2].clone();

      Ok(Config { query, filename })
    }
}
```

#### Calling `Config::new` and Handling Errors

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}
```

- `unwrap_or_else` - 표준 라이브러리에서 `Result<T, E>` 타입에 정의한 메소드. `Result`의 값이 `Ok`라면 `unwrap`과 유사하게 동작 (`Ok`에 저장된 값을 리턴). `Err` 값이면 closure를 이용하여 `unwrap_or_else` 메소드에 전달한 anonymous function을 호출. `Err`의 정적 문자열을 파이프 문자(|) 사이에 선언한 `err`에 전달함.
- `process::exit` - `panic!` macro 처리와 유사하지만, 불필요한 정보가 출력되지 않음.

result:

```sh
$ cargo run
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/chapter12_minigrep`
Problem parsing arguments: not enough arguments
```

### Extracting Logic from main

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}
```

#### Returning Errors from the run Function

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);

    Ok(())
}
```

- `Result<(), Box<dyn Error>>` - trait 객체인 `Box<dyn Error>`는 `Error` trait를 구현하는 타입을 리턴은 하지만, 리턴될 값의 타입을 특정하지는 않는다는 의미. 여러 에러 상황에서 다른 타입의 에러들을 리턴할 수 있음
- `read_to_string()?` - `panic!` 대신 `?` operator를 통해서 함수의 에러값을 리턴
- `Ok(())` - 성공적으로 실행될 경우 어떤 값도 리턴하지 않는다는 의미

result:

```sh
$ cargo run
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
warning: unused `Result` that must be used
  --> src/main.rs:16:5
   |
16 |     run(config);
   |     ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `chapter12_minigrep` (bin "chapter12_minigrep") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/chapter12_minigrep`
Problem parsing arguments: not enough arguments
```

- `warning: unused Result that must be use` - `Result` 값이 에러일 수도 있음을 알려줌.

#### Handling Errors Returned from `run` in `main`

```rust
fn main() {
    // --snip--

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    if let Err(e) = run(config) {
      println!("Application error {}", e);

      process::exit(1);
    }
```

- `run` 함수는 `Ok`의 경우 리턴 값이 없으므로 에러에 대한 핸들링만 작성

### Splitting Code into a Library Crate

##### lib.rs

```rust
use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments")
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  println!("With text: \n{}", contents);

  Ok(())
}
```

##### main.rs

```rust
use std::env;
use std::process;

use chapter12_minigrep::Config;
use chapter12_minigrep as minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    if let Err(e) = minigrep::run(config) {
      println!("Application error {}", e);

      process::exit(1);
    }
}
```

## Developing the Library’s Functionality with Test-Driven Development

TDD 순서는 아래와 같음

1. 실패하는 테스트를 작성, 의도한 이유 때문에 실패하는 지 확인
2. 테스트 성공하기에 충분한 코드를 작성 및 수정
3. 리팩토링하면서 테스트가 계속 성공하는지를 확인
4. 1~3단계 반복

### Writing a Failing Test

테스트 작성

```rust
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}
```

기본적인 `search` 함수 구현. 빈 `vector`를 리턴하여 실패하도록 함.

```rust
pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
  vec![]
}
```

만약 함수에 lifetime 관련 annotation을 빼먹고 컴파일 하게되면 아래와 같은 에러가 발생

```sh
$ cargo test
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
error[E0106]: missing lifetime specifier
  --> src/lib.rs:31:53
   |
31 | pub fn search(_query: &str, _contents: &str) -> Vec<&str> {
   |                       ----             ----         ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `_query` or `_contents`
help: consider introducing a named lifetime parameter
   |
31 | pub fn search<'a>(_query: &'a str, _contents: &'a str) -> Vec<&'a str> {
   |              ++++          ++                  ++              ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter12_minigrep` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
```

`search`함수의 리턴값은 `_contents`의 일부이기 때문에 lifetime이 동일해야함. 다시 정상적으로 함수를 작성하면 아래와 같이 에러가 발생.

result:

```sh
$ cargo test
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 0.67s
     Running unittests (target/debug/deps/chapter12_minigrep-c4b15ff4de6182bc)

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----
thread 'test::one_result' panicked at 'assertion failed: `(left == right)`
  left: `["safe, fast, productive."]`,
 right: `[]`', src/lib.rs:48:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### Writing Code to Pass the Test

에러를 토대로 `search`함수가 테스트를 통과하기 위해선 아래 사항들을 수행해야함.

1. `contents`의 각 줄을 순회.
2. 각 줄이 `query`를 포함하고 있는지 확인
3. `query`가 있다면 리턴할 값의 목록에 해당 라인을 추가
4. 없다면 다음 줄로 넘어감.
5. 해당 값을 리턴

#### Iterating Through Lines with the `lines` Method

```rust
pub fn search<'a>(_query: &str, contents: &'a str) -> Vec<&'a str> {
  for line in contents.lines() {
    // do something with line
  }
}
```

#### Searching Each Line for the Query

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  for line in contents.lines() {
    if line.contains(query) {
      // do something with line
    }
  }
}
```

#### Storing Matching Lines

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line)
    }
  }

  results
}
```

위의 사항들을 다 수행하였기 때문에 테스트는 통과하여야함. 이 이후부터는 테스트의 성공을 확인하면서 `search` 함수를 리팩토링하면 됨.

result:

```sh
$ cargo test
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 0.81s
     Running unittests (target/debug/deps/chapter12_minigrep-c4b15ff4de6182bc)

running 1 test
test test::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/chapter12_minigrep-dd97b2179122a411)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests chapter12_minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Using the `search` Function in the `run` Function

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }
  Ok(())
}
```

result:

```sh
$ cargo run frog poem.txt
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.19s
     Running `target/debug/chapter12_minigrep frog poem.txt`
Searching for "frog"
In file "poem.txt"
How public, like a frog

$ cargo run body poem.txt
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/chapter12_minigrep body poem.txt`
Searching for "body"
In file "poem.txt"
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!

$ cargo run monomorphization poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/chapter12_minigrep monomorphization poem.txt`
Searching for "monomorphization"
In file "poem.txt"
```

## Working with Environment Variables

### Writing a Failing Test for the Case-Insensitive `search` Function

insensitive에 대한 실패 테스트 먼저 작성

```rust
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}
```

### Implementing the `search_case_insensitive` Function

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}
```

- `let query = query.to_lowercase();` - `query`를 소문자로 변환하여, `shadowed variable`에 저장
- `to_lowercase` - 기존 데이터의 참조가 아닌 새로운 데이터의 생성. 즉 문자열 슬라이스가 아닌 문자열.
- `line.to_lowercase().contains(&query)` - 검색할 `line` 또한 소문자로 변환.

result:

```sh
$ cargo test
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 0.70s
     Running unittests (target/debug/deps/chapter12_minigrep-c4b15ff4de6182bc)

running 2 tests
test test::case_sensitive ... ok
test test::case_insensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/chapter12_minigrep-dd97b2179122a411)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests chapter12_minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

두 함수를 트리거 시킬 환경 변수를 추가해서 사용할 수 있도록 변경

```rust
pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments")
    }

    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_sensitive })
  }
}
```

- `let case_sensitive = env::var("CASE_INSENSITIVE").is_err();` - `is_err()` 메소드를 통해서 `var`의 리턴 `Result`가 에러인지 아닌지를 확인함.

그 후 `run` 함수에서 해당 config 변수를 이용해 어떤 함수를 실행 시킬지 구현

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }
  Ok(())
}
```

result:

```sh
$ CASE_INSENSITIVE=1 cargo run to poem.txt
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84s
     Running `target/debug/chapter12_minigrep to poem.txt`
Searching for "to"
In file "poem.txt"
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

## Writing Error Messages to Standard Error Instead of Standard Output

현재 구현으로는 `println!` macro를 통해서 모든 메세지를 처리하고 있는데, 대부분의 터미널은 표준 출력 및 표준 에러를 지원하므로 이를 적용할 수 있도록 변경. 표준 출력은 파일에 저장하면서도 에러 메세지는 화면을 통해 확인하도록 변경.

### Checking Where Errors Are Written

먼저 표준 출력 스트림을 파일로 리다이렉트, 에러 스트림은 화면을 통해 확인할 수 있도록 변경.

##### 변경 전 (에러 스트림도 파일로 리다이렉트 됨)

```
$ cargo run > output.txt

// output.txt
Problem parsing arguments: not enough arguments
```

### Printing Errors to Standard Error

```rust
use std::env;
use std::process;

use chapter12_minigrep::Config;
use chapter12_minigrep as minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    if let Err(e) = minigrep::run(config) {
      eprintln!("Application error {}", e);

      process::exit(1);
    }
}
```

- `eprintln!` - 이 매크로를 이용하여, 에러 스트림에 메세지를 출력할 수 있도록 변경

result:

```sh
$ cargo run > output.txt
   Compiling chapter12_minigrep v0.1.0 (/Users/brouk.develop/playground/rust_practice/chapter12_minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/chapter12_minigrep`
Problem parsing arguments: not enough arguments
```

에러 메세지는 터미널에서만 출력이 되고 `output.txt` 에서는 확인 불가

```
$ cargo run to poem.txt > output.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/chapter12_minigrep to poem.txt`

// output.txt
Searching for "to"
In file "poem.txt"
Are you nobody, too?
How dreary to be somebody!
```

검색 결과는 터미널에 노출되지 않고 파일에 기록됨.
