## Error Handling

## Unrecoverable Errors with panic!

`panic!` macro 를 이용하면 실패 메세지를 출력하고 스택을 정리한 뒤 종료

에러 처리할 방법이 마땅치 않을 때 활용

```rust
fn main() {
    panic!("crash and burn") // thread 'main' panicked at 'crash and burn'
}
```

### Using a panic! Backtrace

```rust
fn main() {
    let v = vec![1,2,3];

    v[99]; // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
```

`vector`의 인덱스가 유효하지 않기 때문에 패닉 발생

C 같은 언어에서는 원치 않던 값이라도 개발자가 지정한 위치의 값을 리턴. 따라서 지정한 위치의 메모리가 `vector`가 관리하는 메모리가 아니더라도 해당 위치의 메모리에 저장된 값을 리턴함 `(buffer overread)`

이를 보안하기 위하여 `Rust`에서는 존재하지 않는 인덱스의 값을 읽으려 한다면 종료시킴

해당 에러를 보면 `RUST_BACKTRACE` 환경 변수를 이용하여 에러가 어떻게 발생했는지 역추적할 수 있다는 점을 보여줌

역추적은 그 지점까지 호출된 모든 함수의 목록

```sh
$ RUST_BACKTRACE=1 cargo run

// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:107:14
   2: core::panicking::panic_bounds_check
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:75:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/slice/index.rs:184:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/slice/index.rs:15:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/alloc/src/vec/mod.rs:2528:9
   6: chapter9_error::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

환경 변수를 설정한 후 실행해보면 역추적 정보를 보여줌

해당 정보들을 얻기 위해선 `debug symbols`이 활성화 되어있어야 함. `cargo build` 혹은 `cargo run` 명령을 `--release` 없이 실행하면 기본적으로 활성화가 됨

## Recoverable Errors with Result

대부분의 에러는 종료해야 할 정도로 치명적이진 않음

`Result`는 다음과 같이 `variants`를 정의하고 있음

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T`는 작업이 성공한 경우 `Ok` variants에 포함될 값의 타입, `E`는 작업이 실패한 경우 포함될 값의 타입

위의 제네릭 타입 매개변수를 이용하여 어떠한 상황에서도 활용할 수 있음

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

`File::open` 함수가 `Result` 타입을 리턴하는지 알 수 있는 방법은 `std` library API를 참고하거나 컴파일러를 돌려보면 가능. 변수 `f`에 `Result`가 아닌 타입으로 `annotation`을 추가해보면 에러를 통해 알 수 있음

```rust
use std::fs::File;

fn main() {
    let f: u32 = File::open("hello.txt"); //  expected `u32`, found enum `Result`
    // note: expected type `u32` found enum `Result<File, std::io::Error>`
}
```

에러 메세지를 확인 해보면 `Result` 타입의 제네릭 변수로 어떤 타입이 할당되어 있는지도 확인 가능

`match`표현식을 통해서 `File::open`의 결과를 핸들링할 수 있음

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
}
```

### Matching on Different Errors

앞선 코드는 실패 이유와는 무관하게 무조건 `panic!` macro를 호출. 실패 원인에 따라 다르게 동작하게 할 수 있음

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

`File::open`의 `Err` 타입은 `io::Error`타입. 해당 타입은 `io::ErrorKind` 타입을 리턴하는 `kind` 메소드를 제공. 이를 이용하여 `파일이 없는 경우`에 해당하는 에러 케이스를 따로 핸들링할 수 있음

### Shortcuts for Panic on Error: `unwrap` and `expect`

`match` 표현식은 코드가 길어지고 의도를 항상 정확하게 표현하지는 못함

`unwrap` 메소드는 `match`표현식과 정확히 같은 동작을 하는 `shortcut` 메소드

`Result`타입의 값이 `Err variant` 라면 `unwrap`은 `panic!` macro를 호출함

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap(); // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:4:37
}
```

`expect` 메소드는 `unwrap`과 유사하지만 `panic! macro`에 에러메세지를 전달할 수 있음

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // thread 'main' panicked at 'Failed to open hello.txt: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:4:37
}
```

따라서 개발자의 의도를 더 명확하게 표현함과 동시에 원인을 쉽게 추적할 수 있음

### Propagating Errors

함수 안에서 발생한 에러는 호출하는 코드에 에러를 리턴하여 호출자가 에러를 처리하게 할 수 있음 `(propagating)`

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 에러가 발생할 경우 함수를 중단하고 에러값을 호출자에게 리턴함
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // 성공할 경우 파일의 내용을 s에 기록하여 리턴
        Err(e) => Err(e) // 실패할 경우 에러 리턴
    }
}
```

이 코드를 호출한 부분에서는 `Ok`, `io::Error`를 처리하면 됨. 위 `propagating`은 `? operator`를 이용하여 더 쉽게 처리 가능

#### A Shortcut for Propagating Errors: the ? Operator

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`Result` 값 뒤에 덫붙이는 `? operator`는 `match` 표현식과 거의 같은 방식의 동작

`Result`값이 `Ok`이 인경우 해당 값이 리턴되며 프로그램이 계속 실행, `Err`라면 해당 값이 전체 함수의 리턴값이 되어 호출자로 전파함

`? operator`의 경우 에러값은 `from` 함수를 이용하여 전달. 이 함수는 `std library`에 정의된 `from trait`에 선언되어 있으며, 에러를 어떤 타입으로부터 다른 타입으로 변환.

`? operator`가 `from`함수를 호출하면 전달된 에러 타입은 현재 함수의 리턴 타입에 정의된 에러 타입으로 변환됨.
다양한 원인에 의해 실패하더라도 함수의 실패 원인을 한 가지 에러 타입으로 리턴하는 경우에 유용함.

`? operator` 다음에 메소드를 연결해서 호출할 수도 있음

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

`fs`의 다른 메소드를 이용하면 더더욱 짧게 함수 작성이 가능

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### Where The ? Operator Can Be Used

`? operator`는 `Return` 타입을 리턴하는 함수에 대해서만 사용 가능. `match` 표현식과 같은 동작을 실행하도록 정의되어 있는데, 해당 표현식에서 `return Err(e)`를 실행하는 코드 때문에 반드시 `Result` 타입이어야 함

아래 예시는 그렇지 않은 경우에 사용할 경우

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?; //  the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
}
```

`main`함수는 특별한 함수이기 때문에 리턴할 수 있는 값의 타입에 제한이 있음

그 중 하나는 `()`이며, 아래와 같이 `Result<T, E>`를 리턴할 수 있음

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

`Box<dyn Error>` 타입은 `trait object`. 후에 서술할 `Using Trait Objects that Allow for Values of Different Types` 에서 설명하기로 하고, `any kind of error`의 의미로 알아두면 됨

`main`함수가 위의 타입정의를 사용하면 `? operator`를 사용 가능함

#### 공식 docs 추가 예제

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

위의 에러 메세지를 보면 `?` operator는 `Option` 타입에서도 사용이 가능함
(`FromResidual`을 implement한 타입도 포함)

`Result`와 마찬가지로 `Option`을 리턴하는 메소드의 `?` operator는 `Option` 함수를 리턴하는 함수 내에서 사용가능함. 위 예제의 경우 `char`가 있을수도 있는 `Some`과 없는 `None`이 리턴됨. 나머지는 `Result`와 동일함

## To panic! or Not to panic!

실패할 가능성이 있는 함수의 경우 `Result` 타입을 권유. 해당 에러를 호출자에게 넘겨 호출자가 적절한 상황에 맞춰서 회복을 시도하던지, 회복불가능한 에러로 처리하던지 할 수 있게끔 하기 위함

하지만 가끔은 `Result` 타입보다 패닉을 발생시키는 것이 적절할 때도 있음

### Examples, Prototype Code, and Tests

`Examples`에서 패닉을 발생시킬 수 있는 `unwrap` 같은 메소드를 호출하는 부분은 상황에 따라 처리해야할 에러가 발생할 수도 있다는 것을 표현하는 것으로 받아들여지고 있음

`unwrap`과 `expect` 메소드는 실제 에러를 어떻게 처리할 것인지에 앞서 `prototyping`에 매우 유용함

`Tests` 중에 메소드 호출이 실패한다면, 실제 테스트 메소드가 아니더라고 전체 테스트를 실패처리해야함

### Cases in Which You Have More Information Than the Compiler

`Result` 타입을 리턴하는 로직의 결과가 `Ok`가 확실하더라도 컴파일러가 이해할 수 없는 로직이라면, `unwrap`과 함께 호출해주는 것이 좋음

`Result` 값을 확보하는 데에는 아무 문제가 없으며, 특정 상황에서는 실패할 수 있는 가능성이 있기 때문

```rust
use std::net::IpAddr;
let home: IpAddr = "127.0.01".parse().unwrap();
```

위의 예제에서 `127.0.0.1`은 유효한 주소이기 때문에 파싱이 실패할 일은 없겠지만 그래도 `unwrap`을 호출할 수 있음. 컴파일러 관점에서는 해당 문자열이 항상 유효하다는 것을 모르기 때문

하드 코딩이 아닌 유저의 input을 사용하는 값이라면 더욱 더 견고하게 `Result`값을 모두 처리해야함

### Guidelines for Error Handling

`bad state`가 될 상황이라면 패닉을 발생시키는 것도 가능

`bad state`란 유효하지 않은 값, 모순된 값, 실수로 놓친 값들이 코드로 전달되는 상황과 더불어 아래 상황들을 포함

- 원래 기대했던 동작이 어쩌다 실패하는 상황이 아님
- 어느 지점 이후의 코드는 `bad state`에 놓이지 않아야만 정상적으로 동작
- 이 정보를 사용중인 타입으로 표현할 방법이 없음

적절하지 않은 값을 전달하면 `panic!` macro를 호출하여 라이브러리를 사용하는 개발자의 코드에 버그가 있음을 알릴 수 있음. 마찬가지로 개발자가 제어할 수 없는 외부 코드를 호출하였는데, 그 코드가 수정할 방법이 없는 잘못된 상태를 리턴하는 경우에도 유용하게 사용 가능

`paser`에 잘못된 형식의 데이터가 오거나, HTTP 요청이 호출 횟수 제한으로 인해 리턴하는 경우 등은 `Result` 타입이 나을 수 있음. 실패할 가능성이 있음을 명확히 하고 이 `bad state`를 호출자에게 전달하여 호출자가 어떻게 해결할 지 결정하게 하는 것이 좋음

유효하지 않은 데이터를 기반으로 작업을 실행하면 코드가 취약점에 노출 될 수 있음. 패닉을 발생시키는 편이 좋음.
표준 라이브러리가 유효하지 않은 범위의 메모리에 접근할 때 `panic!` 매크로를 호출하는 이유도 바로 이런 이유 때문.

입력이 특정 요구 사항을 만족 할 때만 그 동작을 보장하는 `contract`를 가진 함수도 있음. 이 케이스에도 호출자가 명시적으로 처리하도록 도와야할 종류의 에러는 아니기 때문에 패닉을 발생시키는 편이 좋음. 추가적으로 함수에 관하여 API 명세를 반드시 해주어야 함

함수의 유효한 값의 전달은 컴파일러가 보장한다는 것을 알고 있기 때문에 별도로 처리할 필요가 없음. `Option`타입이 아닌 명확한 타입을 매개변수로 정의한다면, 이 함수에 존재하지 않는 값을 전달하는 코드는 컴파일조차 되지 않기 때문에 굳이 런타임에 값의 타입을 검사하지 않아도 됨. 추가적인 예로 `u32` 같은 정수타입의 경우에도 매개변수에 음의 정수가 전달될 일은 없음.

### Creating Custom Types for Validation

```rust
loop {
        // --snip--

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
```

이 코드의 `if` 구문은 입력된 값이 범위를 벗어나는지 체크하고, 문제가 있다면 안내를 한 뒤 `continue`를 통하여 다시 값을 입력하도록 함. 따라서 `guess.cmp(&secret_number)` 에는 반드시 1~100사이의 정수가 비교됨을 알 수 있음. 그러나 이 조건에 만족해야할 함수가 많아진다면 `if`문을 통하여 매번검사하는 것은 비효율적

새로운 타입을 생성하고, 이 타입의 인스턴스를 생성하는 함수에 유효성 검사 코드를 작성하면 반복할 필요가 없어짐.
또한 이 타입을 이용하여 시그니처를 정의한다면 안전하게 원하는 값을 전달 받을 수 있음

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

`Guess` 타입을 생성하는 `new`를 구현. `new` 함수는 `i32` 타입의 매개변수를 통하여 `Guess` 타입을 리턴함.
또한 함수 내부에서 1~100까지의 정수인지를 검사함.

`Guess`의 `value`는 비공개 필드이기 때문에 `value`라는 `getter`함수를 구현. `value`가 비공개여야 하는 이유는 `Guess`를 호출하는 코드가 `value`값을 조작해선 안되기 때문.

이 두가지를 통하여 `Guess`를 호출하는 코드는 1~100사이의 값을 보장함과 동시에 조작을 할 수 없음
