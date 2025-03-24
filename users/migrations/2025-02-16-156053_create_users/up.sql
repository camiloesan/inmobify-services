create table users (
    id uuid primary key,
    name varchar(64) not null,
    last_name varchar(128) not null,
    email varchar(128) not null,
    phone varchar(10) not null,
    password varchar(64) not null,
    created_at timestamp default current_timestamp not null,
    user_type_id integer not null references user_types (id)
);
