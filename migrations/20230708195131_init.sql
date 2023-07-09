CREATE TYPE qr_color AS ENUM (
    'red',
    'orange',
    'brown',
    'light_blue',
    'blue',
    'green',
    'yellow',
    'purple',
    'pink'
);

CREATE TYPE stroge AS ENUM (
  'room101',
  'room102',
  'room206'
);


CREATE TABLE equipment (
    id uuid PRIMARY KEY,
    created_at timestamptz NOT NULL,
    qr_id text NOT NULL,
    qr_color qr_color NOT NULL,
    name text NOT NULL,
    descripiton text,
    model_number text,
    storage stroge,
    usage text,
    usage_season text,
    note text NOT NULL,
    parent_id text NOT NULL
);

