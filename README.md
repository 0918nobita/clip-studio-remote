# CLIP STUDIO REMOTE

ブラウザから、PC上の CLIP STUDIO を遠隔操作する

- PC側で静的ファイル / WebSocket サーバを立ち上げる
  - メッセージを受信すると、その内容に対応したキーを押下するイベントを発火させる
- 静的ページでCLIP STUDIOを操作するためのGUIを表示
  - ボタンをタップされると WebSocket でメッセージを送信する

## 動作環境

PC に関しては macOS のみ対応

## 依存パッケージのインストール

```
$ yarn
```

## Webフロントエンドのビルド

```
$ yarn build
```

## サーバーの起動

```
$ cd server-rs
$ cargo run -- --send-keys
```

スマホブラウザ等から `localhost:8080` にアクセスすると、CLIP STUDIO を操作するためのGUIが表示される
