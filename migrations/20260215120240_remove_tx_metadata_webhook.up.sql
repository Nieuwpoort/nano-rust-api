-- Add up migration script here
DROP TABLE IF EXISTS webhook_logs;

ALTER TABLE paid_transactions
DROP COLUMN IF EXISTS metadata;