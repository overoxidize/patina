use crate::schema::*;
#[derive(Debug, serde::Deserialize, serde::Serialize, diesel::Queryable, QueryableByName)]
#[serde(rename_all = "camelCase")]
#[table_name = "exchanges"]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Exchange {
    pub id: i64,
    pub currencies: Vec<String>,
    pub exchange_volume: i64,
    pub exchange_name: String,
    pub exchange_type: ExchangeTypeEnum,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ExchangeVolume(u32);

#[derive(Debug, serde::Deserialize, serde::Serialize, diesel_derive_enum::DbEnum)] // https://crates.io/crates/diesel-derive-enum
#[serde(rename_all = "camelCase")]

#[DieselType = "Exchange_type_enum"]
pub enum ExchangeTypeEnum {
    Centralized,
    Decentralized,
    Hybrid
}

#[derive(serde::Deserialize)]
pub struct UpdateExchange {
    pub currencies: Option<Vec<String>>,
    pub exchange_name: String,
    pub exchange_type: ExchangeTypeEnum 
    // Rules for the exchange_type field:
        // Centralized -> Decentralized
        // Centralized -> Hybrid
        // Decentralized -> Hybrid
        // Hybrid -> Decentralized
}

#[derive(serde::Deserialize)]
pub struct CreateExchange {
    pub exchange_name: String,
    pub currencies: Option<Vec<String>>,
    pub exchange_type: ExchangeTypeEnum
}

#[derive(serde::Serialize)]
pub struct ExchangeOverview {
    inactive: i64,
    active: i64
}

