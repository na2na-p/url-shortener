use juniper::{FieldResult, graphql_object};

use crate::context::Context;
use crate::graphql::types::mutations::create_shortened_url::create_shortened_url;
use crate::graphql::types::objects::expiry_period::ExpiryPeriod;
use crate::graphql::types::objects::shortened_url::{CreateShortenedUrlResponse};

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn create_shortened_url(
        context: &Context,
        original_url: String,
        custom_short_code: Option<String>,
        expiry_period: Option<ExpiryPeriod>,
    ) -> FieldResult<CreateShortenedUrlResponse> {
        create_shortened_url(context, original_url, custom_short_code, expiry_period).await
    }
}
