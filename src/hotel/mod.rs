use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use base64ct::{Base64, Encoding};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
pub type HotelSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(SimpleObject, Serialize, Deserialize)]
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
            uuid: "the_uuid".into(),
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
        let uuid = guest_hash(&first_name, &last_name, &date_of_birth);
        let new_guest = Guest {
            first_name,
            last_name,
            date_of_birth,
            uuid,
        };
        let db = ctx.data::<Client>().expect("DB Client").database("hotel");
        let guests = db.collection::<Guest>("guests");
        guests
            .insert_one(&new_guest, None)
            .await
            .expect("Insert doc");
        new_guest.uuid
    }
}

fn guest_hash(first_name: &str, last_name: &str, date_of_birth: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(first_name);
    hasher.update(last_name);
    hasher.update(date_of_birth);
    let hash = hasher.finalize();

    Base64::encode_string(&hash)
}
