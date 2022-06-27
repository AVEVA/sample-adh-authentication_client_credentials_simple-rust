use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AppSettings {
    // This sample requires these variables from the appsettings.json file
    resource: String,
    api_version: String,
    tenant_id: String,
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct WellKnownInfo {
    // When reaching the .well-known endpoint, we only need to know the token endpoint
    token_endpoint: String,
}

#[derive(Deserialize)]
struct TokenInfo {
    // When reaching the token endpoint, we only need to know the access token.
    // A more full featured sample would also handle the expiration time to allow for token reuse across multiple requests
    access_token: String,
}

#[tokio::main]
async fn main() {
    println!(
        "{}",
        get_tenant_info()
            .await
            .expect("Problem obtaining tenant info")
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

    let wellknown_info: WellKnownInfo = client.get(wellknown_endpoint).send().await?.json().await?;

    // Step 3: use the client ID and Secret to get the needed bearer token
    let mut params = HashMap::new();
    params.insert("client_id", appsettings.client_id);
    params.insert("client_secret", appsettings.client_secret);
    params.insert("grant_type", "client_credentials".to_string());

    let token_info: TokenInfo = client
        .post(wellknown_info.token_endpoint)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    // Step 4: test token by calling the base tenant endpoint
    let tenant_endpoint = format!(
        "{}/api/{}/Tenants/{}",
        appsettings.resource, appsettings.api_version, appsettings.tenant_id
    );

    let tenant_info = client
        .get(tenant_endpoint)
        .bearer_auth(token_info.access_token)
        .send()
        .await?;

    Ok(tenant_info.status())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn tenant_info_works() {
        let status_code = crate::get_tenant_info()
            .await
            .expect("Problem obtaining tenant info");
        assert_eq!(status_code, reqwest::StatusCode::Forbidden);
    }
}
