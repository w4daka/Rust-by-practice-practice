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
