use graphql_client::reqwest::post_graphql_blocking;
use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;

const MONDAY_URL: &str = "https://api.monday.com/v2";

pub fn get_client() -> Result<Client, reqwest::Error> {
    let monday_api_token =
        std::env::var("MONDAY_API_TOKEN").expect("Missing MONDAY_API_TOKEN env var");
    Client::builder()
        .user_agent("monday-rust/0.1.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("{}", monday_api_token)).unwrap(),
            ))
            .collect(),
        )
        .build()
}
pub fn query<Q: GraphQLQuery>(
    client: &Client,
    variables: Q::Variables,
) -> Result<Response<Q::ResponseData>, reqwest::Error> {
    let res: Response<Q::ResponseData> =
        post_graphql_blocking::<Q, _>(&client, MONDAY_URL, variables).expect("Request failed.");
    Ok(res)
}
