# CLIP STUDIO REMOTE

ブラウザから、PC上の CLIP STUDIO を遠隔操作する

- PC側で Node.js サーバを立ち上げる
  - WebSocket でメッセージを受信すると、その内容に対応したキーを押下するイベントを発火させる
- Node.js サーバから配信する静的ページでCLIP STUDIOを操作するためのGUIを表示
  - ボタンをタップされると WebSocket でメッセージを送信する

## 動作環境

PC に関しては macOS のみ対応

## ビルド

esbuild を用いて、フロントエンド側の js をバンドルする

```
$ yarn build
```

## 起動方法

```
$ yarn start
```

スマホブラウザ等から `localhost:8080` にアクセスすると、CLIP STUDIO を操作するためのGUIが表示される
