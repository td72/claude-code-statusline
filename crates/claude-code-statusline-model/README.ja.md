# claude-code-statusline-model

Claude Codeがステータスラインスクリプトにstdin経由で送信するJSONデータのserde型定義。

[公式ドキュメント](https://code.claude.com/docs/en/statusline)に基づいています。

## 主要な型

- `StatusLineInput` -- 全フィールドを含むルート構造体
- `Model` -- モデル識別子と表示名
- `Workspace` -- 現在のディレクトリ、プロジェクトディレクトリ、追加ディレクトリ
- `Cost` -- セッションコスト(USD)、経過時間、変更行数
- `ContextWindow` / `CurrentUsage` -- コンテキストウィンドウサイズとトークン数
- `Vim` / `VimMode` -- Vimモード状態 (NORMAL / INSERT)
- `Agent` -- アクティブなエージェント名
- `Worktree` -- ワークツリー名、パス、ブランチ情報
- `RateLimits` / `RateLimitWindow` -- Claude.aiレート制限使用状況 (5時間・7日ウィンドウ)

オプショナルフィールド (`vim`, `agent`, `worktree`, `rate_limits`) は `Option` 型で、対応する機能がアクティブな場合のみ存在します。

## 使い方

```rust
use claude_code_statusline_model::StatusLineInput;

let json = std::io::read_to_string(std::io::stdin())?;
let input: StatusLineInput = serde_json::from_str(&json)?;

println!("Model: {}", input.model.display_name);
println!("Cost:  ${:.2}", input.cost.total_cost_usd);

if let Some(vim) = &input.vim {
    println!("Vim mode: {:?}", vim.mode);
}
```
