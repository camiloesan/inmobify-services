CREATE TABLE transactions (
    id uuid primary key,
    prospect_id uuid not null references prospects(id),
    transaction_type_id integer not null references transaction_types(id),
    date TIMESTAMP default NOW () NOT NULL,
    property_id UUID NOT NULL
);
