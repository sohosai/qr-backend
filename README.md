# 学園祭実行委員 物品管理システム

## 開発

### 環境構築
以下のソフトウェアが必要です。

- Docker
- DockerCompose

```sh
docker-compose up -d
```

### ブランチ名のルール
- 新しい機能の追加: `features/#<issue-number>-<issue-summary>`
- 修正・変更: `fix/#<issue-number>-<issue-summary>`
- バグ修正: `fixbug/#<issue-number>-<issue-summary>`
- 設定ファイルの変更など: `chore/#<issue-number>-<issue-summary>`

### コミットについて
- **日本語でコミットメッセージを書いてください**
  - 全員に変更の内容と意図を正確に共有するため
  - 1行目に必ず変更内容を書いてください
  - 2行目にできるだけ変更理由を書いてください
