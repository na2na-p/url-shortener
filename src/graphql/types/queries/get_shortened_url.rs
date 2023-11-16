use juniper::{FieldResult, graphql_value};

use crate::context::Context;
use crate::graphql::types::objects::expiry_period::ExpiryPeriod;
use crate::graphql::types::objects::shortened_url::ShortenedUrl;

pub async fn get_shortened_url(context: &Context, short_code: String) -> FieldResult<ShortenedUrl> {
    let dao = context.shortened_url_dao.clone();
    let model_url = dao.find_by_short_code(&short_code).await?;

    // model_url が None の場合はエラーを返す
    let model_url = model_url.ok_or_else(|| {
        juniper::FieldError::new(
            "Shortened URL not found",
            graphql_value!({ "notFound": "Shortened URL not found" }),
        )
    })?;

    let graphql_url = ShortenedUrl {
        id: model_url.id.unwrap().to_string(),
        original_url: model_url.original_url,
        short_code: model_url.short_code,
        expiry_period: ExpiryPeriod::from(model_url.expiry_period),
        created_at: model_url.created_at.to_rfc3339(),
        expires_at: model_url.expires_at.map(|dt| dt.to_rfc3339()),
    };

    Ok(graphql_url)
}
