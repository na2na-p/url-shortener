use juniper::GraphQLObject;

use crate::graphql::types::objects::expiry_period::ExpiryPeriod;

#[derive(GraphQLObject)]
#[graphql(description = "A shortened URL")]
pub struct ShortenedUrl {
    pub id: String,
    pub original_url: String,
    pub short_code: String,
    pub custom_short_code: Option<String>,
    pub expiry_period: ExpiryPeriod,
    pub created_at: String,
    pub expires_at: Option<String>,
}

// ここに `ShortenedUrl` 関連のリゾルバーなどがあれば、それを追加
