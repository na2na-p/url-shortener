use async_recursion::async_recursion;
use juniper::FieldResult;
use rand::Rng;
use crate::context::Context;
use crate::models::shortened_url::ShortenedUrl as ModelShortenedUrl;
use crate::graphql::types::objects::shortened_url::{ShortenedUrl as  GraphQLShortenedUrl , CreateShortenedUrlResponse};
use crate::dao::shortened_url_dao::ShortenedUrlDao;
use crate::graphql::types::objects::expiry_period::ExpiryPeriod;

pub async fn create_shortened_url(
    context: &Context,
    original_url: String,
    custom_short_code: Option<String>,
    expiry_period: Option<ExpiryPeriod>,
) -> FieldResult<CreateShortenedUrlResponse> {
    let dao = ShortenedUrlDao::new(context.db.clone());
    let mut message: String = "".to_string();

    let short_code = if let Some(code) = custom_short_code {
        if is_custom_short_code_unique(&dao, &code).await? {
            code
        } else {
            message = "Custom short code already exists.".to_string();
            generate_unique_short_code(&dao).await?
        }
    } else {
        generate_unique_short_code(&dao).await?
    };

    let new_shortened_url = ModelShortenedUrl {
        id: None,
        original_url,
        short_code,
        expiry_period: expiry_period.unwrap_or(ExpiryPeriod::Unlimited),
        created_at: chrono::Utc::now(),
        expires_at: None,
    };

    let insert_result = dao.create_shortened_url(&new_shortened_url).await?;

    Ok(CreateShortenedUrlResponse {
        shortened_url: GraphQLShortenedUrl {
            id: insert_result.inserted_id.to_string(),
            original_url: new_shortened_url.original_url,
            short_code: new_shortened_url.short_code,
            expiry_period: new_shortened_url.expiry_period,
            created_at: new_shortened_url.created_at.to_rfc3339(),
            expires_at: new_shortened_url.expires_at.map(|dt| dt.to_rfc3339()),
        },
        message: if message.is_empty() { None } else { Some(message) },
    })
}

async fn is_custom_short_code_unique(dao: &ShortenedUrlDao, custom_short_code: &String) -> FieldResult<bool> {
    let existing_shortened_url = dao.find_by_short_code(custom_short_code).await?;
    Ok(existing_shortened_url.is_none())
}

#[async_recursion]
async fn generate_unique_short_code(dao: &ShortenedUrlDao) -> FieldResult<String> {
    let short_code = generate_short_code(None);
    let is_exists = dao.find_by_short_code(&short_code).await?.is_some();

    if is_exists {
        generate_unique_short_code(dao).await
    } else {
        Ok(short_code)
    }
}

fn generate_short_code(_custom_short_code: Option<String>) -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric))
        .map(char::from)
        .take(6)
        .collect()
}
