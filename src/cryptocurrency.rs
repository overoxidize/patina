#[derive(Debug, serde::Deserialize, serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Cryptocurrency {
    pub id: i64,
    pub exchange_id: i64,
    pub currency_name: String,
    pub ticker: String,
    pub consensus_mechanism: ConsensusMechanismEnum,
    pub currency_volume: i64,
    pub coin_status: CoinStatusEnum,
    pub created_at: chrono::DateTime<chrono::Utc>,
}


#[derive(Debug, serde::Deserialize, serde::Serialize, diesel_derive_enum::DbEnum)]
#[serde(rename_all = "camelCase")]
#[DieselType = "Coin_status_enum"]
pub enum CoinStatusEnum {
    Inactive,
    Active,
}

#[derive(serde::Deserialize, serde::Serialize, diesel_derive_enum::DbEnum)]
#[derive(Debug)]
#[DieselType = "Consensus_mechanism_enum"]
pub enum ConsensusMechanismEnum {
    ProofOfWork,
    ProofOfStake,
    ProofOfIdentity
}

#[derive(serde::Deserialize)]
pub struct UpdateCryptocurrency {
    pub currency_name: String,
    pub ticker: String,
    pub consensus_mechanism: ConsensusMechanismEnum,
    pub exchange_id: Option<Vec<i64>>
}

#[derive(serde::Deserialize)]

pub struct CreateCryptocurrency {
    pub currency_name: String,
    pub ticker: String,
    pub consensus_mechanism: ConsensusMechanismEnum
}