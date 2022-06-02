use crate::sha256_hash;
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Client;
use serde::{Deserialize, Serialize};

pub type HotelSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Guest {
    first_name: String,
    last_name: String,
    date_of_birth: String,
    uuid: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn guests(&self, ctx: &Context<'_>) -> Vec<Guest> {
        let db = ctx
            .data::<Client>()
            .expect("MongoDB client")
            .database("hotel");

        let guests_col = db.collection::<Guest>("guests");
        let mut cursor = guests_col.find(doc! {}, None).await.expect("Get cursor");

        let mut guests: Vec<Guest> = vec![];
        while let Some(guest) = cursor.try_next().await.expect("Receive from cursor") {
            guests.push(guest);
        }
        guests
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
        let uuid = sha256_hash!(&first_name, &last_name, &date_of_birth);
        let new_guest = Guest {
            first_name,
            last_name,
            date_of_birth,
            uuid,
        };
        let db = ctx.data::<Client>().expect("DB Client").database("hotel");
        let guests_col = db.collection::<Guest>("guests");
        guests_col
            .insert_one(&new_guest, None)
            .await
            .expect("Insert doc");
        println!("Created new guest:\n{:?}", &new_guest);
        new_guest.uuid.clone()
    }
}
