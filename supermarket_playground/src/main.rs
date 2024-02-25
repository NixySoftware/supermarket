use graphql_client::{GraphQLQuery, Response};
use serde::Deserialize;
use std::collections::HashMap;

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/get_typename.graphql",
    response_derives = "Debug"
)]
struct GetTypename;

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: String,
    error_description: String,
}

async fn refresh(
    client: reqwest::Client,
    refresh_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .post("https://api.ah.nl/mobile-auth/v1/auth/token/refresh")
        .json(&HashMap::from([
            ("clientId", "appie-android"),
            ("refreshToken", refresh_token),
        ]))
        .send()
        .await?;

    if response.status().is_server_error() {
        let error = response.json::<ErrorResponse>().await?;
        println!("{:?}", error);
    } else {
        // println!("{}", response.text().await?);
        let data = response.json::<serde_json::Value>().await;
        println!("{data:#?}");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "";
    let refresh_token = "";

    let client = reqwest::Client::builder()
        .user_agent("Appie/8.60.1")
        .build()
        .unwrap();

    let request_body = GetTypename::build_query(get_typename::Variables {});

    let response = client
        .post("https://api.ah.nl/graphql")
        .bearer_auth(access_token)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.json::<ErrorResponse>().await?;
        println!("{:?}", error);

        if error.error == "invalid_token" && error.error_description == "Access token expired" {
            let _ = refresh(client, refresh_token).await;
        }
    } else {
        // println!("{}", response.text().await?);
        let data = response
            .json::<Response<get_typename::ResponseData>>()
            .await?;
        println!("{:?}", data);
    }

    Ok(())
}
