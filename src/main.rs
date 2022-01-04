use reqwest::header;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AppSettings {
    resource: String,
    api_version: String,
    tenant_id: String,
    client_id: String,
    client_secret: String,
}

#[tokio::main]
async fn main() {
    println!(
        "{}",
        get_tenant_info().await.unwrap_or_else(|error| {
            panic!("Problem obtaining tenant info: {:?}", error);
        })
    );
}

async fn get_tenant_info() -> Result<reqwest::StatusCode, reqwest::Error> {
    // Step 1: get needed variables
    let file_string =
        std::fs::read_to_string(String::from("appsettings.json")).expect("Failed to open file");
    let appsettings: AppSettings =
        serde_json::from_str(&file_string).expect("appsettings file could not be parsed");

    let client = reqwest::Client::new();

    // Step 2: get the authentication endpoint from the discovery URL
    let wellknown_endpoint = format!(
        "{}/identity/.well-known/openid-configuration",
        appsettings.resource
    );

    let wellknown_info = client.get(wellknown_endpoint).send().await?.text().await?;

    let wellknown_json: Value = serde_json::from_str(&wellknown_info)
        .expect("could not parse wellknown response to a json object");
    let token_endpoint = &wellknown_json["token_endpoint"]
        .to_string()
        .replace("\"", "");

    // Step 3: use the client ID and Secret to get the needed bearer token
    let mut params = HashMap::new();
    params.insert("client_id", appsettings.client_id);
    params.insert("client_secret", appsettings.client_secret);
    params.insert("grant_type", "client_credentials".to_string());

    let token_info = client
        .post(token_endpoint)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let token_info_json: Value = serde_json::from_str(&token_info)
        .expect("could not parse token endpoint response to a json object");
    let access_token = &token_info_json["access_token"]
        .to_string()
        .replace("\"", "");

    // Step 4: test token by calling the base tenant endpoint
    let tenant_endpoint = format!(
        "{}/api/{}/Tenants/{}",
        appsettings.resource, appsettings.api_version, appsettings.tenant_id
    );
    let auth_header = format!("Bearer {}", &access_token);

    let tenant_info = client
        .get(tenant_endpoint)
        .header(header::AUTHORIZATION, auth_header)
        .send()
        .await?;

    Ok(tenant_info.status())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn tenant_info_works() {
        let resp = crate::get_tenant_info().await;

        match resp {
            Ok(status_code) => assert_eq!(status_code, reqwest::StatusCode::OK),
            Err(error) => panic!("Problem obtaining tenant info: {:?}", error),
        }
    }
}
