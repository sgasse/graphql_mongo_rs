use async_graphql::{
    ComplexObject, Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};

pub type HotelSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Guest {
    first_name: String,
    last_name: String,
    date_of_birth: String,
}

#[ComplexObject]
impl Guest {
    async fn guest_hash(&self) -> &str {
        "the_hash".into()
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn guests(&self, ctx: &Context<'_>) -> Vec<Guest> {
        vec![Guest {
            first_name: "John".into(),
            last_name: "Doe".into(),
            date_of_birth: "1991-01-01".into(),
        }]
    }
}
