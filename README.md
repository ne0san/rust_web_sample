# rust_web_sample

適当に作った Rust の Web バックエンドの例

## 必要なものをインストール

### `cargo watch`をインストール

```bash
cargo install cargo-watch
```

### `sea-orm-cli`をインストール

```bash
cargo install sea-orm-cli
```

## DB 初期化

### migrate init を実行

```bash
sea-orm-cli migrate init
```

# アーキテクチャについて

オニオンアーキテクチャ

## インタフェース層

`crates/interface`

ルーティングとか UI とかを配置

アプリケーション層以下に依存

## インフラ層

`crates/infra`

ファイルアクセスや DB アクセスなどを配置

アプリケーション層以下に依存

ドメインモデルへの直接の依存も OK(ORM の Entity をドメインモデルに変換する必要があるため)

リポジトリはここに置く

## アプリケーション層

`crates/application`

インターフェースとドメインとインフラのつなぎをする中間層
リポジトリはここにも置く

データ変換とかはここでやる(入力をドメインモデルに変換など)

## ドメイン層

`crates/domain`

ドメインモデル層にのみ依存する

複数のドメインモデルにまたがるビジネスロジックや、ドメインモデルに依存しないビジネスロジックを書く。

## ドメインモデル層

`crates/domain-model`

ほかの層に依存しない

ドメインモデル毎に、自身の定義と、自身の振る舞いの定義を書く
