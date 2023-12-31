/*
 * Socialapp
 *
 * Socialapp is a generic social network.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: ignacio.gomez.arboleda@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Url {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl Url {
    pub fn new(url: String, alias: String) -> Url {
        Url {
            url,
            alias,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}


