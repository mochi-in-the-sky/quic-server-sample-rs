# 目的
- RustのQUIC実装[quinn](https://github.com/quinn-rs/quinn)を使ってみること
- サービス立ち上げるひな型を作ること

# 使用法
- 環境変数を取得し、設定に反映する

| 設定名 | 初期値 | 内容 |
| ------| ------| ------|
| THROWSTERHOUSE_FIVE_BIND_PORT | 7777 | ポート番号 |
| THROWSTERHOUSE_FIVE_CERT_PATH | オレオレ | 証明書パス |
| THROWSTERHOUSE_FIVE_KEY_PATH | オレオレ | 秘密鍵パス |
