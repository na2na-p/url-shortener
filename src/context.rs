use std::sync::Arc;

use juniper::Context as JuniperContext;
use mongodb::Collection;

use crate::dao::shortened_url_dao::ShortenedUrlDao;
use crate::models::shortened_url::ShortenedUrl;

// GraphQLのコンテキスト型
pub struct Context {
    pub db: Collection<ShortenedUrl>,
    pub shortened_url_dao: Arc<ShortenedUrlDao>,
}

impl JuniperContext for Context {}
