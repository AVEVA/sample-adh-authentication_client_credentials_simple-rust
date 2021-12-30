use reqwest;
use reqwest::{header, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AppSettings{
    resource: String,
    api_version: String,
    tenant_id: String,
    client_id: String,
    client_secret: String,
}

#[tokio::main]
async fn main() {

    // Step 1: get needed variables 
    let file = std::fs::read_to_string(String::from("appsettings.json")).expect("Failed to open file");
    let appsettings: AppSettings = serde_json::from_str(&file).expect("appsettings file could not be parsed");

    let client = reqwest::Client::new();

    // Step 2: get the authentication endpoint from the discovery URL
    let wellknown_endpoint = format!("{}/identity/.well-known/openid-configuration", appsettings.resource);
    
    let wellknown_info = client.get(wellknown_endpoint)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let wellknown_json: Value = serde_json::from_str(&wellknown_info).unwrap();
    let token_endpoint = &wellknown_json["token_endpoint"].to_string().replace("\"","");

    // Step 3: use the client ID and Secret to get the needed bearer token
    let mut params = HashMap::new();
    params.insert("client_id", appsettings.client_id);
    params.insert("client_secret", appsettings.client_secret);
    params.insert("grant_type", "client_credentials".to_string());

    let token_info = client.post(token_endpoint)
        .form(&params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let token_info_json: Value = serde_json::from_str(&token_info).unwrap();
    let access_token = &token_info_json["access_token"].to_string().replace("\"","");

    // Step 4: test token by calling the base tenant endpoint
    let tenant_endpoint = format!("{}/api/{}/Tenants/{}", appsettings.resource, appsettings.api_version, appsettings.tenant_id);
    let auth_header = format!("Bearer {}", &access_token);

    let tenant_info = client.get(tenant_endpoint)
        .header(header::AUTHORIZATION, auth_header)
        .send()
        .await
        .unwrap();
    
    // test it by making sure we got a valid http status code
    assert_eq!(tenant_info.status(), StatusCode::OK);
}
