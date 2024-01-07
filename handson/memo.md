- C/C++ のメモリ管理の難しさを解決したい！
- Rust: さび
- rust-lang
  - https://github.com/rust-lang/rust
  - rust 自身も rust で書かれている！
- メモリ管理
  - python: GC
  - Rust: 所有権システム

## grammar

- `println!`
  - `!` は Rust の**マクロである**ことを示している
    - 普通の関数ではない
    - 他言語では、前処理・プリプロセッサと呼ばれる機能
  - マクロを利用することでデータ型を手軽に出力している
- 戻り値
  - `return 値;` or `値`
- 基本的に変数は値が immutable
  - 変更したいケースでは mut をつける
- **『文字』と『文字列』の区別**
  - 文字
    - 1 文字を表すデータ型
    - `'` で囲む
    - char
  - 文字列
    - 複数の文字を表すデータ型
    - `"` で囲む
    - &str
      - & は must? ppoi
      - String vs &str
- クロージャー
  - Python のラムダ式みたいなもん
- 可変配列
  - ベクター？
- expect, unwrap でエラー処理を簡略化することも可能だが、その場合は強制終了される
- Option 型
  - Some
  - None
- イテレータから直接データを取得することはできない
  - for 文などとの組み合わせが必要
  - Vector 型としておくと便利なことが！

## cargo

- パッケージマネージャーの機能も持つ
  - pip, npm
- ライブラリー（クレート）の管理
  - ライブラリ一覧
    - https://crates.io/

``` sh
cargo --help

cargo new hi
cargo run
```
