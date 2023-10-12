# 学園祭実行委員 物品管理システム

## 開発

### 環境構築

#### Dockerを使う場合

以下のソフトウェアが必要です。

- Docker
- DockerCompose

以下の5つの環境変数を適切に設定します。

- `POSTGRES_DB`
- `POSTGRES_USER`
- `POSTGRES_PASSWORD`
- `DATABASE_URL`
- `MEILI_MASTER_KEY`

そのうえで以下のコマンドを実行します。

```sh
docker-compose up -d
```

#### ローカルで構築する場合

依存ソフトウェアは以下の通りです。

- cargo
- rustc
- sqlx-cli
- postgesql
- meilisearch

このソフトウェアはRustで書かれています。そのためRustのコンパイラであるrustcとパッケージマネージャ兼ビルドツールであるcargoをインストールする必要があります。
rustupを使用した公式の方法に従うことでそれぞれインストールすることができます（[公式のダウンロードページ](https://www.rust-lang.org/ja/tools/install)）。

このとき、`apt-get install`などでインストールしようとすると構築に失敗する事例が報告されており、**必ず**公式のツールを使うようにしてください。

以下に例をあげますが、必ず公式を参照してください。

```
# Windows
# インストーラをダウンロードして起動

# Mac, Ubuntu
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --help
```


sqlx-cliはcargoを通じてインストールします。

```
cargo install sqlx-cli
```

でインストールができます。


postgresqlは各々の環境に合わせたインストールを行ってください（[公式のダウンロードページ](https://www.postgresql.org/download/)）。

以下に一例を示しますが、公式のページなどを参照するようにしてください。少なくともWindowsの場合はインストール後にpathの設定をする必要があります。

```
# Windows
winget install postgresql -s winget

# Ubuntu
sudo apt-get update
sudo apt-get -y install postgresql

# Mac
brew update
brew install postgresql
```

検索エンジンであるmeilisearchは各々の環境に合わせたインストールを行なってください（[公式のインストール方法説明ページ](https://www.meilisearch.com/docs/learn/getting_started/installation#installation)）。

その後、環境変数

- `MEILI_MASTER_KEY`
- `MEILI_HTTP_ADDR`
- `MEILI_URL`

の2つを設定した上で

```sh
meilisearch --master-key=$MEILI_MASTER_KEY --http-addr=$MEILI_HTTP_ADDR
```

のようにして起動して下さい。ただし、Windowsなどでは具体的なコマンドは異なるかもしれません。


### データベースの設定

postgresqlのURLを`DATABASE_URL`環境変数に設定する必要があります。以下は一例です。

```
# PowerShell
$Env:DATABASE_URL = "postgresql://localhost:5432/postgres?user=postgres&password=postgres"

# bashなど
DATABASE_URL="postgresql://localhost:5432/postgres?user=postgres&password=postgres"
```

sqlx-cliを使いデータベースの作成とマイグレーションの適用を行います。

```
sqlx db create
sqlx migrate run
```

ちなみに、データベースの削除を行いたい場合は

```
sqlx db drop
```

でできます。マイグレーションの作成と動作確認の時に使えると思います。


### SQLの検証について

CIを回す際にはデータベースを構築せずに、データベース内のデータを保存したJSONファイルを元に検証を行います。
そのJSONファイルは

```
cargo sqlx prepare
```

を行うことで生成することができます。
データベースに変更を加えた場合には忘れずに実行してcommitしてください。

詳しくは[Enable building in "offline" mode with `query!()`](https://github.com/launchbadge/sqlx/blob/master/sqlx-cli/README.md#enable-building-in-offline-mode-with-query)を参照してください。

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
- フォーマットとリンターでのチェックを行ってください
  - フォーマットは`cargo fmt`で行えます
  - リンターの適用は`cargo clippy`で行えます
