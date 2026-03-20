# claude-code-statusline-widgets

`StatusLineInput` のフィールドをステータスライン出力に変換する高レベルウィジェット。各ウィジェットは `claude-code-statusline-components` のコンポーネントを1つ以上組み合わせて、意味のある表示単位を生成します。

## `Widget` トレイト

全ウィジェットは共通のトレイトを実装:

```rust
pub trait Widget {
    fn render(&self, input: &StatusLineInput) -> Option<String>;
}
```

必要なデータが存在しない場合は `None` を返します（例: エージェント未使用時の `AgentInfo`）。

## ウィジェット一覧

| ウィジェット | 表示内容 | `None` を返す条件 |
|---|---|---|
| `ModelInfo` | モデル表示名。`short` オプションで括弧付きサフィックスを除去 | なし |
| `CostSummary` | コスト、経過時間、変更行数。アイコンプレフィックス設定可能 (`cost_prefix`, `duration_prefix`, `lines_prefix`) | なし |
| `ContextUsage` | コンテキストウィンドウのプログレスバー + トークン数 | なし |
| `TokenAlert` | 200kトークン超過時の警告 | トークンが閾値未満 |
| `VimStatus` | 現在のVimモード (NORMAL / INSERT)。モードごとの背景色/前景色設定に対応 | Vimモード無効時 |
| `AgentInfo` | アクティブなエージェント名 | エージェント未使用時 |
| `WorktreeInfo` | ワークツリーのブランチまたは名前 | ワークツリーセッション外 |
| `WorkspaceInfo` | 現在の作業ディレクトリ | なし |
| `GitBranch` | 現在のgitブランチ (`git branch --show-current` を実行) | gitリポジトリ外またはdetached HEAD |
| `RateLimit` | レート制限プログレスバー + リセットまでのカウントダウン。`reset_separator` 設定可能 | レート制限データなし (非Claude.ai) |

## 使い方

```rust
use claude_code_statusline_model::StatusLineInput;
use claude_code_statusline_widgets::{Widget, ModelInfo, CostSummary, ContextUsage};

let input: StatusLineInput = serde_json::from_str(&json)?;

let parts: Vec<String> = [
    ModelInfo::default().render(&input),
    CostSummary::default().render(&input),
    ContextUsage::default().render(&input),
]
.into_iter()
.flatten()
.collect();

println!("{}", parts.join(" | "));
```

各ウィジェットは公開フィールドを通じてカスタマイズ可能です。
