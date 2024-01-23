``` sh
cargo run --quiet -- --help
```

cat の仕様

``` sh
$ man cat
...
     -b      Number the non-blank output lines, starting at 1.

$ cat -n tests/inputs/include_blank.txt
     1  hoge
     2  the following line is blank
     3
     4  pien

# i don't know when to use this flag...
$ cat -b tests/inputs/include_blank.txt
     1  hoge
     2  the following line is blank

     3  pien
```
