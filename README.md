ラズパイ用のプログラム

```
armv7-unknown-linux-gnueabihf
```

tcpの接続確認
適当なTCPサーバーを立てる
```bash
nc -l 12345 < test.txt
```

プログラムを動かす
```bash
cargo run
```


