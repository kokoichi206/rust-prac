## クレートとモジュール

- パッケージ
  - 機能を提供する単位
  - ローとディレクトリに Cargo.toml
- クレート
  - Rust のコンパイル単位
  - 2種類
    - ライブラリクレート
      - 他のパッケージから利用できるよう
      - `src/lib.rs`
    - バイナリクレート
- モジュール
  - コードをグループ化する機能
  - pub は**1つ上のスコープ**への公開！

## ライブラリ

- anyhow/thiserror
  - Err をより扱いやすくする
  - dyn Trait
    - impl Trait のように、トレイトを型として扱う構文
    - impl はコンパイル時にサイズが確定
    - dyn は実行時にサイズが決まる → **動的ディスパッチ！**
- Serde
  - JSON を扱う
    - YAML, TOML も

## db

- diesel
  - OR mapping, クエリービルダー
  - async どうなってる？
  - 高機能
- sqlx
  - 後発
  - シンプルな SQL ライブラリ
  - **SQL のコンパイル時チェック、マイグレーションのみ**
  - 非同期処理に対応

``` sh
cargo install sqlx-cli

sqlx migrate add init
sqlx migrate add label
```
