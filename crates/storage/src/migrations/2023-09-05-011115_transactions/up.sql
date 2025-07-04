CREATE TABLE transactions_types (
    id VARCHAR(32) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL default ''
);

CREATE TABLE transactions
(
    id           VARCHAR(256) NOT NULL PRIMARY KEY,
    chain        VARCHAR(16)  NOT NULL REFERENCES chains (id) ON DELETE CASCADE,
    from_address VARCHAR(256),
    to_address   VARCHAR(256),
    contract     VARCHAR(256),
    memo         VARCHAR(256),
    state        VARCHAR(16)  NOT NULL,
    kind         VARCHAR(16)  NOT NULL REFERENCES transactions_types (id) ON DELETE CASCADE,
    value        VARCHAR(256),
    asset_id     VARCHAR      NOT NULL REFERENCES assets (id) ON DELETE CASCADE,
    fee          VARCHAR(32),
    utxo_inputs  jsonb,
    utxo_outputs jsonb,
    fee_asset_id VARCHAR      NOT NULL REFERENCES assets (id) ON DELETE CASCADE,
    updated_at   timestamp    NOT NULL default current_timestamp,
    created_at   timestamp    NOT NULL default current_timestamp,
    metadata     jsonb
);

SELECT diesel_manage_updated_at('transactions');

CREATE INDEX transactions_created_at_idx ON transactions (created_at DESC);
CREATE INDEX transactions_updated_at_idx ON transactions (updated_at DESC);
CREATE INDEX transactions_kind_idx ON transactions (kind);
CREATE INDEX transactions_chain_idx ON transactions (chain);
CREATE INDEX transactions_asset_id_idx ON transactions (asset_id);

