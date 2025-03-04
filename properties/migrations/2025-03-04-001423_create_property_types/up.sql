create table property_types (
    id serial primary key,
    type varchar(16) not null unique
);

insert into property_types(type) values
    ('commercial'),
    ('residential');