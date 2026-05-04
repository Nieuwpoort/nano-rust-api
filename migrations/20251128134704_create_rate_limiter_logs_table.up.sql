CREATE TABLE rate_limiter_logs (
    id BIGSERIAL PRIMARY KEY,
    hits INT NOT NULL,
    client_ip VARCHAR(45),
    api_key VARCHAR(128),
    uri TEXT NOT NULL,
    success BOOLEAN NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_rate_limiter_logs ON rate_limiter_logs (
    client_ip,
    api_key,
    uri,
    success,
    timestamp
    );