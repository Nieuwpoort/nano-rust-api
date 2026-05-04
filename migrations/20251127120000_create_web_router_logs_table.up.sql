CREATE TABLE web_router_logs (
    id BIGSERIAL PRIMARY KEY,
    client_ip VARCHAR(45),
    method VARCHAR(10) NOT NULL,
    uri TEXT NOT NULL,
    success BOOLEAN NOT NULL,
    api_key VARCHAR(128),
    user_id BIGINT,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_web_router_logs ON web_router_logs (
    client_ip,
    uri,
    success,
    user_id, 
    timestamp);