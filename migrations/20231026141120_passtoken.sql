CREATE TABLE passtoken (
    token text PRIMARY KEY,
    role text NOT NULL,
    created_at timestamptz NOT NULL,
    limit_days int NOT NULL
);
