use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AppSettings{
    resource: String,
    api_version: String,
    tenant_id: String,
    client_id: String,
    client_secret: String,
}

fn main() {

    let file = std::fs::read_to_string(String::from("appsettings.json")).expect("Failed to open file");
    let app_settings: AppSettings = serde_json::from_str(&file).expect("appsettings file could not be parsed");

    // Step 1: get needed variables 
    let resource = app_settings.resource;
    let api_version = app_settings.api_version;
    let tenant_id = app_settings.tenant_id;
    let client_id = app_settings.client_id;
    let client_secret = app_settings.client_secret;

    // Step 2: get the authentication endpoint from the discovery URL
    
    
    // Step 3: use the client ID and Secret to get the needed bearer token
    
    // Step 4: test token by calling the base tenant endpoint
    
    // test it by making sure we got a valid http status code
}
