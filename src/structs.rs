use std::fmt;

use uuid::Uuid;
use utils::f64_from_string;

// Public

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub iso: String,
    pub epoch: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "f64_from_string")]
    pub min_size: f64
}

// Private

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Uuid,
    pub currency: String,
    #[serde(deserialize_with = "f64_from_string")]
    pub balance: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub available: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub hold: f64,
    pub profile_id: Uuid
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountHistory {
    pub id: usize,
    pub created_at: String,
    #[serde(deserialize_with = "f64_from_string")]
    pub amount: f64,
    #[serde(deserialize_with = "f64_from_string")]
    pub balance: f64,
    #[serde(rename = "type")]
    pub _type: String,
    pub details: AccountHistoryDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountHistoryDetails {
    order_id: Uuid
}
// Message
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    message: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
