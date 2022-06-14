use crate::{sha256_hash, PubSub};
use async_graphql::{Context, Object, Schema, SimpleObject, Subscription};
use futures::{stream::TryStreamExt, Stream};
use mongodb::bson::doc;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub type HotelSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
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
        loop {
            match cursor.try_next().await {
                Ok(Some(guest)) => guests.push(guest),
                Ok(None) => break,
                Err(err) => println!("Error in reading cursor: {}", &err),
            }
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

        let pubsub = ctx.data::<PubSub>().expect("Type Pubsub");
        pubsub.read().await.publish(new_guest.clone());
        println!("Created new guest:\n{:?}", &new_guest);
        new_guest.uuid.clone()
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }

    async fn new_guests(&self, ctx: &Context<'_>) -> impl Stream<Item = Guest> {
        let pubsub = ctx.data::<PubSub>().expect("Type Pubsub");
        let stream = pubsub.write().await.subscribe::<Guest>();
        stream
    }
}
