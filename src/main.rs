use juniper_warp::make_graphql_filter;
use warp::{Filter, reject};

use context::Context;

use crate::graphql::schema::create_schema;

mod graphql;
mod context;
mod handlers;

// カスタムリジェクションの定義
#[derive(Debug)]
struct InternalServerError;

impl reject::Reject for InternalServerError {}

#[tokio::main]
async fn main() {
    // 環境変数をロードする
    dotenv::dotenv().ok();

    // Juniperのスキーマを作成
    let schema = create_schema();


    let context_filter = warp::any().map(move || Context { /* コンテキストの初期化 */ }).boxed();


    // GraphQLのエンドポイント
    let graphql_route = warp::path("api")
        .and(warp::post())
        .and(make_graphql_filter(schema, context_filter));

    // 短縮URLのリダイレクト
    let redirect_route = warp::path!("r" / String)
        .and_then(handlers::redirect::redirect_short_url);

    // すべてのルートを組み合わせる
    let routes = graphql_route.or(redirect_route);

    // サーバーを起動
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
