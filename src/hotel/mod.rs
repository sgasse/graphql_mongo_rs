use async_graphql::{ComplexObject, Context, EmptySubscription, Object, Schema, SimpleObject};
use mongodb::Client;
use serde::{Deserialize, Serialize};

pub type HotelSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(SimpleObject, Serialize, Deserialize)]
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
        let client = ctx.data::<Client>().expect("MongoDB client");
        for db_name in client
            .list_database_names(None, None)
            .await
            .expect("List databases")
        {
            println!("DB in guests: {}", db_name);
        }
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
        let new_guest = Guest {
            first_name,
            last_name,
            date_of_birth,
        };
        let db = ctx.data::<Client>().expect("DB Client").database("hotel");
        let guests = db.collection::<Guest>("guests");
        guests
            .insert_one(new_guest, None)
            .await
            .expect("Insert doc");
        "guest_hash".into()
    }
}
