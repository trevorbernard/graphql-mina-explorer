use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery, Response};
use reqwest::blocking::Client;

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/transaction_schema.graphql",
    query_path = "src/transaction_query.graphql",
    response_derives = "Debug",
)]
pub struct TransactionQuery;

fn main() {
    let variables = transaction_query::Variables {
        date_time_gte: Some("2023-05-20T06:00:00Z".to_string()),
        date_time_lte: Some("2023-05-28T06:00:00Z".to_string()),
        memo1: Some("E4YVPwLUR2LrP9tSSi3fjw1svcZys1gJHrGvRefwVTCMbP2NQRqdW".to_string()),
        memo2: Some("E4YVe5wRALCJJ2dGEwRMyH7Z8y1QxzM76M8rXivFo5XbeBJdKryV6".to_string()),
        memo3: Some("E4YbUmaZjNgLgezBD3JzyGKuCn4iugZ5EcXT1JuNTudm5tT4MHvKz".to_string()),
        memo4: Some("E4YbUmaZZqAoUdTZYvZkSmLjHfccTMbb5RnTQHixwRWq2YqLdLZyE".to_string()),
    };
    let client = Client::new();
    let response_body =
    post_graphql::<TransactionQuery, _>(&client, "https://graphql.minaexplorer.com", variables).unwrap();

    println!("{:?}", response_body);
}