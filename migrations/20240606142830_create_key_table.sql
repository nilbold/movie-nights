-- api access keys
CREATE TABLE key(
    -- SHA-256 hash of a given key
    hash bytea PRIMARY KEY,
    -- the user friendly name or nick identifying the key
    name TEXT NOT NULL,
    -- key privileges
    scope TEXT NOT NULL,
    created_at timestamptz NOT NULL,
    last_used timestamptz
);
