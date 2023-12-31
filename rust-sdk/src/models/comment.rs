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
pub struct Comment {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "like_count", skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "username")]
    pub username: String,
}

impl Comment {
    pub fn new(content: String, username: String) -> Comment {
        Comment {
            id: None,
            content,
            like_count: None,
            created_at: None,
            username,
        }
    }
}


