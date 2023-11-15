use mongodb::{bson::doc, Collection};

use crate::models::shortened_url::ShortenedUrl as ModelShortenedUrl;

pub struct ShortenedUrlDao {
    collection: Collection<ModelShortenedUrl>,
}

impl ShortenedUrlDao {
    pub fn new(collection: Collection<ModelShortenedUrl>) -> Self {
        ShortenedUrlDao { collection }
    }

    pub async fn create_shortened_url(&self, shortened_url: ModelShortenedUrl) -> mongodb::error::Result<()> {
        self.collection.insert_one(shortened_url, None).await?;
        Ok(())
    }

    pub async fn find_by_short_code(&self, short_code: &str) -> mongodb::error::Result<Option<ModelShortenedUrl>> {
        self.collection.find_one(doc! { "short_code": short_code }, None).await
    }
}