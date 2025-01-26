CREATE TABLE IF NOT EXISTS price_feeds (
    id SERIAL PRIMARY KEY,
    pair VARCHAR(42) NOT NULL,
    price NUMERIC(32, 18) NOT NULL,
    block_number BIGINT NOT NULL,
    timestamp TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_pair ON price_feeds (pair);
CREATE INDEX IF NOT EXISTS idx_timestamp ON price_feeds (timestamp);