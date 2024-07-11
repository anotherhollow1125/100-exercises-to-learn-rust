// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

mod data;
mod db;
mod store;
mod web;

pub use web::{serve, Config};

// # 設計思想
//
// 「非」同期なので、演習 7-13 で全て Arc<Mutex<T>> にしたのとは反対に、7-11 で採用されているバージョン番号方式(Update日時で管理)を採用したい
// 今回はせっかくの非同期処理であることを利用し、DBを絡ませてみたいと考える
//
// ## 機能1 DBからの取得・DBへの挿入・変更
//
// [ ] sqlite 及び sqlx を用いたDBを利用する
// [ ] ローカルにファイルとして保存し、次回起動時もアクセスできるようにする
// [x] いい機会なので？idには UUIDv7 を利用する
// [x] バージョンは chrono の DateTime<FixedOffset> を利用する
// [ ] 衝突実験やDIにした意味を出すため、axumも使ってみる
//
// ## 機能2 キャッシュ機能
//
// [ ] サーバーをサーバーらしくするため、キャッシュ機能を提供する
// [ ] 具体的には、(今回はおもちゃなので)サーバー起動時からやり取りがあったチケットは全て BTreeMap に保存しておくこととする
// [ ] 挿入時・更新時にはキャッシュとDB両方にアクセスする
// [ ] 取得時は、キャッシュにデータがあればそれを、なければDBから、それでもなければ None を返す
//
// ## DI
//
// [x] 大した手間じゃなさそうだしやってみる
// [x] shaku を利用する
//
// ## エラーハンドリング
//
// [x] 面倒なので anyhow で統括した
//
// ## 備考
//
// [x] ~~面倒なので~~ 整合性担保のために 7-10 及び 7-11 をコピペして開始する。もちろんIDをUuidに変更する等の改変は行うし忠実に従うような真似はしない
