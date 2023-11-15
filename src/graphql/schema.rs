use juniper::{EmptyMutation, EmptySubscription, RootNode};

use crate::context::Context;
use crate::graphql::types::mutation::Mutation;
use crate::graphql::types::query::Query;

// ここにミューテーションのリゾルバーを追加する

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query,
        Mutation,
        EmptySubscription::new(),
    )
}
