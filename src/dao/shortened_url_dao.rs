use juniper::futures::TryStreamExt;
use mongodb::{bson::doc, Collection};
use mongodb::results::InsertOneResult;

use crate::models::shortened_url::ShortenedUrl as ModelShortenedUrl;

#[derive(Clone)]
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

    pub async fn get_all_shortened_urls(&self) -> mongodb::error::Result<Vec<ModelShortenedUrl>> {
        self.collection.find(None, None).await?
            .try_collect::<Vec<_>>().await
    }

    pub async fn delete_by_short_code(&self, short_code: &String) -> mongodb::error::Result<Option<ModelShortenedUrl>> {
        self.collection.find_one_and_delete(doc! { "short_code": short_code }, None).await
    }
}
