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

문자열은 일종의 collection

`collection of bytes`이면서 해당 byte를 문자열로 처리할 때 유용한 메소드를 제공

### What is a string?

`Rust`는 오직 string slice인 `str`타입만을 지원

string slice는 어딘가에 `UTF-8`으로 인코딩 되어 저장된 문자열에 대한 참조

`String Literal`은 프로그램을 컴파일한 바이너리 결과에 포함되므로 string slice

`String`타입은 `Rust`의 standard library가 지원하는 타입. `UTF-8`형식으로 인코딩된 문자열 타입

### Createing a new string

`String`타입은 `Vec<T>`가 지원하는 대부분의 operation을 지원

```rust
    let mut s = String::new();
```

`String Literal`과 마찬가지로 `Display crate`를 구현한 모든 타입에서 사용할 수 있는 `to_string` 메소드를 활용할 수도 있음

```rust
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly
    let s = "initial contents".to_string();
```

마찬가지로 `String::from`을 사용해도 됨

```rust
    let s = String::from("initial contents");
```

`UTF-8`을 사용하기 때문에 어떤 데이터라도 적절하게 인코딩 되었다면 사용 가능

```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
```

### Updating a String

`String`은 `Vec<T>`타입과 마찬가지로 data를 계속 push하면 크기와 저장된 데이터가 변경 될 수 있음. 또한 `format!`macro 혹은 `+` operator를 이용하여 연결 가능

1. Appending to a String with `push_str` and `push`

```rust
    let mut s = String::from("foo");
    s.push_str("bar");
```

위 코드의 결과로 변수 s는 `foobar`가 됨

`push_str` 메소드가 string slice를 이용하는 이유는 매개변수의 ownership을 가질 필요가 없기 때문

```rust
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {}", s2);
```

만약 `push_str` 메소드가 `s2`에 대한 onwership을 갖게 되면 `println!("s2: {}", s2);`에서 에러가 발생하게 될 것

`push`메소드는 single character를 매개 변수로 받아서 `String`에 추가

```rust
    let mut s = String::from("lo");
    s.push('l'); // single quote 사용해야함
```

2. Concatenation with the `+` Operator or the `format!` macro

```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

`+` operator는 `add` 메소드를 사용하는데, 이 메소드의 signature 때문에 s1을 사용할 수 없으며 s2는 참조를 전달해야 함

```rust
    // + operator
    fn add(self, s: &str) -> String {
```

standard library의 signature와 정확히 일치하진 않고, standard library에서는 제네릭을 이용하고 있음

변수 s2는 `&`를 사용하고 있으므로, `s2`의 참조를 첫번째 `String`에 추가하는 것과 같음. 매개변수 `s`는 참조 형식이기 때문에 `String`에는 오직 `&str` 타입의 값만 추가할 수 있으며, 두 개의 `String`을 결합할 수는 없음

`add` 메소드의 매개변수로 `&s2`를 전달할 수 있는 이유는 컴파일러가 `&String` 인수를 `&str`로 알아서 변환함. 메소드가 호출되면 `Rust`는 `deref coercion`을 사용하여 `&s2`를 `&s2[..]`로 변환함

`add` 메소드는 매개변수 `s`의 ownership이 없기 때문에 `s2`는 문자열을 추가하는 작업 이후에도 유효함

`add` 메소드는 첫 번째 매개변수 `self`의 ownership이 있음. 따라서 변수 `s1`은 메소드의 scope로 이동하기 때문에 그 이후 유효하지 않음.

정리하면 `add` 메소드는 첫 번째 매개변수의 ownership을 받아 두 번째 매개변수의 값을 복사하여 덧붙인 후 ownership을 다시 돌려주는 것

단 여러 개의 문자열을 결합할 때는 `+` operator는 적합하지 않음

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s1);
```

`format!` macro는 `println!`과 같은 방식으로 동작, `String`을 리턴함 또한 매개변수의 ownership도 갖지 않음

### Indexing into String

`Rust`에서는 `String`에 접근할 때 인덱스를 이용하면 에러가 발생

```rust
    let s1 = String::from("hello");
    let h = s1[0]; // `String` cannot be indexed by `{integer}`
```

1. Internal Representation

`String`은 `Vec<u8>`을 감싼 타입

```rust
    let len = String::from("Hola").len();
```

위 코드의 `len`은 4. `Vector`에 저장된 문자열은 길이가 `4byte`

```rust
    let len = String::from("안녕하세요").len();
```

위 코드의 `len`은 15. `안녕하세요`라는 값을 `UTF-8`로 인코딩하면 15byte를 사용. 이는 유니코드의 스칼라값은 3byte를 차지하기 때문

```rust
    let hello = String::from("안녕하세요");
    let answer = &hello[0];
```

`answer`의 값은 236. `안`의 첫 번째 바이트는 236이고, 두 번째 바이트는 149 따라서 `answer`의 값은 236

예상치 못한 값을 리턴하기 때문에 이를 방지하기 위해 `Rust`에서 차단함

2. Bytes and Scalar valeus and Grapheme cluseters

`Rust`의 관점에서 `String`은 크게 `bytes`, `scalar values`, `grapheme cluster` 세 가지로 구분

`안녕하세요`는 아래와 같은 `u8` 값들의 `Vector`에 저장됨

`[236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148]`

총 15개의 바이트값이 최종적으로 저장되는 형태

이 값을 `Rust`의 `char`타입인 `Unicode scalar`로 표현하면 아래와 같음

`['안', '녕', '하', '세', '요']`

이 `Vector`에는 다섯 개의 `char`값이 저장됨. 해당 데이터를 `grapheme cluster`로 표현하면 아래와 같음

`["안", "녕", "하", "세", "요"]`

`String` 타입에서 인덱스 사용 지원을 하지 않는 마지막 이유는 인덱스 처리에는 `O(1)` 이 소요되어야 하지만 이에 대한 성능 보장을 할 수 없기 때문.
유효한 문자 파악을 위해 처음부터 스캔해야하기 때문

### Slicing Strings

따라서 정말로 인덱스를 이용하여 string slice를 생성하기 원한다면 좀 더 정확하게 조건을 명시해야 함

`[]`기호에 하나의 숫자를 인덱스로써 전달하지 말고 특정 바이트의 범위를 지정해야함

```rust
    let hello = String::from("안녕하세요");
    let s = &hello[0..3];
```

변수 s는 문자열의 처음 3byte 값을 저장하는 `&str`타입. 한 글자는 3bytes이기 때문에 s는 `안`이 됨

`&hello[0..1]`과 같이 표현한다면 인덱스와 마찬가지로 runtime panic 발생

`` thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside '안' (bytes 0..3) of `안녕하세요`' ``

### Methods for iterating over strings

개별 유니코드 스칼라 값이 필요하다면 가장 좋은 방법인 `chars` 메소드를 이용

```rust
    let a = "안녕하세요".chars();

    println!("{:?}", a); // Chars(['안', '녕', '하', '세', '요'])

    for c in "안녕하세요".chars() {
        println!("{}", c);
    }
```

`bytes` 메소드는 마찬가지로 각 문자의 바이트를 리턴할 필요가 있을 때 활용

```rust
    for c in "안녕하세요".bytes() {
        println!("{}", c);
    }
```

유효한 유니코드 스칼라 값은 1byte보다 큰 값

`String`에서 `grapheme cluster`를 가져오는 것은 복잡해서 지원 안함. 필요하다면 https://crates.io 에서 찾아서 사용

### String are not so simple

`Rust`가 string에 대하여 채택한 방식의 단점은 조금 더 복잡하다는 점이지만, 개발 시점에서 `ASCII`문자가 아닌 다른 형식 문자를 다룰 때 에러를 처리해야 할 필요가 없다는 장점도 있음

## Storing keys with associated values in Hash Maps

`HashMap<K, V>` 타입은 `K` 타입의 키에 `V`타입의 값을 mapping. 이 때 키와 값을 어떻게 저장할 지 결정하기 위하여 `hashing function` 사용
. 이 data structure는 각각의 언어에서 `hash table`, `dictionary`, `associative array`등 으로 불림

`HashMap`은 `Vector`처럼 인덱스를 이용하는 것이 아닌 키를 이용하여 데이터를 조회하고자 할 때 유용

### Creating a new Hash Map

`new`함수를 사용하여 빈 `HashMap`생성 가능. `insert`를 이용하여 새로운 키와 값 추가

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

`Vector`와 마찬가지로 데이터는 `heap memory`에 저장. 또한 모든 키와 모든 값의 타입은 같아야함

키와 값을 가진 `vector of tuples`의 `collect` 메소드를 통해서 생성할 수도 있음

팀 이름을 가진 `Vector`와 점수를 가진 `Vector` 두 개를 `zip` 메소드를 이용하여 `vector of tuples`로 구성. 그 후에 `collect` 메소드를 통하여 해시 맵으로 변환

```rust
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

`collect` 메소드는 여러가지 data structure 생성 가능. 따라서 `HashMap<_, _>` 과 같이 `type annotation`이 필요. key, value에 대한 타입 매개변수는 `underscore(_)` 사용 가능

### Hash Maps and Ownership

`i32`와 같이 `copy trait`를 구현한 타입들은 값들이 `Hash Map`으로 복사 된다.

`String`과 같이 값을 소유하는 타입은 `Hash Map`이 해당 값들의 Ownership을 갖게 됨

```rust
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // try using them and see what compiler error you get

    // println!("{:?}", field_name); value borrowed here after move
```

`Hash Map`에 참조를 추가하면 그 값은 이동하지 않음. 다만 `Hash Map`이 유효한 범위에 있는 동안은 함께 유효해야함

### Accessing values in a Hash Map

값에 접근할 때는 `get` 메소드 활용

```rust
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
```

`score` 변수의 타입은 `Some(&10)`. `get` 메소드는 `Option<&V>`를 리턴하기 때문

`for loop`을 이용하면 key, value 쌍을 순회 할 수 있음

```rust
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

### Updating a Hash Map

각 키에는 오직 하나의 값만 할당 가능. 따라서 이미 값이 있다면 교체할 지, 기존 값을 무시할 지, 새 값을 무시 할 지, 기존 값이 없을 때만 추가할 지 정해야 함.
결합도 가능함

1. Overwriting a value

```rust
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores) // {"Blue": 25}
```

2. Only inserting a value if the key has no value

```rust
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores) // {"Yellow": 50, "Blue": 10}
```

`or_insert`의 리턴은 가변 참조

3. Updating a value based on the old value

```rust
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
```

`or_insert`로부터 가변 참조를 받고, `asterisk`를 이용하여 가변 참조 변수를 `deref`하여 새 값을 할당 시킴. 해당 가변 참조는 `for loop`가 종료되면 `scope`밖으로 나가기 때문에 `borrowing rules` 위반도 아님

### Hashing Functions

`Hash Map`은 암호학적으로 강력한 `hashing functions`를 이용하여 `Dos(Denial of Service)` 공격 방지 가능. 가장 빠른 `hashing algorithm`을 사용하진 않지만, 그보다 보안 향상을 시키는 것이 필요하기 때문. 코드의 성능을 보고 기본 `hashing functions`이 너무 느리다면 다른 `hasher`를 사용. `hasher`란 `BuildHasher trait`를 구현하는 타입
