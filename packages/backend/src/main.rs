use std::env;
use std::sync::Arc;

use juniper_warp::make_graphql_filter;
use log::info;
use mongodb::Client;
use warp::{Filter, reject};

use context::Context;

use crate::dao::shortened_url_dao::ShortenedUrlDao;
use crate::graphql::schema::create_schema;
use crate::handlers::redirect::redirect_short_url;
use crate::models::shortened_url::ShortenedUrl;

mod graphql;
mod context;
mod handlers;
mod dao;
mod models;

#[derive(Debug)]
struct InternalServerError;

impl reject::Reject for InternalServerError {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Starting server...");
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&mongo_uri).await.expect("Failed to connect to MongoDB");
    let db = client.database("shortener");
    let collection = db.collection::<ShortenedUrl>("shortened_urls");

    let dao = Arc::new(ShortenedUrlDao::new(collection.clone()));

    let schema = create_schema();
    let context_filter = warp::any().map({
        let dao = dao.clone();
        move || Context {
            db: collection.clone(),
            shortened_url_dao: dao.clone(),
        }
    }).boxed();

    let graphql_route = warp::path("api")
        .and(warp::post())
        .and(make_graphql_filter(schema, context_filter));

    let redirect_route = warp::path::param()
        .and_then(move |short_code: String| {
            redirect_short_url(dao.clone(), short_code)
        });

    let routes = graphql_route.or(redirect_route).recover(handlers::rejection::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
