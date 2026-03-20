# claude-code-statusline-components

Claude Codeステータスライン構築用の低レベル描画コンポーネント。各コンポーネントは単一の値型を受け取り、ANSIカラーコード付きのターミナル表示用文字列を生成します。

## コンポーネント

| コンポーネント | 用途 | 出力例 |
|---|---|---|
| `ProgressBar` | パーセンテージをカラーバーで表示 | `██████░░░░ 60%` |
| `Duration` | ミリ秒を人間可読な時間に変換 | `2m 5s` |
| `Currency` | 浮動小数点数を通貨フォーマット | `$0.05` |
| `Count` | 整数をスタイル付き表示 (plain, comma, compact) | `15.0k` |
| `Countdown` | Unixタイムスタンプ差分を残り時間に変換 | `2h 30m` |
| `Label` | テキストにカラーや括弧を付与 | `[Opus]` |
| `Path` | パスの表示スタイル (basename, full, home短縮) | `~/projects/myapp` |
| `Indicator` | boolフラグをアイコン/テキストに変換 | (trueの時に警告記号) |

## ユーティリティ

- `color::Color` -- ANSIカラー列挙型 (Red, Green, Yellow, ... , Ansi256)
- `color::Threshold` -- 値の範囲をカラーにマッピング
- `color::colored()` / `color::color_for_value()` -- カラー出力ヘルパー

## 使い方

```rust
use claude_code_statusline_components::ProgressBar;
use claude_code_statusline_components::Currency;
use claude_code_statusline_components::Duration;

let bar = ProgressBar::default();
println!("{}", bar.render(73.0)); // 73%のカラーバー

let cost = Currency::default();
println!("{}", cost.render(1.234)); // "$1.23"

let dur = Duration::default();
println!("{}", dur.render(125_000)); // "2m 5s"
```

全コンポーネントは `Default` を実装しており、公開フィールドでカスタマイズ可能です。
