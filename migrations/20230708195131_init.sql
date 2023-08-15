CREATE TABLE fixtures (
    id uuid PRIMARY KEY,
    created_at timestamptz NOT NULL,
    qr_id text NOT NULL,
    qr_color text NOT NULL,
    name text NOT NULL,
    description text,
    model_number text,
    storage text NOT NULL,
    usage text,
    usage_season text,
    note text NOT NULL,
    parent_id text NOT NULL
);

