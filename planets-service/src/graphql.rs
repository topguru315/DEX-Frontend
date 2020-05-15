use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::Context;

use crate::model::{Planet, Storage};

pub struct QueryRoot;

pub type TestSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[async_graphql::Object]
impl QueryRoot {
    async fn planets(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<Planet> {
        ctx.data::<Storage>().planets()
    }
}
