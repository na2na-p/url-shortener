use mongodb::{Collection};
use crate::models::shortened_url::ShortenedUrl;
use juniper::Context as JuniperContext;

// GraphQLのコンテキスト型
pub struct Context {
    // MongoDBのコレクションへの参照を追加
    pub db: Collection<ShortenedUrl>,
}

impl JuniperContext for Context {}
