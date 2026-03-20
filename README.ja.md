# claude-code-statusline

[Claude Code](https://code.claude.com/) 用の、設定可能なマルチラインステータスラインツール。Rust製。

stdinからJSONセッションデータを読み取り、ANSIカラー付きのカスタマイズ可能なステータスバーを描画します。レイアウトとスタイルはTOML設定ファイルで制御します。lualineスタイルのバッジレイアウト（背景色付き）とANSI 256色コードに対応しています。

```
 NORMAL 📁 my-project  main
🤖 Opus 4.6 ██░░░░░░░░ 17% (173k/1.00M)
💰 $0.87 | ⏱ 9m 2s | 📝 +423 -15
🕐 █░░░░ 24% ↻2h 30m   📅 █░░░░ 15% ↻2d 14h
```

## インストール

```sh
cargo install --path .
```

## セットアップ

`~/.claude/settings.json` に追加:

```json
{
  "statusLine": {
    "type": "command",
    "command": "claude-code-statusline"
  }
}
```

## 設定

設定ファイルは以下の優先順で読み込まれます:

1. `--config <path>` フラグ
2. `~/.config/claude-code-statusline/config.toml`
3. 組み込みデフォルト

デフォルト設定をコピーして始めましょう:

```sh
mkdir -p ~/.config/claude-code-statusline
cp config.default.toml ~/.config/claude-code-statusline/config.toml
```

### 設定フォーマット

TOMLで行とウィジェットを定義:

```toml
# [[line]] はステータスバーの1行
[[line]]
widgets = ["vim", "workspace", "git_branch", "worktree"]
separator = ""

[[line]]
widgets = ["model", "context_usage"]
separator = " "

[[line]]
widgets = ["cost_summary"]
separator = " | "

[[line]]
widgets = ["rate_limit_5h", "rate_limit_7d"]
separator = "   "

# 各ウィジェットをカスタマイズ
[widget.model]
prefix = "🤖 "
short = true
color = "255"

[widget.context_usage]
bar_width = 10
show_tokens = true
token_style = "compact"
```

### lualineスタイルのバッジレイアウト

`separator = ""` とウィジェットごとの背景色（`bg`）を使うことで、lualineスタイルの外観を実現できます。名前付きカラーに加えて、ANSI 256色コード（`"0"` -- `"255"`）もサポートしています。

```toml
[[line]]
widgets = ["vim", "workspace", "git_branch"]
separator = ""

# lualine テーマ: normal=#80d8ff, insert=#c3e88d, fg=#263238
[widget.vim]
normal_bg = "117"   # #87d7ff ≈ #80d8ff
normal_fg = "236"   # #303030 ≈ #263238
insert_bg = "150"   # #afd787 ≈ #c3e88d
insert_fg = "236"

# lualine section b: fg=#eeffff bg=#515559
[widget.workspace]
prefix = "📁 "
style = "basename"
color = "255"
bg = "240"

# lualine section c: fg=#eeffff bg=#2E3C43
[widget.git_branch]
prefix = " "
color = "255"
bg = "237"
```

### 利用可能なウィジェット

| ウィジェット | 説明 |
|---|---|
| `model` | モデル名 (例: `Opus 4.6`)。`short = true` で括弧付きサフィックスを除去 |
| `workspace` | 現在のディレクトリ |
| `git_branch` | 現在のgitブランチ (`git branch --show-current` で取得。リポジトリ外では非表示) |
| `agent` | エージェント名 (非アクティブ時は非表示) |
| `worktree` | ワークツリーブランチ (非アクティブ時は非表示) |
| `vim` | Vimモード。モードごとの背景色/前景色設定に対応 (無効時は非表示) |
| `context_usage` | コンテキストウィンドウ プログレスバー + トークン数 |
| `cost_summary` | コスト、経過時間、変更行数。アイコンプレフィックス設定可能 |
| `token_alert` | 200kトークン超過時の警告インジケータ |
| `rate_limit_5h` | 5時間レート制限使用率 + カウントダウン |
| `rate_limit_7d` | 7日レート制限使用率 + カウントダウン |

データがないウィジェット（例: `--agent` 未使用時の `agent`）は自動的にスキップされます。JSON入力のパースに失敗した場合、空のステータスラインではなくフォールバックメッセージが表示されます。

## ワークスペース構成

```
Cargo.toml                              # ワークスペース + CLIバイナリ
config.default.toml                     # デフォルト設定
src/                                    # CLIアプリケーション
crates/
  claude-code-statusline-model/         # statusline JSON入力のserde型定義
  claude-code-statusline-components/    # プリミティブ描画コンポーネント
  claude-code-statusline-widgets/       # 高レベルウィジェット (Widget trait)
```

## ライセンス

MIT
