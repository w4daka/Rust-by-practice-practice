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

---

An unresolved name was used.(未解決の名前が使われている)

Erroneous code examples:(誤ったコードの例)

```rust
something_that_doesnt_exist::foo;
// error: unresolved name `something_that_doesnt_exist::foo`
//存在しないモジュールからfooという中身を実行しようとしている

// or:

trait Foo {
    fn bar() {
        Self; // error: unresolved name `Self`
    }
}
// trait = 他の型が実装すべき「共通の振る舞い（メソッド）」を定義するためのインターフェースのようなもの。ここでは`Foo`という名前のtraitを作っている。

//fn bar(){...}= そのtraitを実装した型が共通でつかえる。「初期の振る舞い」を定義できる特に指定がなければ、みんなこの`bar`を使ってねっという意味(詳しくは補足へ)
//Self = そのtraitを実装している型自身を指す型別名 なぜエラーになるかはselfは型であって実行できる値や変数ではないから。

// or:

let x = unknown_variable;  // error: unresolved name `unknown_variable`
```

Please verify that the name wasn't misspelled and ensure that the
identifier being referred to is valid for the given situation. Example:

```rust
enum something_that_does_exist {
    Foo,
}


Or:
mod something_that_does_exist {
    pub static foo : i32 = 0i32;
}

something_that_does_exist::foo; // ok!

Or:

let unknown_variable = 12u32;
let x = unknown_variable; // ok!
```

If the item is not defined in the current module, it must be imported using a
`use` statement, like so:

```rust
use foo::bar;
bar();
```

If the item you are importing is not defined in some super-module of the
current module, then it must also be declared as public (e.g., `pub fn`).

---

#### 補足(Gemini)

#### 「「共通の振る舞い（メソッド）」を定義するためのインターフェースがよくわからない」という自分の問いに対するgeminiの返答

「インターフェース」や「共通の振る舞い」という言葉は、プログラミング独特の抽象的な表現なので、最初はピンとこなくて当然です！

身近な例として、**「リモコン」**を想像してみると分かりやすくなります。

---

1. 「インターフェース（トレイト）」を現実で例えると？

世の中には、テレビ、エアコン、照明など、色々な家電がありますよね。これらは全く別物ですが、共通して**「スイッチを押せば動く」**という振る舞いを持っています。

ここで、**「電源ボタンがあるもの」**という共通のルールを定義したとします。これがプログラミングでいう**トレイト（インターフェース）**です。

- **トレイトの名前:** `電源操作`
- **共通のメソッド:** `スイッチを入れる()`

---

2. なぜこれが必要なの？（メリット）

もし「電源操作」という共通のルールがないと、プログラムを書くときにこうなります。

- テレビを付けるときは `tv.turn_on_tv()`
- エアコンのときは `aircon.start_cooling()`
- 照明のときは `light.switch_light_on()`

これでは、新しい家電が増えるたびに、専用の呼び出し方を覚え直さなければなりません。

しかし、トレイト（インターフェース）を使えば、**「中身が何であれ、とにかく『スイッチを入れる()』という名前のボタンさえ持っていればOK！」**という仕組みが作れます。

---

3. コードで見るとどうなる？

Rustのトレイトで書くと、このようになります。

```rust
// 1. 「共通の振る舞い」というルールを決める
trait Power {
    fn switch_on(&self); // 「このボタンを実装してね」という約束
}

// 2. テレビにそのルールを適用する
struct TV;
impl Power for TV {
    fn switch_on(&self) {
        println!("テレビの画面が付きます");
    }
}

// 3. エアコンにそのルールを適用する
struct Aircon;
impl Power for Aircon {
    fn switch_on(&self) {
        println!("エアコンから風が出ます");
    }
}

```

このように、**「TVもAirconも `Power` という同じルール（インターフェース）に従っている」**ので、使う側は相手がTVかAirconかを深く気にせず、ただ `switch_on()` を呼べば良くなります。

---

4. まとめ

- **インターフェース（トレイト）**: 「どんな機能（メソッド）を持っているか」という**契約書**。
- **共通の振る舞い**: 違う種類のデータ（TVやエアコン）であっても、**同じ名前の命令（メソッド）で動かせる**こと。

「中身がバラバラでも、操作方法だけ統一する」というのがインターフェースの真髄です。

この「共通のルール」を使って、複数のものを一気に操作する方法（ジェネリクスなど）について、具体例を見てみますか？

#### 「そのトレイトを実装した型が共通で使える「初期の振る舞い」を定義できます。がよくわからない」という問いに対するgeminiの回答

「初期の振る舞い（デフォルト実装）」は、いわば**「テンプレート（ひな形）」**のようなものです。

先ほどのリモコンの例をもう少し進化させて説明しますね。

---

1. 「デフォルト実装」を現実で例えると？

例えば、**「スマート家電」というトレイト（ルール）**を作るとします。
最近のスマート家電には、だいたい**「スマホに通知を送る」**という機能がついていますよね。

このとき、テレビも、エアコンも、冷蔵庫も、**「通知を送る」という動作自体は全く同じ（スマホにメッセージを飛ばすだけ）**だとします。

- **デフォルト実装がない場合：**
  テレビ、エアコン、冷蔵庫のそれぞれの説明書（実装コード）に、毎回同じ「通知の送り方」を書き込まなければなりません。面倒ですし、書き間違えるかもしれません。
- **デフォルト実装がある場合：**
  トレイト（ルール）側に、あらかじめ「通知の送り方」を書いておきます。すると、各家電は**「特にこだわりがなければ、この共通のやり方を使ってね」**という状態になります。

これが**「初期の振る舞い」**です。

---

2.  コードで見てみる

Rustのコードで、この「初期の振る舞い」をどう書くか見てみましょう。

```rust
trait SmartDevice {
    // 1. 各家電で「中身」が違うので、名前だけ決める（デフォルトなし）
    fn operate(&self);

    // 2. どの家電でも「中身」が同じなので、最初から書いておく（デフォルト実装）
    fn send_notification(&self) {
        println!("スマホに「動作完了」と通知しました（共通機能）");
    }
}

struct RobotVacuum; // お掃除ロボット
impl SmartDevice for RobotVacuum {
    fn operate(&self) {
        println!("掃除を開始します");
    }
    // send_notification は書かなくても、自動的に「共通機能」が使える！
}

```

---

3.  「上書き（オーバーライド）」もできる

もし、特定の家電だけ「特別な通知を送りたい」と思ったら、デフォルトの振る舞いを**上書き**することもできます。

```rust
struct SecurityCamera; // 防犯カメラ
impl SmartDevice for SecurityCamera {
    fn operate(&self) {
        println!("録画中...");
    }

    // デフォルトは使わず、自分専用の動きにする！
    fn send_notification(&self) {
        println!("【緊急】不審者を検知しました！");
    }
}

```

---

まとめ：なぜ「初期の振る舞い」が便利なのか？

- **楽ができる：** 同じコードを何度も書かなくて済む。
- **最低限の保証：** そのトレイトを実装しさえすれば、細かい設定をしなくても「最低限これだけは動く」という状態にできる。
