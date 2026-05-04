CREATE TABLE topup_api_key_credits (
    id BIGSERIAL PRIMARY KEY,
    transaction_id VARCHAR(255) UNIQUE NOT NULL,
    api_key VARCHAR(255) REFERENCES api_keys(api_key) ON DELETE CASCADE,
    topup_amount BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_topup_api_key_credits_transaction_id ON topup_api_key_credits(transaction_id);
CREATE INDEX idx_topup_api_key_credits_api_key ON topup_api_key_credits(api_key);
CREATE INDEX idx_topup_api_key_credits_created_at ON topup_api_key_credits(created_at);