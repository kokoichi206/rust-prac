## 変数定義


``` teeeeeeee
/x 5 def
/y 20 def

puts
```

## 条件式

``` 
{
    (condition)
}
{
    (true 時に実行される)
}
{
    (false 時に実行される)
} if
```

``` 
{
    x y <
}
{
    y x -
}
{
    x y -
} if
```

## 関数定義

``` teeeeeeee
/square { dup * } def

/double { 2 * } def

10 double puts
10 square puts
```
