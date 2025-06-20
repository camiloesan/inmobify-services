create table transaction_types (
    id serial primary key,
    disposition varchar(16) not null unique
);

insert into transaction_types(disposition) values
    ('sale'),
    ('lease');
