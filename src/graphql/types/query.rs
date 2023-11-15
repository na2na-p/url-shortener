use juniper::graphql_object;

use crate::context::Context;
use crate::graphql::types::objects::expiry_period::ExpiryPeriod;
use crate::graphql::types::objects::shortened_url::ShortenedUrl;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    // `getShortenedUrl` クエリのリゾルバー
    fn get_shortened_url(_context: &Context, short_code: String) -> juniper::FieldResult<ShortenedUrl> {
        // データベースから短縮URLを検索するロジックを実装
        // ここでは仮のデータを返します
        Ok(ShortenedUrl {
            id: "1".to_string(),
            original_url: "https://example.com".to_string(),
            short_code,
            custom_short_code: None,
            expiry_period: ExpiryPeriod::Unlimited,
            created_at: "2021-01-01T00:00:00Z".to_string(),
            expires_at: None,
        })
    }
}
