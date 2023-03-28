CREATE TYPE qr_status_enum as ENUM ('in_use', 'discarded', 'damaged');

CREATE TABLE qrs (
    id char(4) PRIMARY KEY,
    qr_status qr_status_enum NOT NULL,
);

CREATE TABLE places (
    id uuid PRIMARY KEY,
    place_name varchar(256) NOT NULL,
);

CREATE TABLE containers (
    id uuid PRIMARY KEY,
    container_name varchar(256) NOT NULL,
);

CREATE TABLE onwers (
    id uuid PRIMARY KEY,
    owner_name varchar(256) NOT NULL,
);

CREATE TABLE items (
    id uuid PRIMARY KEY,
    qr_id char(4),
    is_in_use boolean NOT NULL,
    use_place_id uuid,
    store_place_id uuid NOT NULL,
    container_id uuid,
    owner_id uuid NOT NULL,
    FOREIGN KEY (qr_id) REFERENCES qrs (id),
    FOREIGN KEY (use_place_id) REFERENCES places (id),
    FOREIGN KEY (store_place_id) REFERENCES places (id),
    FOREIGN KEY (container_id) REFERENCES containers (id),
    FOREIGN KEY (owner_id) REFERENCES owners (id),
);

CREATE TABLE item_status (
    id uuid PRIMARY KEY,
    status_name varchar(256) NOT NULL,
    details varchar(1024),
);

CREATE TABLE link_item_status (
    id uuid NOT NULL,
    item_id uuid NOT NULL,
    linked_status_id uuid NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items (id) ON DELETE CASCADE,
    FOREIGN KEY (linked_status_id) REFERENCES item_status (id),
);

CREATE INDEX ON link_item_status ( item_id );
CREATE INDEX ON link_item_status ( link_item_status );