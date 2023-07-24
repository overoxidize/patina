// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use crate::cryptocurrency::Coin_status_enum;
    use crate::cryptocurrency::Consensus_mechanism_enum;
    use crate::exchange::Exchange_type_enum;

    cryptocurrencys (id) {
        id -> Int8,
        exchange_id -> Int8,
        currency_name -> Text,
        ticker -> Text,
        consensus_mechanism -> Consensus_mechanism_enum,
        currency_volume -> Int8,
        coin_status -> Coin_status_enum,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::cryptocurrency::Coin_status_enum;
    use crate::cryptocurrency::Consensus_mechanism_enum;
    use crate::exchange::Exchange_type_enum;

    exchanges (id) {
        id -> Int8,
        currencies -> Array<Text>,
        exchange_volume -> Int8,
        exchange_name -> Text,
        exchange_type -> Exchange_type_enum,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(cryptocurrencys -> exchanges (exchange_id));

diesel::allow_tables_to_appear_in_same_query!(
    cryptocurrencys,
    exchanges,
);
