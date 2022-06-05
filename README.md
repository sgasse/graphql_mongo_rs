# GraphQL Server + MongoDB

Build a GraphQL server with a MongoDB as database backend in Rust.

## Build & Run

The project can be built and run with `docker-compose`:

```bash
docker-compose up -d --build
```

## Try it out

After building and running the project via `docker-compose`, navigate to
[http://127.0.0.1:8080/](http://127.0.0.1:8080/) to access the GraphQL
playground.

On the right of the playground, you can find expanders explaining the available
queries, mutations, subscriptions and the data schema.

### Queries

You can query for guests with the snipped below. Note that you won't get any
guest until you create one.

```graphql
query {
  guests {
    firstName
    lastName
  }
}
```

### Mutations

You can create guests via mutations. An example is below.

```graphql
mutation {
  createGuest(firstName: "Jane", lastName: "Doe", dateOfBirth: "1990-01-02")
}
```

### Subscriptions

The GraphQL server also supports subscriptions. To see it in action, it is best
to create two tabs in the playground, subcribe in the first one and create a new
guest in the second one. For the subscription, you can use the snipped below.

```graphql
subscription {
  newGuests {
    firstName
    lastName
    uuid
  }
}
```
