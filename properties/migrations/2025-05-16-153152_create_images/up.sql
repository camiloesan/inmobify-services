create table images (
    id uuid primary key,
    name varchar(255) not null,
    path varchar(255) not null,
    created_at timestamp default NOW () not null,
    property_id uuid not null references properties (id)
);
