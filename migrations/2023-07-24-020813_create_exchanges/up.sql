-- Your SQL goes here
CREATE TYPE EXCHANGE_TYPE_ENUM AS ENUM ('centralized', 'decentralized', 'hybrid');

CREATE TABLE IF NOT EXISTS exchanges (
    id BIGSERIAL PRIMARY KEY,
    currencies TEXT[] NOT NULL,
    exchange_volume BIGINT NOT NULL,
    exchange_name TEXT NOT NULL,
    exchange_type EXCHANGE_TYPE_ENUM NOT NULL DEFAULT 'decentralized',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

INSERT INTO exchanges
(currencies, exchange_volume, exchange_name, exchange_type)
VALUES
(ARRAY ['Bitcoin', 'Litecoin', 'Ethereum'], 800000, 'Coin Pouch', 'decentralized'),
(ARRAY ['Bitcoin', 'Litecoin', 'Ethereum'], 100000, 'Change House', 'decentralized'),
(ARRAY ['Bitcoin', 'Litecoin', 'Ethereum'], 100000, 'Cryptokeeper', 'decentralized');