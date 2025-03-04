create table disposition_types (
    id serial primary key,
    disposition varchar(16) not null unique
);

insert into disposition_types(disposition) values
    ('sale'),
    ('lease');