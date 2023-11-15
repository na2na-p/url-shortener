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
    dotenv::dotenv().ok();

    let schema = create_schema();
    let context_filter = warp::any().map(move || Context { /* 初期化 */ }).boxed();

    let graphql_route = warp::path("api")
        .and(make_graphql_filter(schema, context_filter));

    let redirect_route = warp::path!("r" / String)
        .and_then(handlers::redirect::redirect_short_url);

    let routes = graphql_route.or(redirect_route);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
