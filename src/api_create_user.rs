use base64::Engine as _;
use rust_sdk::apis::configuration::BasicAuth;
use rust_sdk::apis::configuration::Configuration;
use rust_sdk::apis::user_api;
use rust_sdk::models::create_user_request;
use rust_sdk::models::AccessToken;
use std::error::Error;

pub async fn e2e_user_lifecycle(_flags: &[String]) -> Result<(), Box<dyn Error>> {
    // set HTTP_PROXY
    std::env::set_var("HTTP_PROXY", "http://localhost:9090");
    std::env::set_var("HTTPS_PROXY", "http://localhost:9090");

    // create clients
    let mut configuration = Configuration::new();

    let current_unix_nano = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let username: String = "testuser_rust".to_string() + &current_unix_nano.to_string();
    let password = "password".to_string() + &current_unix_nano.to_string();
    let first_name = "test_first_name_rust";
    let last_name = "test_last_name_rust";
    let email = "test_email".to_string() + &current_unix_nano.to_string() + "@test.com";

    // create user
    let create_user_request = create_user_request::CreateUserRequest::new(
        username.clone(),
        password.clone(),
        first_name.to_string(),
        last_name.to_string(),
        email.clone(),
    );

    let res = user_api::create_user(&configuration, create_user_request).await;
    match res {
        Ok(_user) => {}
        Err(e) => {
            println!("failed to create user: {:?}", e);
        }
    }

    let basic_auth = BasicAuth::from((username.to_string(), Some(password.to_string())));

    // add auth to configuration
    configuration.basic_auth = Some(basic_auth);

    // assert basic auth is set
    if configuration.basic_auth == None
        || configuration.clone().basic_auth.unwrap().0 != username.to_string()
    {
        println!("basic auth was not set");
        std::process::exit(1);
    }

    // print configuration
    // configuration.basic_auth = Some((username.clone(), Some(password.clone())));
    configuration.basic_auth = Some((username.clone(), Some(password.clone())));

    let scope_list_users = "socialapp.users.list socialapp.users.read".to_string();
    let token = fetch_oauth_token_client_credentials(
        username.clone().to_string(),
        password.clone().to_string(),
        Vec::from([scope_list_users]),
    )
    .await?;
    configuration.oauth_access_token = Some(token.access_token);
    // list users
    let users = user_api::list_users(&configuration, Some(100), Some(0)).await;

    // Check for error first
    let users = match users {
        Ok(users) => {
            if users.is_empty() {
                println!("no users found");
                std::process::exit(1);
            }
            users
        }
        Err(err) => {
            println!("failed to list users: {:?}", err);
            std::process::exit(1)
        }
    };

    // fetch user by username
    let fetched_user_res = user_api::get_user_by_username(&configuration, &users[0].username).await;
    let fetched_user = match fetched_user_res {
        Ok(user) => user,
        Err(err) => {
            println!("failed to fetch user: {:?}", err);
            std::process::exit(1)
        }
    };

    if fetched_user.username != users[0].username {
        println!("fetched user is not the same as the first user");
        std::process::exit(1)
    }
    if fetched_user.id != users[0].id {
        println!("fetched user is not the same as the first user");
        std::process::exit(1)
    }
    if fetched_user.email != users[0].email {
        println!("fetched user is not the same as the first user");
        std::process::exit(1)
    }

    println!("FINISHED");

    return Ok(());
}

// fetch oauth token and return it as a string
pub async fn fetch_oauth_token_client_credentials(
    client_id: String,
    client_secret: String,
    scopes: Vec<String>,
) -> Result<AccessToken, Box<dyn Error>> {
    let client = reqwest::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    let client_secret_content = format!("{}:{}", client_id, client_secret);
    let base64_token = base64::engine::general_purpose::STANDARD.encode(client_secret_content);
    let authorization_header_value = format!("Basic {}", base64_token);
    headers.insert("Authorization", authorization_header_value.parse()?);
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);

    // set params
    let mut params = std::collections::HashMap::new();
    params.insert("grant_type", "client_credentials");

    // add scoeps
    let scopes_param = scopes.join(" ");
    params.insert("scope", &scopes_param);

    let request = client
        .request(
            reqwest::Method::POST,
            "https://socialapp.gomezignacio.com/v1/oauth/token",
        )
        .headers(headers)
        .form(&params);

    let response = request.send().await?;
    let body = response.text().await?;
    let token: AccessToken = serde_json::from_str(&body)?;
    return Ok(token);
}
