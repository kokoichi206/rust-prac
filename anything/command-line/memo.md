## sec 1

``` sh
cargo run --quiet

man true
```

- Exit Values Make Programs Composable
- reporting the exit status is a characteristic of well-behaved command-line programs

## sec 2

- [unit type](https://doc.rust-lang.org/std/primitive.unit.html) is like an empty value
  - It's not quite like a null pointer or undefined value in other languages
    - null reference his "billion-dollar mistake"
      - it must logically be worth at least a billion dollars
- [Args documentation](https://doc.rust-lang.org/stable/std/env/struct.Args.html)
- [clap crate](https://docs.rs/clap/latest/clap/index.html/)
  - command-line argument parser

``` sh
❯ du -shc .
 44M    .
 44M    total

❯ cargo clean
     Removed 365 files, 43.4MiB total
❯ du -shc .
 20K    .
 20K    total
```

- statement vs expression
  - statement: 文
    - some action
    - NOT return a value
  - expression: 式
    - ALWAYS return value(s)
- if `if` is a expression, u can assign it to some variable
  - Python has **both** an if statement and an if expression.
- error is just an implementation of `std::error::Error` trait
  - https://doc.rust-lang.org/std/error/index.html
- **memory**
  - stack basically
    - LIFO
    - fixed size
  - heap
    - like Vec type
    - can change its size during runtime
    - to know where to find the memory, it uses pointer in stack
      - also known as reference
- `Box` to create a smart pointer to heap memory
- **Trust, but verify.**

## sec 3

- `io::BufRead` trait
  - https://doc.rust-lang.org/std/io/trait.BufRead.html
- Box
   - **allocate memory on the heap** by putting the return value
 - Enumerate
   - https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
   - for obj and counter
- bat
  - cat clone
  - https://github.com/sharkdp/bat?tab=readme-ov-file

## sec 5

- [std::io::Cursor](https://doc.rust-lang.org/std/io/struct.Cursor.html)
  - used with in-memory buffers, 

## english

- conjunction
  - 接続詞
- ad hoc programs on the command line
- To recap:
- poke around the docs
- parlance
  - 用語
- some of which will intentionally not exist or will be unreadable
- go down the rabbit hole
  - 本筋（日常、常識）から外れる
  - 別世界（違う世界）に行く
  - 底なし沼にはまる
- That's really a lot of information, and I don't blame you if your eyes glazed over
- amorphous
  - まとまりのない
- You made big strides in this chapter
- How did that go? Swell, I bet!
- I don't know about you, but I think that's pretty sweet
