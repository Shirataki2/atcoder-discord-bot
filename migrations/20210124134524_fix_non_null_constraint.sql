-- Add migration script here
ALTER TABLE submission ALTER COLUMN account_id SET NOT NULL;
