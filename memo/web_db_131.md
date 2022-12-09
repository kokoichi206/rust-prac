## Rust

GC を持たず、コンパイル時にメモリの使い方を決定するためメモリ効率がいい。

| 従来版 |             Rust 版             |
| :----: | :-----------------------------: |
|   ls   | [exa](https://the.exa.website/) |
|  grep  |             ripgrep             |
|  cat   |               bat               |
|  find  |               fd                |

- https://github.com/ogham/exa
- https://github.com/BurntSushi/ripgrep

Rust が持つ型システムと Rust 特有の所有権モデルによるメモリ管理のおかげで、メモリ安全でないコード、スレッド間でデータ競合の起こりうるコードはコンパイルエラーになる！

```sh
cargo new first-project

cd first-project
cargo run
```

Rust がマルチパラダイムであるが故の難しさは、ドキュメントでキャッチアップできるよう Rust コミュニティが学習曲線の緩和に注力してくれている！  
「高パフォーマンスかつ安全」なプログラムを開発することは、そもそも難しい！なので、コンパイルエラーと向き合うことは安全性を得るための必要な学習コストと思おう！

- main 関数が最初に実行される
- let で変数を束縛
- デフォルトではイミュータブル
- Option 型「存在しない可能性がのある値」
- Result 型「Option と似ているが、エラー表現の Err にも値を持てる」
- 所有権を持つ = 「変数がある値を保持していること」
  - 所有権を渡さない限りは、スコープを抜ける時に変数が破棄される
  - 所有権を移動させずに値を参照できる「借用」という仕組み、いいかもしれない
  - Borrow Checker により、所有と借用の関係性の正しさをコンパイル時に調べる
- インターフェース継承のみある
  - trait
- async/await による非同期プログラミング
  - Rust では非同期プログラミングの記法のみ規定あり
  - 利用する非同期ランタイムはベット指定する必要がある！
    - Tokio, async-std, smol など
    - Tokio が活発かな

### Cargo

Cargo は Rust におけるパッケージ管理兼ビルドツール

```sh
cargo --list

cargo add rand

rustup component add rustfmt
rustup component add clippy

cargo fmt

# 静的解析ツール
cargo clippy

10073  brew reinstall rustup-init
10074  rustup-init\n
10075  cargo fmt\n
10076  cargo clippy\n

cargo test

cargo install cargo-make
```

test を実際に書くときは、cfg, mod などとして、テスト時以外はコンパイルしないようにする！

[特別扱いされるディレクトリ: Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)

- rust-analyzer は必須！

### web 開発の部分

`?` の文字は、Rust のエラー処理の糖衣構文。  
`Result<T, E>` を返す型に対し、失敗時は ? でリターンし、成功時は中身を取り出す、という動作をする。

