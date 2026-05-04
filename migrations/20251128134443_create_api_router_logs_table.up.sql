CREATE TABLE api_router_logs (
    id BIGSERIAL PRIMARY KEY,
    client_ip VARCHAR(45),
    method VARCHAR(10) NOT NULL,
    uri TEXT NOT NULL,
    success BOOLEAN NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_api_router_logs ON api_router_logs (
    client_ip,
    uri,
    success,
    timestamp
);