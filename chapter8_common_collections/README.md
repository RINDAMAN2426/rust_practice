# Common Collections

- `vector` 연속된 일련의 값
- `string` collection of characters
- `hash map` 특별한 키와 값을 연결. 좀 더 범용적인 데이터 구조인 `map`을 구현.

`collections`은 https://doc.rust-lang.org/std/collections/index.html를 참조

## Storing Lists of Values with Vectors

`vector`는 `Vec<T>`의 형태

하나 이상의 값을 하나의 `data structure`에 저장 가능, 모든 값은 메모리에서 연속으로 저장

오로지 같은 타입의 값만 저장 가능

### Creating a New Vector

```rust
    let v: Vec<i32> = Vec:new();
```

`Vect<T>`는 제네릭을 이용해 어느 타입이든 저장할 수 있고, 해당 타입을 `angle brackets`에 명시해야함

일반적으로 `Ruts`에서는 `Vector`에 값을 추가하면 타입을 유추할 수 있으므로 `type annotation`이 별로 필요 없음

편의를 위해 `vec!`매크로를 제공하며, 지정한 값을 저장하는 새로운 `Vector`를 생성

```rust
    let v = vec![1,2,3];
```

### Updating Vector

수정을 위해선 `mut`을 사용하고, `push` 메소드를 이용

```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

### Dropping a Vector drops its elements

다른 `struct`와 동일하게 `scope`에서 벗어날 시 `drop` 메소드가 호출됨

```rust
    {
        let v= vec![1,2,3,4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```

`Vector`가 메모리에서 해제되면 지정된 모든 값들도 함께 해제. 이 때 참조형 값을 저장하는 경우에는 복잡해짐

### Reading Elements of Vectors

아래 예제는 `indexing systax`와 `get` 메소드를 통해 저장된 값에 접근하는 예제

```rust
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third elements."),
    }
```

`Vector`의 인덱스는 0부터 시작

`&`와 `[]`를 이용하여 저장된 값에 대한 참조를 리턴 혹은 `get`메소드를 이용하여 `Option<&T>` 타입의 값을 리턴

```rust
    let v = vec![1,2,3,4,5];

    let does_not_exists = &v[100];
    let does_not_exists = v.get(100);
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
```

`[]` 메소드는 `panic`이 발생, 지정한 참조가 존재하지 않는 값을 가리키기 때문
-> 이를 이용해 `Vector`가 존재하지 않는 값에 대하여 접근 시 충돌을 일으켜 강제 종료를 유도할 수 있음

`get` 메소드는 `Some` 혹은 `None`이 리턴 -> 따라서 `Vector`가 존재하지 않는 값에 대하여 접근하려는 시도가 빈번할 때 유용함, 사용자의 input에 대한 결과를 판단하여 다른 올바른 `Vector`의 값에 접근하도록 유도할 수 있음

유효한 참조값을 얻게 된다면 `Vector`에 저장된 값에 대한 참조가 계속해서 유효할 수 있도록 `borrow checker`가 실행되어 `ownership`과 `borrow rules`를 적용

아래 코드는 `Vector`에 저장된 첫 번째 값에 대한 불변 참조를 가지면서, `Vector`에 새로운 값을 추가하려 하므로 에러가 발생하게 됨

```rust
    let mut v = vec![1,2,3,4,5];

    let first = &v[0]; // immutable borrow occurs here

    v.push(6); // mutable borrow occurs here

    println!("The first element is {}", first); // immutable borrow laster used here

    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
```

위 에러가 발생하는 이유는 현재 `Vector`의 크기가 충분히 크지 않다면, `Vector`의 마지막에 새로운 값을 추가하기 위해 새 메모리를 할당하고 이미 저장된 값들을 새로운 메모리로 옮겨야 할 수도 있기 때문. 이 때 `first` 변수에 저장된 참조는 메모리로부터 해제되며 `borrow rules`에 의해 프로그램은 이런 상황으로부터 보호되어야하기 때문.

### Iterating over the Values in a Vector

아래 예시는 `for loop`을 이용하여 `Vector`에 저장된 값을 출력하는 예시

```rust
    let v = vec![1,2,3,4,5];
    for i in &v {
        println!("{}", i)
    }
```

`mutable vector`에서 가변 참조를 얻어와 값을 변경할 수도 있음

```rust
    let mut v = vec![1,2,3,4,5];
    for i in &mut v {
        *i += 50;
    }
```

`mutable reference`가 가리키는 값을 변경하려면 `dereference operator (*)`를 이용해서 변수에 저장된 값을 가져와야함

### Using an Enum to Store Multiple Types

`Vector`는 오로지 같은 타입의 값만 저장할 수 있기 때문에 불편함이 생길 수 있음
-> `enum` 타입의 경우 `variants`는 같은 `enum` 타입이므로 이를 이용할 수 있음

```rust
    enum SpreadsheetCall {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCall::Int(3),
        SpreadsheetCall::Text(String::from("blue")),
        SpreadsheetCall::Float(10.12)
    ];
```

`Rust`는 컴파일 시점에 `Vector`에 어떤 타입의 값이 저장될지 알아야 하기 때문에 각각의 값을 저장하기 위해 어느 정도의 힙 메모리가 필요한지도 정확히 판단

만일 `Vector`가 어떤 타입이든 저장할 수 있다면, 저장된 값에 대한 연산을 수행할 때 타입의 차이로 인한 에러가 발생할 수 있음

`enum`과 `match`를 조합하면 러스트가 컴파일 시점에 모든 경우의 수에 대한 처리를 보장

`Vector`에 저장해야 할 타입들을 완벽히 알 수 없다면 `enum`은 큰 도움이 되지 못함 -> 이 경우 `trait object`를 이용하면 됨

## Storing UTF-8 Encoded Text with Strings
