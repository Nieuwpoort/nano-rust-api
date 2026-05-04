-- Rollback: Drop paid_transactions table

DROP INDEX IF EXISTS idx_transactions_api_key_id;
DROP INDEX IF EXISTS idx_transactions_receive_address;
DROP INDEX IF EXISTS idx_transactions_transaction_id;
DROP TABLE IF EXISTS paid_transactions;