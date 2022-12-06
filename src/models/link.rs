use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Link {
    pub url: String,
    pub name: String,
    pub desc: String,
}
