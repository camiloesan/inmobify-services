create table property_statuses (
    id serial primary key,
    status_name varchar(32) not null unique
);

insert into property_statuses(status_name) values
    ('available'),
    ('unavailable'),
    ('sold');