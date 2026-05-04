-- Migration: Create webhook_logs table for delivery tracking
-- Purpose: Track webhook delivery attempts, success/failure, and retry logic

CREATE TABLE webhook_logs (
    id BIGSERIAL PRIMARY KEY,
    transaction_id VARCHAR(36) NOT NULL,
    webhook_url TEXT NOT NULL,
    attempt_number INTEGER NOT NULL DEFAULT 1,
    http_status_code INTEGER,
    response_body TEXT,
    error_message TEXT,
    delivered_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Index for transaction_id (query all attempts for a transaction)
CREATE INDEX idx_webhook_logs_transaction_id ON webhook_logs(transaction_id);

-- Index for delivered_at (find failed deliveries for retry)
CREATE INDEX idx_webhook_logs_delivered_at ON webhook_logs(delivered_at);

-- Index for cleanup (delete old logs)
CREATE INDEX idx_webhook_logs_created_at ON webhook_logs(created_at);

-- Optional: Add comments for documentation
COMMENT ON TABLE webhook_logs IS 'Tracks webhook delivery attempts for debugging and monitoring';
COMMENT ON COLUMN webhook_logs.transaction_id IS 'UUID of the transaction this webhook is for';
COMMENT ON COLUMN webhook_logs.webhook_url IS 'The URL we attempted to POST to';
COMMENT ON COLUMN webhook_logs.attempt_number IS 'Retry attempt number (1, 2, or 3)';
COMMENT ON COLUMN webhook_logs.http_status_code IS 'HTTP response code from webhook endpoint';
COMMENT ON COLUMN webhook_logs.response_body IS 'Response body from webhook endpoint (truncated to 1KB)';
COMMENT ON COLUMN webhook_logs.error_message IS 'Error message if delivery failed';
COMMENT ON COLUMN webhook_logs.delivered_at IS 'NULL if failed, timestamp if successful';
