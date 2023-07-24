-- Your SQL goes here
CREATE TYPE CONSENSUS_MECHANISM_ENUM AS ENUM ('pos', 'pow');
CREATE TYPE COIN_STATUS_ENUM AS ENUM ('active', 'inactive');

CREATE TABLE IF NOT EXISTS cryptocurrencys (
    id BIGSERIAL PRIMARY KEY,
    exchange_id BIGINT NOT NULL,
    currency_name TEXT NOT NULL,
    ticker TEXT NOT NULL,
    consensus_mechanism CONSENSUS_MECHANISM_ENUM NOT NULL DEFAULT 'pow',
    currency_volume BIGINT NOT NULL,
    coin_status COIN_STATUS_ENUM NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc'),
    CONSTRAINT exchange_fk
        FOREIGN KEY (exchange_id)
        REFERENCES exchanges(id)
        ON DELETE CASCADE
);

INSERT INTO cryptocurrencys
(exchange_id, currency_name, ticker, consensus_mechanism, currency_volume, coin_status)
VALUES
(1, 'Bitcoin', '$BTC', 'pow', 10000, 'active'),
(2, 'Litecoin', '$LTC', 'pow', 10000, 'active'),
(3, 'Ethereum', '$ETH', 'pos', 10000, 'active');