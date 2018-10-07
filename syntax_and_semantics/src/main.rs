// main関数
fn main() {
    variable_bindings();
    primitive_type();
}

// プリミティブ型
fn primitive_type() {
    // boolean
    let b: bool = true;
    println!("bool: {}", b);
    // char
    let c: char = 'x';
    let a = 'あ'; // charは4byte
    println!("char: {}, {}", c, a);
    // 数値
    let n = 32; // i32
    let f = 1.4; // f64
    println!("n: {}, f: {}", n, f);
    let size: isize = 64; // isize, usize 環境のポインタサイズに依存
    println!("size: {}", size);
    // 配列
    let array = [1, 2, 3];
    let mut m_array = ["Graydon", "Brian", "Niko"];
    // 初期化
    let i_array = [0; 20]; // 20の長さの配列を0で初期化
    // 長さはlenメソッドで取得
    println!("allay len: {}", array.len()); // 3
    // 添え字アクセス
    println!("m_array[1]: {}, i_array[19]: {}", m_array[1], i_array[19]); // Brian
    // 配列の内容を変更
    m_array[1] = "bob";
    println!("m_array[1]: {}", m_array[1]); // Bob
    // スライシング
    let complete = &array[..]; // すべての要素のスライス
    println!("complete: size:{}, complete[2]: {}", complete.len(), complete[2]);
    // タプル(固定サイズの順序ありリスト, 違う型でもOK)
    let t = (1, "hello");
    // 展開可能
    let (one, hello) = t;
    println!("one: {}, hello: {}", one, hello);
    // タプルは[]ではなく.(n) でアクセス
    println!("t.1: {}", t.1);
}

// 関数は後ろに書いてもOK
// 変数束縛
fn variable_bindings() {
    // 初期化、デフォルト変更不可
    let a = 1;
    // 左側の式はパターン
    let (x, y) = (3, 4);
    // 型指定も可能
    let b: i32 = 5;
    // 変更可能にするならmut
    let mut c = 6;
    println!("c:{}", c);
    c = 7;
    // 後で初期化も可能
    let d;
    d = 8;
    println!("a:{} b:{} c:{} d:{} x:{} y:{}",a, b, c, d, x, y);
    // スコープとシャドーイング
    let e: i32 = 17;
    {
        // スコープ内では上書き
        let e: i32 = 1;
        let f: i32 = 2;
        // eは1
        println!("e:{} f:{}", e, f);
    }
    // ここではfはアクセス不可
    // eは17
    println!("e:{}", e);
    // 関数呼び出し
    print_number(5);
    print_sum(5, 6);
    print_number(add_one(100));
    print_number(foo(100, true));
    print_number(foo(100, false));
    // 関数ポインタ
    let f: fn(i32) -> i32 = add_one;
    print_number(f(200)); // 201
    // 型指定省略可
    let f2 = add_one;
    print_number(f2(300)); // 301
    // panic
    // diverges();
    // 発散関数は任意の型として利用可能?
    // let fx: i32 = diverges();
    // let fy: String = diverges();
}

// 関数
fn print_number(x: i32) {
    println!("number is: {}", x);
}

// 二つの引数
fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

// 戻り値
fn add_one(x: i32) -> i32 {
    x + 1 // 戻り値はセミコロンをつけない
}

// 早期リターン
fn foo(x: i32, no_add: bool) -> i32 {
    if no_add { return x; } // 早期リターンの場合はreturnで
    x + 2 // return をつけても良いが、よろしくないスタイル
}

// 発散する関数
//fn diverges() -> ! {
//    panic!("This function never returns!");
//}
