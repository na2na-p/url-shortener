use juniper::graphql_object;

use crate::context::Context;
use crate::graphql::types::objects::shortened_url::ShortenedUrl;
use crate::graphql::types::queries::get_shortened_url::get_shortened_url;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    // `getShortenedUrl` クエリのリゾルバー
    fn get_shortened_url(context: &Context, short_code: String) -> juniper::FieldResult<ShortenedUrl> {
        get_shortened_url(context, short_code)
    }
}
