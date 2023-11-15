use juniper::FieldResult;

use crate::context::Context;
use crate::graphql::types::objects::expiry_period::ExpiryPeriod;
use crate::graphql::types::objects::shortened_url::ShortenedUrl;

pub fn get_shortened_url(_context: &Context, short_code: String) -> FieldResult<ShortenedUrl> {
    Ok(ShortenedUrl {
        id: "1".to_string(),
        original_url: "https://example.com".to_string(),
        short_code,
        expiry_period: ExpiryPeriod::Unlimited,
        created_at: "2021-01-01T00:00:00Z".to_string(),
        expires_at: None,
    })
}
