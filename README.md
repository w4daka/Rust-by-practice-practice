# Rust By Practiceをやる

I do [this book](https://practice.course.rs/why-exercise.html)

次のコードを実行しようとすると

```rust
fn main() {
    let x: i32;
    let y: i32;

    assert_eq!(x, 5);
    println!("Success!")
}
```

次のようなエラーがでる

```shell
   Compiling variable v0.1.0 (/home/doxaripo/projects/rust_by_practice/variable)
error[E0381]: used binding `x` isn't initialized
 --> src/main.rs:5:5
  |
2 |     let x: i32;
  |         - binding declared here but left uninitialized
...
5 |     assert_eq!(x, 5);
  |     ^^^^^^^^^^^^^^^^ `x` used here but it isn't initialized
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider assigning a value
  |
2 |     let x: i32 = 42;
  |                ++++

warning: unused variable: `y`
 --> src/main.rs:3:9
  |
3 |     let y: i32;
  |         ^ help: if this is intentional, prefix it with an underscore: `_y`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

For more information about this error, try `rustc --explain E0381`.
warning: `variable` (bin "variable") generated 1 warning
error: could not compile `variable` (bin "variable") due to 1 previous error; 1 warning emitted

[Process exited 101]
```

和訳してみる。

- used binding `x` isn't initialized →"使われている束縛 x が初期化されていない"
- kimiによるとRustのコンパイラーのエラーメッセージでは変数`x`が宣言されているが値が代入される前に使用されたことを表すらしい。
- this error originates in the macro `assert_eq`→このエラーは`assert_eq`マクロに起因する。
- `assert_eq`を展開してみた。

```rust
    match(&x, &5){
    (left_val,right_val) => {
        if!(*left_val== *right_val){
            let kind = core::panicking::AssertKind::Eq;
            core::panicking::assert_failed(kind, &*left_val, &*right_val,core::option::Option::None);
        }
    }

    };
```

warnだからyの方はとりあえず無視する。

```rust
fn main() {
    let x: i32 = 5;
    let y: i32;

    assert_eq!(x, 5);
    println!("Success!")
}
```

にしたら一応通った。が警告が出る。

```shell
warning: unused variable: `y`
 --> src/main.rs:3:9
  |
3 |     let y: i32;
  |         ^ help: if this is intentional, prefix it with an underscore: `_y`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `variable` (bin "variable") generated 1 warning (run `cargo fix --bin "variabl
e" -p variable` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/variable`
Success!

[Process exited 0]
```

消せばいいってこと？
そうじゃねえだろ

- help: if this is intentional, prefix it with an underscore: `_y`→「もしこれが意図的なもの（あえて使っていないの）なら、接頭辞にアンダースコアをつけて \_y にしてください

これで通った。

```rust
fn main() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Success!")
}
```

### Use `mut` to mark a variable as mutable.

以下のコードはmutをmutableにするためにつけたら直った。

```rust

// Fill the blanks in the code to make it compile
fn main() {
    let __ __ = 1;
    __ += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
```

通ったcode

```rust

// Fill the blanks in the code to make it compile
fn main() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
```

## Scope

### A scope is the range within the program for which the item is valid

和訳「スコープとは、プログラム内において、ある項目（変数や関数など）が有効である範囲のことです。」

次のcodeを実行すると

```rust
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y);
}
```

以下のようなerror

```shell
   Compiling scope v0.1.0 (/home/doxaripo/projects/rust_by_practice/scope)
error[E0425]: cannot find value `y` in this scope
 --> src/main.rs:8:70
  |
8 |     println!("Outer scope value of x is {} and value of y is {}", x, y);
  |                                                                      ^
  |
help: the binding `y` is available in a different scope in the same function
 --> src/main.rs:5:13
  |
5 |         let y: i32 = 5;
  |             ^

For more information about this error, try `rustc --explain E0425`.
error: could not compile `scope` (bin "scope") due to 1 previous error

[Process exited 101]
```

和訳してみる

- error[E0425]: cannot find value `y` in this scope
- スコープ内でyというvalueを見つけられない。
- value = 「値」「変数」
- help: the binding `y` is available in a different scope in the same function
- 束縛された`y`を同じ変数として異なるscopeで有効にすることが解決策ではなく`y`という名前の束縛自体は別の場所にある。ということ。
  外に出したら解決したがerrorの説明も見る

  解決したcode

```rust
fn main() {
    let x: i32 = 10;
    let y;
    {
        y = 32;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y);
}
```

#### エラーの説明

An unresolved name was used.

Erroneous code examples:

```rust
something_that_doesnt_exist::foo;
// error: unresolved name `something_that_doesnt_exist::foo`

// or:

trait Foo {
    fn bar() {
        Self; // error: unresolved name `Self`
    }
}

// or:

let x = unknown_variable;  // error: unresolved name `unknown_variable`
```

Please verify that the name wasn't misspelled and ensure that the
identifier being referred to is valid for the given situation. Example:

```
enum something_that_does_exist {
    Foo,
}
```

Or:

```
mod something_that_does_exist {
    pub static foo : i32 = 0i32;
}

something_that_does_exist::foo; // ok!
```

Or:

```
let unknown_variable = 12u32;
let x = unknown_variable; // ok!
```

If the item is not defined in the current module, it must be imported using a
`use` statement, like so:

```
use foo::bar;
bar();
```

If the item you are importing is not defined in some super-module of the
current module, then it must also be declared as public (e.g., `pub fn`).

---
