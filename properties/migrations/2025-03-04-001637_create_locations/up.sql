create table locations (
    id serial primary key,
    street varchar(255) not null,
    house_number varchar(255) not null,
    neighborhood varchar(255) not null,
    zip_code varchar(5) not null,
    latitude varchar(255) not null,
    longitude varchar(255) not null,
    city_name varchar(255) not null,
    state_id integer not null references states (id)
);

INSERT INTO
    locations (
        street,
        house_number,
        neighborhood,
        zip_code,
        city_name,
        state_id,
        latitude,
        longitude
    )
VALUES
    (
        'Main Street',
        '123',
        'Downtown',
        '90210',
        'Xalapa',
        1,
        '34.0901',
        '-118.4065'
    ),
    (
        'Oak Avenue',
        '45B',
        'West End',
        '33101',
        'Miami',
        2,
        '25.7751',
        '-80.2105'
    ),
    (
        'Pine Road',
        '678',
        'Uptown',
        '60601',
        'Chicago',
        3,
        '41.8858',
        '-87.6229'
    ),
    (
        'Cedar Lane',
        '910',
        'Eastside',
        '94102',
        'San Francisco',
        4,
        '37.7793',
        '-122.4192'
    ),
    (
        'Elm Street',
        '12',
        'Riverside',
        '75201',
        'Dallas',
        5,
        '32.7876',
        '-96.7994'
    ),
    (
        'Maple Drive',
        '345',
        'Hillcrest',
        '98101',
        'Seattle',
        1,
        '47.6097',
        '-122.3331'
    ),
    (
        'Birch Boulevard',
        '67',
        'Midtown',
        '30303',
        'Atlanta',
        2,
        '33.7557',
        '-84.3906'
    ),
    (
        'Spruce Way',
        '89A',
        'North Park',
        '85001',
        'Phoenix',
        3,
        '33.4484',
        '-112.0740'
    ),
    (
        'Willow Court',
        '101',
        'Southside',
        '77002',
        'Houston',
        4,
        '29.7569',
        '-95.3626'
    ),
    (
        'Ash Street',
        '234',
        'Lakeshore',
        '10001',
        'New York',
        5,
        '40.7505',
        '-73.9965'
    );
