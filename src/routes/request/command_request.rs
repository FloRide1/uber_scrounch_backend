use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRequest {
    pub location: i32,

    pub items: Vec<CommandItemRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandItemRequest {
    pub id: i32,

    pub amount: i32,
}
