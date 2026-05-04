-- Migration: Create paid_transactions table for audit trail
-- Purpose: Store completed transactions for history and verification
-- Active transactions remain in cache, only paid ones are persisted

CREATE TABLE paid_transactions (
    id BIGSERIAL PRIMARY KEY,
    api_key_id BIGINT NOT NULL REFERENCES api_keys(id) ON DELETE CASCADE,
    transaction_id VARCHAR(36) UNIQUE NOT NULL,
    receive_address VARCHAR(65) NOT NULL,
    amount VARCHAR(50) NOT NULL,
    metadata TEXT,
    redirect_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_transactions_api_key_id ON paid_transactions(api_key_id);
CREATE INDEX idx_transactions_receive_address ON paid_transactions(receive_address);
CREATE INDEX idx_transactions_transaction_id ON paid_transactions(transaction_id);


-- Optional: Add comment for documentation
COMMENT ON TABLE paid_transactions IS 'Stores completed payment transactions for audit trail and history. Active (unpaid) transactions are in cache only.';
COMMENT ON COLUMN paid_transactions.transaction_id IS 'UUID v4 generated when transaction is created';
COMMENT ON COLUMN paid_transactions.receive_address IS 'Nano address receiving the payment (merchant)';
COMMENT ON COLUMN paid_transactions.amount IS 'Amount in Nano (string to preserve precision)';
COMMENT ON COLUMN paid_transactions.metadata IS 'Optional JSON string with custom merchant data';