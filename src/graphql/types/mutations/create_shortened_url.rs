use juniper::FieldResult;

use crate::context::Context;
use crate::graphql::types::objects::expiry_period::ExpiryPeriod;
use crate::graphql::types::objects::shortened_url::ShortenedUrl;

pub fn create_shortened_url(
    context: &Context,
    original_url: String,
    custom_short_code: Option<String>,
    expiry_period: Option<ExpiryPeriod>,
) -> FieldResult<ShortenedUrl> {
    let short_code = custom_short_code.clone().unwrap_or_else(|| "generated_code".to_string());
    // データベースに新しい短縮URLを作成するロジックを実装
    // ここでは仮のデータを返します
    Ok(ShortenedUrl {
        id: "2".to_string(),
        original_url,
        short_code,
        custom_short_code,
        expiry_period: expiry_period.unwrap_or(ExpiryPeriod::Unlimited),
        created_at: "2021-01-02T00:00:00Z".to_string(),
        expires_at: None,
    })
}
