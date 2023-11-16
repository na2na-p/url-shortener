use mongodb::{bson::doc, Collection};
use mongodb::results::InsertOneResult;

use crate::models::shortened_url::ShortenedUrl as ModelShortenedUrl;

#[derive(Clone)] // Clone トレイトを追加
pub struct ShortenedUrlDao {
    collection: Collection<ModelShortenedUrl>,
}

impl ShortenedUrlDao {
    pub fn new(collection: Collection<ModelShortenedUrl>) -> Self {
        ShortenedUrlDao { collection }
    }

    pub async fn create_shortened_url(&self, shortened_url: &ModelShortenedUrl) -> mongodb::error::Result<InsertOneResult> {
        let insert_result = self.collection.insert_one(shortened_url, None).await?;
        Ok(insert_result)
    }

    pub async fn find_by_short_code(&self, short_code: &String) -> mongodb::error::Result<Option<ModelShortenedUrl>> {
        self.collection.find_one(doc! { "short_code": short_code }, None).await
    }
}
