use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use gql_server_rs::model::{HotelSchema, MutationRoot, QueryRoot, SubscriptionRoot};
use mongodb::{options::ClientOptions, Client};
use type_pubsub::TypePubSub;

async fn index(schema: web::Data<HotelSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

async fn index_ws(
    schema: web::Data<HotelSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn db_client(mongo_url: &str) -> Client {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(mongo_url)
        .await
        .expect("Client options");

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Client with options");

    // List the names of the databases in that deployment.
    println!("Connected to MongoDB at {}\nCollections:", mongo_url);
    for db_name in client
        .list_database_names(None, None)
        .await
        .expect("List databases")
    {
        println!("{}", db_name);
    }

    client
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_url = "mongodb://localhost:27017";
    let playground_address = "127.0.0.1:8080";

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(db_client(mongo_url).await)
        .data(TypePubSub::default())
        .finish();

    println!("Playground at: http://{}", playground_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(playground_address)?
    .run()
    .await
}
