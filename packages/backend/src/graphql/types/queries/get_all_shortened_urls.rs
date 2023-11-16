use juniper::FieldResult;

use crate::context::Context;
use crate::graphql::types::objects::shortened_url::ShortenedUrl;

pub async fn get_all_shortened_urls(context: &Context) -> FieldResult<Vec<ShortenedUrl>> {
    let dao = context.shortened_url_dao.clone();
    let model_urls = dao.get_all_shortened_urls().await?;

    // MongoDBのモデルからGraphQLのモデルへの変換
    let graphql_urls = model_urls.into_iter().map(|model_url| {
        ShortenedUrl {
            id: model_url.id.unwrap().to_string(), // MongoDBのObjectIdから文字列への変換が必要
            original_url: model_url.original_url,
            short_code: model_url.short_code,
            expiry_period: model_url.expiry_period,
            created_at: model_url.created_at.to_rfc3339(), // DateTimeからISO8601文字列への変換
            expires_at: model_url.expires_at.map(|dt| dt.to_rfc3339()),
        }
    }).collect();

    Ok(graphql_urls)
}
