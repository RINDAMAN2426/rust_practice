# Managing Growing Projects with Packages, Crates, and Modules

패키지는 `multiple binary crates`를 포함. `library crate`도 추가 가능

`cargo`는 패키지 간에 연관이 많은 대형 프로젝트를 위한 `Cargo workspace`를 제공한다.

`module system` 은 아래와 같다

- `Packages` A Cargo feature that lets you build, test, and share crates
- `Crates` A tree of modules that produces a library or executable
- `Modules and use` Let you control the organization, scope, and privacy of paths
- `Paths` A way of naming an item, such as a struct, function, or module

## Packages and Crates

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

## Defining Modules to Control Scope and Privacy

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
