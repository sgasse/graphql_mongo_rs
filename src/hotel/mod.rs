use async_graphql::{ComplexObject, Context, EmptySubscription, Object, Schema, SimpleObject};

pub type HotelSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

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

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_guest(
        &self,
        ctx: &Context<'_>,
        first_name: String,
        last_name: String,
        date_of_birth: String,
    ) -> String {
        println!(
            "Would create guest {} {} born on {}",
            first_name, last_name, date_of_birth
        );
        "guest_hash".into()
    }
}
