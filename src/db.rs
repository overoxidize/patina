use std::env;
use diesel::backend::Backend;
use diesel::prelude::*;
use diesel::{r2d2, PgConnection};
use serde::de::DeserializeOwned;
use crate::schema::cryptocurrencys;
use crate::schema::exchanges;
use crate::cryptocurrency::Cryptocurrency;
use crate::exchange::Exchange;


fn get_conn() -> PgConnection {
    dotenv::dotenv().unwrap();

    let db_url = env::var("DATABASE_URL").unwrap();

    PgConnection::establish(&db_url).unwrap()
}

fn all_cryptocurrency(conn: &PgConnection) -> Vec<Cryptocurrency> {
    cryptocurrencys::table.load(conn).unwrap()
}

fn all_cryptocurrency_chronological(conn: &mut PgConnection) -> Vec<Cryptocurrency> {
    cryptocurrencys::table
        .order_by(cryptocurrencys::created_at.asc())
        .load(conn)
        .unwrap()
}

fn cryptocurrency_by_id(conn: &PgConnection, crypto_id: i64) -> Cryptocurrency {
    cryptocurrencys::table
        .filter(cryptocurrencys::id.eq(crypto_id))
        .first(conn)
        .unwrap()
}

fn cryptocurrency_by_name(conn: &PgConnection, crypto_name: String) -> Cryptocurrency {
    cryptocurrencys::table
        .filter(cryptocurrencys::currency_name.eq(crypto_name))
        .first(conn)
        .unwrap()
}

fn recent_cryptocurrency(conn: &PgConnection) -> Vec<Cryptocurrency> {
    let past_day = chrono::Utc::now() - chrono::Duration::days(1);

    cryptocurrencys::table
        .filter(cryptocurrencys::created_at.ge(past_day))
        .load(conn)
        .unwrap()
}

fn all_exchanges(conn: &PgConnection) -> Vec<Exchange> {
    exchanges::table.load(conn).unwrap()
}

fn all_exchanges_chronological(conn: &mut PgConnection) -> Vec<Exchange> {
    exchanges::table
        .order_by(exchanges::created_at.asc())
        .load(conn)
        .unwrap()
}

fn exchange_by_id(conn: &PgConnection, exchange_id: i64) -> Exchange {
    exchanges::table
        .filter(exchanges::id.eq(exchange_id))
        .first(conn)
        .unwrap()
}

fn exchange_by_name(conn: &PgConnection, crypto_name: String) -> Exchange {
    exchanges::table
        .filter(exchanges::exchange_name.eq(crypto_name))
        .first(conn)
        .unwrap()
}

fn recent_exchange(conn: &PgConnection) -> Vec<Exchange > {
    let past_day = chrono::Utc::now() - chrono::Duration::days(1);

    exchanges::table
        .filter(exchanges::created_at.ge(past_day))
        .load(conn)
        .unwrap()
}