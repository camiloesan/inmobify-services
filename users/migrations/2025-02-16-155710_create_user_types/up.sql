create table user_types (
    id serial primary key,
    type varchar(16) not null
);

insert into user_types (type) 
    values 
    ('buyer'),
    ('tenant'),
    ('administrator');