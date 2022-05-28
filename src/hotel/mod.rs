use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

pub type HotelSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Clone)]
pub struct Guest {
    first_name: String,
    last_name: String,
    date_of_birth: String,
    guest_hash: String,
}

#[Object]
impl Guest {
    async fn first_name(&self) -> &str {
        &self.first_name
    }
    async fn last_name(&self) -> &str {
        &self.last_name
    }
    async fn date_of_birth(&self) -> &str {
        &self.date_of_birth
    }
    async fn guest_hash(&self) -> &str {
        &self.guest_hash
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
            guest_hash: "123".into(),
        }]
    }
}
