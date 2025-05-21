create table users (
    id uuid primary key,
    name varchar(64) not null,
    last_name varchar(128) not null,
    email varchar(128) not null,
    phone varchar(10) not null,
    password varchar(64) not null,
    created_at timestamp default current_timestamp not null
);

INSERT INTO
    users (id, name, last_name, email, phone, password)
VALUES
    (
        '6ba7b810-9dad-11d1-80b4-00c04fd430c8',
        'John',
        'Doe',
        'john.doe@example.com',
        '5551234567',
        'P@ssw0rd123'
    ),
    (
        '7c4f14b8-9e9e-4e9b-8d8c-5e5f5e5f5e5f',
        'Jane',
        'Smith',
        'jane.smith@example.com',
        '5557654321',
        'SecureP@ss22'
    ),
    (
        '8f1c4b9d-4e9e-4e9b-8d8c-6f6f6f6f6f6f',
        'Michael',
        'Johnson',
        'mjohnson@example.com',
        '5559876543',
        'MJLogin2023!'
    ),
    (
        'bf5e789f-d580-4c8d-98c7-7d2ced703ede',
        'Sarah',
        'Williams',
        'swilliams@example.com',
        '5551357924',
        'Williams$456'
    ),
    (
        'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11',
        'Robert',
        'Brown',
        'rbrown@example.com',
        '5552468013',
        'Br0wn!2023'
    ),
    (
        'b1a5e4f8-9e9e-4e9b-8d8c-1e1e1e1e1e1e',
        'Emily',
        'Davis',
        'edavis@example.com',
        '5553692581',
        'DavisEm2023#'
    ),
    (
        'c2d6e4f8-9e9e-4e9b-8d8c-2f2f2f2f2f2f',
        'David',
        'Miller',
        'dmiller@example.com',
        '5554817263',
        'Mill3rD@vid'
    ),
    (
        'd3e7f8a9-9e9e-4e9b-8d8c-3f3f3f3f3f3f',
        'Olivia',
        'Wilson',
        'owilson@example.com',
        '5556472839',
        'Wilson2023$'
    ),
    (
        'e4f8a9b0-9e9e-4e9b-8d8c-4f4f4f4f4f4f',
        'James',
        'Moore',
        'jmoore@example.com',
        '5558273645',
        'M00reJ@mes!'
    ),
    (
        '5e9e4f8f-9e9e-4e9b-8d8c-5e5e5e5e5e5e',
        'Sofia',
        'Taylor',
        'staylor@example.com',
        '5551928374',
        'T@yl0rSof1a'
    );
