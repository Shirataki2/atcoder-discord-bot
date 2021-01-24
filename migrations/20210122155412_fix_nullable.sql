-- Add migration script here
ALTER TABLE guild_accounts ALTER COLUMN guild_id SET NOT NULL;
ALTER TABLE guild_accounts ALTER COLUMN account_id SET NOT NULL;
