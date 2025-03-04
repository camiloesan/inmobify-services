create table locations (
    id serial primary key,
    street varchar(255) not null,
    house_number varchar(255) not null,
    neighborhood varchar(255) not null,
    zip_code varchar (5) not null,
    city_id integer not null references cities (id),
    state_id integer not null references states (id)
);

INSERT INTO locations (
    street, 
    house_number, 
    neighborhood, 
    zip_code, 
    city_id, 
    state_id
) VALUES
    ('Main Street', '123', 'Downtown', '90210', 1, 1),
    ('Oak Avenue', '45B', 'West End', '33101', 2, 2),
    ('Pine Road', '678', 'Uptown', '60601', 3, 3),
    ('Cedar Lane', '910', 'Eastside', '94102', 4, 1),
    ('Elm Street', '12', 'Riverside', '75201', 5, 2),
    ('Maple Drive', '345', 'Hillcrest', '98101', 1, 3),
    ('Birch Boulevard', '67', 'Midtown', '30303', 2, 1),
    ('Spruce Way', '89A', 'North Park', '85001', 3, 2),
    ('Willow Court', '101', 'Southside', '77002', 4, 3),
    ('Ash Street', '234', 'Lakeshore', '10001', 5, 1);