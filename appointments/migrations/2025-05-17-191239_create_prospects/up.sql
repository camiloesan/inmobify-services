CREATE TABLE prospects (
    id UUID PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    last_name varchar(128) not null,
    email VARCHAR(128) NOT NULL,
    phone VARCHAR(10) NOT NULL,
    created_at timestamp default current_timestamp not null,
    property_id UUID NOT NULL,
    owner_id UUID NOT NULL
);
