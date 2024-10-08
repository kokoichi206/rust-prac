# memo

- すべてのコンピュータが並列らしい
- 型安全であることと、言語の方チェックがコンパイル時に行われるか実行時に行われるかは独立
  - c はコンパイル時に方チェックをするが型安全ではない
  - python は実行時に行うが方安全
- ベクタ
  - ピープ上に確保される
  - 可変長
- **ダングリングポインタ**が c, c++ ではよく発生していた
  - Rust では解放済みのオブジェクトへのポインタを使ってしまうことはない
- Rust
  - 値の生存期間の制御
  - 解放済みオブジェクトへのポインタを使えない仕組み
- 制約により考えなくて良くなったこと
  - ダングリングポインタ
  - 多重フリー
  - 初期化されていないメモリ
- **ポインタが安全に使われることが証明されている**

## 所有権

- c, c++
  - 所有者が被所有者の生存期間を決定し、他のものはその決定を尊重する必要がある
- rust
  - 変数はその値を所有する
  - その宣言されたブロックを制御が離れた時、変数はドロップされ、値もドロップされる
- `Box<T>` は所有の別の例
  - ヒープ上の型 T の値へのポインタ
  - `Box::new(v)`
    - ヒープ上にメモリを確保し、値 v をそこに移動し、そのヒープ空間を指す Box を返す
- Rust における値のドロップ
  - 変数がスコープから外れる
  - ベクタの要素を削除
  - etc..

### 移動

- 関数の引数や戻り値では、移動、が発生している
  - 受渡先が値の生存期間を制御する
- 代入
  - Python
    - 参照カウントをすることを代償に、代入を安価にした
  - C++
    - オブジェクトのディープコピーをすることを代償に、全てのメモリの所有権を明確にした
- 所有権が渡ると、**代入元は未初期化状態になる**
  - 初期化されていない値の使用を賢明にも禁じている
- 移動されるのは
  - 値そのものであって、それが保有するヒープ上のストレージは移動されない
  - コンパイラのコード生成は、それらをうまく見透かす
- Copy 型
  - **ほとんどの型は移動する**
  - Copy 型の値を代入すると、値は移動されず、コピーされる
  - 単純なビット単位のコピーだけで事足りる型がだけが Copy となり得る
- **基本的な操作は単純であるべきで、高価である可能性がある操作は明示的に行うべき**

### Rc と Arc: 所有権の共有

- ほとんどの値が唯一の所有者を持つ
- 例外
  - **その値を使っている全てのものが使い終わるまで生きていて欲しいような値**
  - **参照カウンタのポインタ型 Rc と Arc**
  - Atomic Reference Count
- Rust のメモリ安全性とスレッド安全性は、**ある値が、共有されていると同時に可変にはならない**ことに依存している
  - Rc ポインタから参照されているものは共有されるものと想定するので、不変であるべきだと判断する

## 参照

- `Box<T>` や Vec の内部にあるポインタは、所有権を持つポインタであった
- **所有権を持たないポインタ型、参照、もある！**
  - 参照先の生存期間に何の影響も持たない
- 参照は参照先よりも長生きしてはいけない
  - **ある値に対する参照を作ることを借用**と呼ぶ
    - 借りたものはいつかは所有者に返す必要がある
- 参照
  - 共有参照: shared reference
    - 参照先を読むことはできるが変更はできない
    - `&T`
    - Copy 型である
  - 可変参照: mutable reference
    - **同じ値に対する可変参照と、他の参照は同時に使用できない！**
    - `&mut e`
    - Copy 型ではない
- **ある値への共有参照が存在する間は、所有者といえど値を変更できない**
- **共有と更新を完全に別に扱う**
- 値渡しと参照渡し
  - **値渡し**: pass by value
    - **値の所有権を移動するような方法**で関数へ値を渡すこと
  - **参照渡し**: pass by reference
    - **関数に値の参照**を渡すこと
  - **Rust ではこの違いが所有権に影響する**
- **参照解決**
  - `.` 演算子については、暗黙的に**左のオペランドを参照解決**している
    - `println!` マクロも `.` 演算子を使うコードに展開されているため！！
  - **C++ との違い**
    - 必要がある場所全てで、暗黙に参照と左辺値（メモリの位置を示す式）との間で変換している！
- **参照はヌルにならない**
  - 参照のデフォルト初期値は存在しない
  - `Option<&T>`
    - **機械語のレベルでは None はヌルポインタ**
    - `Some(r)` はゼロ以外のアドレス
    - 使う前に None 可動化を確認しないと使えない
  - **Rust では参照先がなくなった参照ができてしまうようなコードは書けない**
    - 参照先よりも長生きすることは許されない
- **fat pointer**
  - 何らかの値へのアドレスと、その値を使うために必要な情報を持つワードの２ワードで構成されたもの
  - **スライスへの参照**
    - スライスの開始点のアドレスと長さをもつファットポインタ
  - **trait object**
    - **特定のトレイトを実装した値への参照**
- **生存期間: lifetime**
  - **全ての参照型に対して**、その参照の使われ方によって生じる性夜雨を反映した生存期間を割り当てる
  - ある参照が安全に利用できる機関のこと！
  - プログラム中での参照の使われ方によって生じる制約を理解し、その制約を満たす生存期間を見つける！
  - `'a`
    - **tick A と発音する**
    - 生存機関パラメータ
  - `<'a>`
    - **任意の生存期間 `'a` に対して**
- `'static` 生存期間
  - グローバル変数に参照を隠すような関数を書くには、関数のシグネチャにそにとを反映させる必要がある
- 生存機関の省略
  - **関数が引数として参照を1つだけ取り、1つだけ参照を返す場合, Rust コンパイラがコレらの参照が同じ生存期間を持つと仮定する**
  - 省略できない
    - **参照型が他の型の定義に含まれている場合**

``` rust
struct S<'a> {
    r: &'a i32,
}
```

- `S {r: &x}`
  - tick a は x の生存機関の中に含まれるという制約
- コレクションを、参照しつつ同時に変更するのは、Rust に限らず難しい領域となっている
  - 注意が必要
  - Rust では変更アクセスを排他的にすることで防いでいる
- 共有参照と可変参照が混在できない
  - データの競合が生じる条件
    - ある値が可変
    - スレッド間で共有されている
    - 場合のみ
  - Rust では ↑ の状況を排除している！
