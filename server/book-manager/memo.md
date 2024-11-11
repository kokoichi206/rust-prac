``` sh
cargo install --force cargo-make
```

- bin, lib mode
  - lib: クレートとして頒布する用
- cargo.toml
  - dev-dependencies セクション
- axum
  - handler, router
- hyper
  - HTTP クライアントや HTTP サーバーを立ち上げられる機能
- tower
  - タイムアウト、レートリミット、認証、ロードバランシング
  - Service, Layer
- tokio
  - 非同期処理の実行基盤
- 非同期プログラミング
  - asynchronous programming
  - 非同期タスクそれ自体は、スレッドを軽量にしたもの
  - **グリーンスレッド**
    - **ランタイム内の、仮想的に動作するスレッド**
    - Java でも M:N のグリーンスレッドが v21 から入ったぽい
    - v21 は 2023/9~
  - **IO 多重化**
    - IO の待ち時間に別の処理を行わせる
- Rust での非同期プログラミング
  - Feature トレイトがある
- **非同期ランタイム**
  - asynchronous runtime
  - 標準にランタイムが組み込まれていない
    - Rust という言語の usecase が広く、汎用的な

``` sh
# cargo-nextest
cargo install cargo-nextest
```

- sqlx
  - **ORM ではなくクエリビルダー**

``` sh
cargo add sqlx@0.7.3 --features runtime-tokio,uuid,macros,postgres,migrate
```
