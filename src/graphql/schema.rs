use juniper::{EmptyMutation, EmptySubscription, RootNode};

use crate::context::Context;
use crate::graphql::types::query::Query;

// ここにミューテーションのリゾルバーを追加する

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::new(),
        EmptySubscription::new(),
    )
}
