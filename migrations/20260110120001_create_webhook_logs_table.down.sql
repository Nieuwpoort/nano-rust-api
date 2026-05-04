-- Rollback: Drop webhook_logs table

DROP INDEX IF EXISTS idx_webhook_logs_created_at;
DROP INDEX IF EXISTS idx_webhook_logs_delivered_at;
DROP INDEX IF EXISTS idx_webhook_logs_transaction_id;
DROP TABLE IF EXISTS webhook_logs;
