CREATE TABLE lending (
    id uuid PRIMARY KEY,
    fixtures_id uuid NOT NULL,
    spot_name text NOT NULL,
    lending_at timestamptz NOT NULL,
    returned_at timestamptz,
    borrower_name text NOT NULL,
    borrower_number int NOT NULL,
    borrower_org text
);
