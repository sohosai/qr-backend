-- Add migration script here
CREATE TABLE spot (
    name text PRIMARY KEY,
    area text NOT NULL,
    building text,
    floor int,
    room text
);
