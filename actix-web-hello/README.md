## Setup

```sh
cargo add actix-web

cargo add serde -F derive
```

Serde は、Rust の型から他のデータ形式に変換する操作（シリアライズ）、およびその逆（デシリアライズ）をサポートするクレートです！

derive アトリビュートで Deserialize を指定、とかできる。  
Deserialize を derive する！

```sh
curl http://localhost:8080/?name=pien&age=18
```
