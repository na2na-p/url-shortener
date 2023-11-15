use std::env;
use juniper_warp::make_graphql_filter;
use mongodb::Client;
use warp::{Filter, reject};

use context::Context;

use crate::graphql::schema::create_schema;
use crate::models::shortened_url::ShortenedUrl;

mod graphql;
mod context;
mod handlers;
mod dao;
mod models;

// カスタムリジェクションの定義
#[derive(Debug)]
struct InternalServerError;

impl reject::Reject for InternalServerError {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&mongo_uri).await.expect("Failed to connect to MongoDB");
    let db = client.database("shortener");
    let collection = db.collection::<ShortenedUrl>("shortened_urls");

    let schema = create_schema();
    let context_filter = warp::any().map(move || Context {
        db: collection.clone(),
    }).boxed();

    let graphql_route = warp::path("api")
        .and(warp::post())
        .and(make_graphql_filter(schema, context_filter));

    let redirect_route = warp::path!("r" / String)
        .and_then(handlers::redirect::redirect_short_url);

    let routes = graphql_route.or(redirect_route).recover(handlers::rejection::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
