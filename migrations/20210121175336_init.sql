-- Add migration script here
CREATE TABLE guild
(
    id BIGINT NOT NULL,
    channel_id BIGINT,
    CONSTRAINT guild_pk PRIMARY KEY (id)
);

CREATE TABLE account
(
    id BIGINT NOT NULL,
    atcoder_id TEXT NOT NULL,
    CONSTRAINT account_pk PRIMARY KEY (id)
);

CREATE TABLE guild_accounts
(
    id SERIAL UNIQUE NOT NULL,
    guild_id BIGINT REFERENCES guild(id) ON DELETE CASCADE,
    account_id BIGINT REFERENCES account(id) ON DELETE CASCADE,
    CONSTRAINT guild_accounts_pk PRIMARY KEY (id)
);
