# やること

- [x] オニオンアーキテクチャの理解
- [ ] ユーザネーム作成の FW を作成
  - [x] ユーザネームモデルの作成
    - [x] ドキュメントテスト
  - [x] ユーザネーム作成サービスの作成
  - [x] インターフェース層
    - [x] json デシリアライズ確認
  - [x] アプリケーション層
    - [x] インターフェース層からどうやってデータを渡してもらうか
    - [x] エラーをどうやって返却するか
  - [x] インフラ層
    - [x] インフラ層にエンティティを
    - [x] インデックス作る
    - [x] date を自動更新にする
    - [x] リポジトリの実装
    - [x] username をユニーク&PK にする
    - [x] マイグレーション
  - [x] 全部非同期にする
  - [x] インターフェース層の実装
  - [x] コンストラクタインジェクション
  - [x] insert できてんのに RecordNotInserted が帰ってくるんだがどういうことやねん
    - [x] ID ないとおっちぬってことがわかった
  - [x] 動作確認
- [x] Rust の DI の理解
  - [x] コンストラクタインジェクションする コンポジションルートでコンストラクトする
  - [x] 動的ディスパッチ
- [ ] 残りのコンテキストの実装
  - [x] 投稿
  - [x] ログイン
  - [x] 投稿取得
  - [ ] 使用禁止用語登録
    - [ ] ドメインモデル
    - [ ] ドメインサービス
    - [ ] アプリサービス
    - [ ] インターフェース
    - [ ] インフラ
    - [ ] 動作確認
- [ ] 結合試験コード
- [x] モックテスト
- [ ] トランザクション貼る
- [ ] README に全てをわかりやすく記載して、note.md を削除
- [ ] 全てのコミットを一つにしてしまう
- [ ] この note を削除
