CREATE TABLE container (
    id uuid PRIMARY KEY,
    qr_id text NOT NULL,
    qr_color text NOT NULL,
    storage text NOT NULL,
    description text NOT NULL
);
