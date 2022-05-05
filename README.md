# tuneweather

## 概要

Rustの練習のため、簡単なCLIツールを作りました

![image](https://user-images.githubusercontent.com/911649/166955829-1b8e3001-1c21-4ffb-a457-fef3c2d05de0.png)

以下の例のように、パラメータに都市名を指定して（tokyo,osaka,nagoyaのみ対応）直近5日分の天気を取得します。

データ元は、ログインなしで使用できるWebAPIを使用させていただいています

https://open-meteo.com/en

```
## 実行例
cargo build
cd ./target/debug
./tuneweather --city nagoya
```

## 注意点

* 安全性・保守性等、色々と問題があると思います。

## 初めてRustを触ってみた感想

非常に簡単な機能しかないですが、

* パラメータの受け取り（clap)
* WebAPIからのデータ取得(surf)
* JSONから構造体へのパース(serde)

など、色々勉強になりました。


