# [4](https://practice.course.rs/variables.html) やる。

次のようなコードを実行すると

```rust
// Fix the error with the use of define_x
fn main() {
    println!("{}, world", x);
}

fn define_x() {
    let x = "hello";
}
```

以下のようなエラーが出る

```shell
   Compiling scope2 v0.1.0 (/home/doxaripo/projects/rust_by_practice/scope2)
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:3:27
  |
3 |     println!("{}, world", x);
  |                           ^ not found in this scope

warning: unused variable: `x`
 --> src/main.rs:7:9
  |
7 |     let x = "hello";
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

For more information about this error, try `rustc --explain E0425`.
warning: `scope2` (bin "scope2") generated 1 warning
error: could not compile `scope2` (bin "scope2") due to 1 previous error; 1 warning emi
tted

[Process exited 101]

```

訳してみる。

- error[E0425]: cannot find value `x` in this scope
- `x`という値をこのスコープの中で見つけられない。
- warning: unused variable: `x`
- 警告：変数`x`が使われていない。

なんとなく思うのは`let x`を宣言してさらにdefine_xの戻り値をそれに代入すればいいのでは？
