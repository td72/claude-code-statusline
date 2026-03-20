# claude-code-statusline

[Claude Code](https://code.claude.com/) 用の、設定可能なマルチラインステータスラインツール。Rust製。

stdinからJSONセッションデータを読み取り、ANSIカラー付きのカスタマイズ可能なステータスバーを描画します。レイアウトとスタイルはTOML設定ファイルで制御します。

```
[Opus 4.6 (1M context)] | 📁 my-project | 🤖 security-reviewer | [NORMAL]
██░░░░░░░░ 17% (173k/1.00M)
$0.87 | 9m 2s | +423 -15
5h: █░░░░ 24% resets in 2h 30m | 7d: █░░░░ 15% resets in 2d 14h
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
widgets = ["model", "workspace", "agent", "worktree", "vim"]
separator = " | "

[[line]]
widgets = ["context_usage", "cost_summary", "token_alert", "rate_limit_5h"]
separator = " | "

# 各ウィジェットをカスタマイズ
[widget.model]
color = "cyan"
bracket = "square"

[widget.context_usage]
bar_width = 10
show_tokens = true
token_style = "compact"
```

### 利用可能なウィジェット

| ウィジェット | 説明 |
|---|---|
| `model` | モデル名 (例: `[Opus]`) |
| `workspace` | 現在のディレクトリ |
| `agent` | エージェント名 (非アクティブ時は非表示) |
| `worktree` | ワークツリーブランチ (非アクティブ時は非表示) |
| `vim` | Vimモード (無効時は非表示) |
| `context_usage` | コンテキストウィンドウ プログレスバー + トークン数 |
| `cost_summary` | コスト、経過時間、変更行数 |
| `token_alert` | 200kトークン超過時の警告インジケータ |
| `rate_limit_5h` | 5時間レート制限使用率 + カウントダウン |
| `rate_limit_7d` | 7日レート制限使用率 + カウントダウン |

データがないウィジェット（例: `--agent` 未使用時の `agent`）は自動的にスキップされます。

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
