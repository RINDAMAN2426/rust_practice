# 소유권

모든 프로그램은 실행 중인 동안 메모리를 관리해야 한다. GC를 통하여 해제하는 언어도 있고, 명시적으로 할당 해제를 하는 언어도 있다.
`러스트는 컴파일러가 컴파일 시점에 검사하는 다양한 규칙으로 이루어지는 소유권 시스템으로 관리한다. 따라서 소유권에 관한 기능은 실행 성능에 아무런 영향을 미치지 않는다.`

## Stack, Heap

### Stack

- LIFO
- Stack의 데이터는 고정된 크기를 가져야한다.(런타임에 동적으로 변하는 데이터는 힙메모리에 저장한다.)

### Heap

- 힙 메모리에서 일정한 공간을 찾아 사용 중인 메모리로 표시한 후, 포인터를 넘긴다. (allocating on the heap)

스택에 데이터를 푸시하는 것이 힙보다 빠른 이유는 저장할 공간을 찾을 필요가 없기 때문.

힙 메모리에 저장된 데이터에 대한 접근 또한 스택 보다 느리다.

코드에서 함수를 호출할 때 함수에는 여러 값이 전달되며, 함수의 로컬 변수에 할당되어 스택에 저장된다. 실행이 완료 되면 이 값들은 스택에서 제거 된다.

힙에 저장되는 데이터의 중복을 최소하해 사용하지 않는 데이터를 제거하면 메모리 부족 문제를 해결할 수 있는데, 러스트의 소유권은 바로 이 문제를 해결하려 하는 방법이다.

## 소유권 규칙

- 러스트가 다루는 각각의 값은 owner라고 부르는 변수를 갖고 있다.
- 특정 시점에 값의 owner는 단 하나뿐이다.
- owner가 범위를 벗어나면 그 값은 제거된다.

## 변수 범위

```rust
{
  let s = "hello"; // s는 이 시점부터 유효하다.
} // block을 벗어나면 s는 유효하지 않다.
```

## String 타입

앞서 설명된 타입들은 스택에 저장되며 범위를 벗어나면 제거되지만, 힙에 저장되는 데이터를 이용하여 제거되는 시점을 알아야한다.

```rust
let mut s= String::from("hello");

s.push_str(", world!");

println!("{}", s);
```

## 메모리와 할당

- 문자열 리터럴은 컴파일 시점에서 알고 있기 때문에 효율적이지만 불변이다.
- 런타임에 길이가 변경되는 문자열은 그 문자열이 사용할 메모리를 바이너리 형태로 미리 변환할 수가 없다.
- 런타임에 운영체제에 메모리를 요청해야하며, 사용이 완료된 후에는 운영체제에 돌려줄 방법이 필요하다.

운영체제에 메모리 요청은 `String::from` 함수를 호출하여 이 함수가 필요한 메모리를 요청한다.

GC가 있는 언어의 경우에는 GC가 사용하지 않는 메모리를 추적해 해제하므로 개발자가 처리할 필요가 없으나, GC가 없는 언어의 경우에는 사용하지 않는 시점을 인지하여 직접 메모리를 돌려주어야한다.
정확한 한번의 `allocate`와 `free`가 이루어져야한다.

러스트의 경우는 할당과 해제를 다른 방식으로 수행한다.
`변수에 할당된 메모리는 변수를 소유한 범위를 벗어나는 순간 자동으로 해제한다.`

위의 코드에서 닫는 중괄호를 만난다면 (이는 C++의 RAII와 유사한 부분이다.)

### 변수와 데이터가 상호작용하는 방식: Move

```rust
let x = 5;
let y - x;
```

이 코드는 정수를 할당하기 때문에 5라는 값 두 개가 스택에 저장된다.

```rust
let s1 = String:from("hello");
let s2 = s1;
```

`String` 타입의 `s1`은 메모리에 대한 `pointer`, `length`, `capacity` 로 구성된다. 이 데이터는 스택에 저장된다. `length`는 현재 어느정도의 메모리를 사용중인지 바이트 단위로 표현한 값이며, `capacity`는 운영체제로부터 할당받은 총 메모리를 바이트로 표현한 값이다.

`s1`을 `s2`에 대입하면 `String`타입의 데이터가 복사된다. 즉 포인터가 가리키는 힙 메모리의 실제 데이터가 아닌, `pointer`, `length`, `capacity`가 스택에 복사된다. 이로 인하여 `drop`함수를 통하여 메모리 해제를 하려할 때, 두 개의 `pointer`가 같은 위치를 가리키고 있음에 `double free error`가 발생하게 된다. 메모리를 두 번 해제하는 것은 `corruption`을 일으키며 보안상의 취약점이 될 수 있다.

러스트는 위의 경우 `s1`은 더 이상 유효하지 않다고 판단하기 때문에 `s1`이 범위를 벗어날 때 메모리를 해제하지 않는다. 다른 언어의 얕은 복사와는 달리, 러스트의 경우 첫째 변수를 무효화해버리기 때문에 이를 `move`라고 한다.

### 변수와 데이터가 상호작용하는 방식: Clone

Stack 데이터가 아닌 Heap에 저장된 `String`데이터를 복사하기를 원한다면 `clone`이라는 메서드를 사용해야 한다.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

복사하려는 메모리의 크기에 따라 무거운 작업이 될 수도 있다. 이 메서드를 호출하는 부분은 뭔가 다른 작업이 수행된다는 것을 시각적으로 확인할 수 있는 부분이라고 할 수 있다.

### 스택 전용 데이터: Copy

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

위의 코드에서 변수 x는 여전히 유효하며, 변수 y로 이동하지도 않는다. 정수형 같은 타입은 컴파일 시점에 이미 그 크기를 알 수 있으며, 온전히 스택에 저장되기 때문에 실제 값을 복사해도 전혀 부담되지 않는다. 무효화할 이유가 전혀 없다.

러스트는 스택에 저장되는 정수형 같은 타입에 대하여 `Copy trait`라는 특별한 특성을 제공한다. `Copy trait`가 적용되어 있다면 이전 변수를 새 변수에 할당해도 무효화되지 않는다. 다만 타입의 일부에 `Drop trait`가 적용되어 있다면 `Copy trait`를 적용할 수 없다. 만일 범위를 벗어나는 시점에 뭔가 특별한 처리가 필요한 타입에 `Copy trait`를 적용하려 하면 컴파일 에러가 발생한다.

`Copy trait`가 적용된 타입에 대해선 통상적으로 단순한 스칼라 값이 있다. 할당이 필요하거나 특정 형태의 자원에도 `Copy trait`가 적용되어 있다.

- u32와 같은 모든 정수형 타입
- ture, false만을 갖는 bool
- char
- f64와 같은 부동 소수점 타입
- `Copy trait`가 적용된 타입을 포함하는 튜플. `(i32, i32)` 튜플은 적용이 되지만 `(i32, String)`에는 적용이 되지 않는다.

## 소유권과 함수

값을 함수에 전달한다는 것은 값을 변수에 대입하는 것과 유사하다. 변수를 함수에 전달하면 대입과 마찬가지로 `move`혹은 `copy`가 이루어진다.

```rust
fn main() {
  let s = String::from("hello"); // 변수 s가 main 스코프 내에 생성된다.

  takes_ownership(s); // s의 값은 함수 내로 이동하고 이 시점부터 변수 s는 더이상 유효 하지 않다.

  let x = 5; // 변수 x가 범위 내에 생성된다.

  makes_copy(x); // 변수 x의 값이 함수 내로 이동한다.
                 // i32값은 copy를 수행하므로 이 시점 이후로도 여전히 휴효하다.
} // 이 시점에서 변수 x가 스코프를 벗어난 후, 변수 s도 범위를 벗어나지만 변수 s의 값은 함수 내로 이동했기 때문에 아무런 일도 일어나지 않는다.

fn takes_ownership(some_string: String) { // some_string 변수가 범위 내에 생성된다.
  println!("{}", some_string);
} // 이 시점에서 some_string 변수가 범위를 벗어나며 'drop'이 호출되고 some_string의 메모리가 해제된다.

fn makes_copy(some_integer: i32) { // some_integer 변수가 범위 내에 생성된다.
  println!("{}", some_integer);
} // 이 시점에서 some_integer 변수가 범위를 벗어나지만 아무런 일도 일어나지 않는다.
```

## 리턴 값과 범위

리턴값도 소유권을 이전한다.

```rust
fn main() {
  let s1 = gives_ownership(); // gives_ownership 의 리턴값이 변수 s1으로 옮겨진다.

  let s2 = String::from("hello"); // 변수 s2가 범위 내 생성된다.

  let s3 = takes_and_gives_back(s2); // 변수 s2는 taks_and_gives_back으로 옮겨간 후 리턴값은 s3으로 옮겨진다.
} // 이 시점에서 변수 s1, s3는 drop함수가 호출된다.
  // 변수 s2또한 스코프를 벗어나지만 takes_and_gives_back로 옮겨갔기 때문에 아무런 일도 일어나지 않는다.

fn gives_ownership() -> String {          // gives_ownership 함수의 리턴값은 호출한 함수로 옮겨진다.
  let some_string = String::from("hello");// some_string이 스코프 내에 생성된다.

  some_string                             // 리턴되고 나면 호출한 함수로 옮겨진다.
}

fn takes_and_gives_back(a_string: String) -> String { // String을 전달받아 다시 리턴한다.
  a_string                                            // a_string이 스코프내에 생성된다.
}                                                     // 리턴하고 나면 호출한 함수로 옮겨진다.
```

값을 다른 변수에 할당하면 소유권은 옮겨진다. 힙 메모리에 저장된 변수의 데이터는 소유권이 다른 변수로 옮겨지지 않았다면 스코프를 벗어날 때 `drop`함수에 의해 제거된다.

```rust
// 해당 변수를 다시 사용하기 위해선 매번 다시 리턴을 해야하는데.. 이런 코드는 짜증날 것이다.
fn main() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("'{}'의 길이는 {}입니다.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}
```

# 참조와 대여

위의 함수는 아래와 같이 개선할 수 있다.

```rust
fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("'{}'의 길이는 {}입니다.", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
```

`&(ampersands)`를 이용하여 소유권을 가져오지 않고도 값을 참조할 수 있다.

`&s1`은 소유권은 가져오지 않고 값을 읽을 수 있는 참조를 생성할 수 있다. 참조는 소유권을 갖지 않기 때문에 참조가 범위를 벗어나더라도 `drop`이 호출되지 않는다. 이와 같이 참조를 전달하는 것을 `borrowing`이라고 한다. 참조는 불변이기 때문에 변경할 수 없다.

## 가변 참조

```rust
fn main() {
  let mut s = String::from("hello");

  change(&mut s);

  println!("{}", s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
```

`&mut s`와 같이 가변 참조를 생성한 후, 함수가 가변 참조를 전달받으면 수정을 할 수 있다. 단

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

특정 스코프내에서 가변 참조는 오직 한개만 존재하여야 하고, 위의 코드는 에러가 발생한다. 이 제약 덕분에 `data races`를 컴파일 시점에 방지할 수 있다. `data races`는 `race condition`과 유사하며 주로 아래 이유들로 인해 발생한다.

- 둘 혹은 그 이상의 포인터가 동시에 같은 데이터를 읽거나 쓰기 위해 접근
- 최소한 하나의 포인터가 데이터를 쓰기 위해 사용될 때
- 데이터에 대한 접근을 동기화할 수 있는 메커니즘이 없을 때

`data races`는 예측하기 어려운 결과를 유발하며, 런타임에서 원인을 파악하고 수정하기 또한 어렵다. 러스트는 이런 케이스를 컴파일을 허용하지 않는 방식으로 문제를 예방한다.

```rust
let mut s = String::from("hello");

let r1 = &s; // O
let r2 = &s; // O
let r3 = &mut s; // Error
```

위와 같이 불변 참조를 사용 중일 때에는 가변 참조를 생성할 수 없다. 어디선가 불변 참조를 사용한다면 그 값이 변경되서는 안되기 때문이다. 데이터를 읽는 행위는 아무런 영향이 없기 때문에 불변 참조가 여러개 생성되는 것은 무방하다.

## 죽은 참조

죽은 포인터란, 이미 해제되어 다른 정보를 저장하도록 변경된 메모리를 계속해서 참조하는 포인터. 러스트는 이 죽음 참조가 발생하지 않도록 컴파일러가 보장해준다. 어떤 데이터에 대한 참조를 생성하면, 컴파일러가 데이터에 대한 참조를 실행하기에 앞서 데이터가 범위를 벗어났는가 확인한다.

```rust
fn main() {
  let ref_to_nothing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");

  &s // 변수 s는 dangle 스코프내에 생성되었기 때문에, 실행 후에 메모리는 해제되게 된다.
     // 해제된 메모리를 계속하여 참조하려 시도하기 때문에 에러를 유발할 수 있으므로, 컴파일에서 허용되지 않는다.
}
```

## 참조에 대한 규칙

- 어느 한 시점에 코드는 하나의 가변 참조 혹은 여러 개의 불변 참조를 생성할 수 있으며, 둘 모두를 생성할 수는 없다.
- 참조는 항상 유효해야한다.

# 슬라이스 타입

슬라이스는 소유권을 갖지 않는 타입이다. 컬렉션 전체가 아닌 컬렉션 내의 연속된 요소들을 참조할 수 있다.

```rust
fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s); // word = 5;

  s.clear();

  // 이 경우 word의 5값은 실제로는 아무런 쓸모가 없기 때문에 이를 이용하여 clear된 s에서 추출하려 하면 버그를 유발한다.
  // 그렇다 하여 word의 값을 s의 변화에 따라 조정하는 것은 유용하지도 않고 에러를 유발하기 쉽다.
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();                         // String 타입을 bytes 배열로 return

  for (i, &item) in bytes.iter().enumerate() {  // iter로 bytes배열의 iterator 생성
                                                         // enumerates는 (index, &item) 을 리턴
    if item == b' ' {                                    // b' '는 공백 표현의 바이트 리터럴 문법
      return i;
    }
  }
  s.len()
}
```

## 문자열 슬라이스

```rust
fn main() {
  let s = String::from("hello world");

  let hello = &s[0..5];   // String에 대한 참조 뿐 아니라 지정된 부분에 대한 참조만을 얻게 된다.
  let world = &s[6..11];

  println!("{} {}", hello, world);
}
```

`[start...end]`의 형태이며, `start`는 시작할 요소의 인덱스, `end`는 마지막 요소 다음 인덱스를 의미한다.
즉 `end`에서 `start`만큼 뺀 데이터를 저장하는 구조체.

```rust
  let s = String::from("hello");

  let len = s.len();
  // 아래 둘은 동일
  let slice = &s[0..2];
  let slice = &s[..2];

  // 아래 둘도 동일
  let slice = &s[3..len];
  let slice = &s[3..];

  // 아래 둘도 동일
  let slice = &[0..len];
  let slice = &[..];
```

> 문자열 슬라이스의 범위 인덱스는 반드시 유효한 UTF-8 문자를 추출해야하며, 다중바이트 문자의 중간에 해당하는 인덱스로 생성하게 된다면, 프로그램은 에러와 함께 중단 될 것이다.

문자열 슬라이스를 이용하여 위의 `first_word`함수를 수정하면 아래와 같다.

```rust
fn main() {
  let s = String::from("hello world");

  let word = first_word(&s);

  // 아래 코드는 이제 정상적으로 컴파일 에러를 표시한다.
  s.clear();

  println!("the first word is : {}", word);
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
```

### 문자열 리터럴은 슬라이스다.

```rust
let s = "Hello, world";
```

위의 s의 타입은 `&str`이다. 바이너리의 어느 한 지점을 가리키는 슬라이스다. `&str`은 불변 참조이기 때문에 문자열 리터럴은 불변참조이다.

이를 이용해서 `first_word` 함수를 한번 더 개선하면 다음과 같다.

```rust
fn main() {
  let my_string = String::from("hello world");

  let word = first_word(&my_string[..]);

  let my_string_literal = "hello world";

  let word = first_word(&my_string_literal[..]);

  let word = first_word(my_string_literal);
}

// String -> &str으로 변경함으로써 같은 기능이지만
// 더 보편적이고 유용한 API를 디자인할 수 있다.
// literal은 그대로 전달하면 되고, String은 전체 슬라이스를 전달하면 된다.
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

## 다른 타입의 슬라이스

```rust
fn main() {
  let a = [1,2,3,4,5];

  let slice = &a[1..3]; // 문자열 슬라이스와 동일하게 참조할 수 있다.
}
```
