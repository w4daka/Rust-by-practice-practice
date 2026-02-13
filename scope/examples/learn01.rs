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

fn main() {
    let my_tv = TV;
    let my_aircon = Aircon;

    my_tv.switch_on();
    my_aircon.switch_on();
}
