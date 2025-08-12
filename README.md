# workout_rust
Rustの修行用リポジトリ。Rustで作成したアプリケーションを追加していく。

## ディレクトリ構成
```
workout_rust
┗╸ todo_cli
```

| ディレクトリ名 | 内容 | 学び |
| --- | --- | --- |
| todo_cli | コマンドラインを使ってタスクの「追加」「完了」「削除」「クリア」を実行できる。 | 構造体、ファイル入出力、エラーハンドリング |
| todo_gui | GUIを使ったToDoリスト | "egui+eframe" |
| test_proj | Rustのテストを書く練習をする |
| rpn_cli | 逆ポーランド記法(RPN: Reverse Polish Notation)を計算するアプリケーション<br> - 扱う演算子は「+ - * / %」のみ<br> - 数値は32ビット整数のみ |
| todo_web | ToDo Webアプリケーション | Webフレームワーク：actix-web |