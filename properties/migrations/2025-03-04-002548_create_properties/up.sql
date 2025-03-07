create table properties (
    id uuid primary key,
    title varchar(255) not null,
    img_path varchar(255) not null,
    description varchar(1024),
    n_rooms integer not null,
    n_bathrooms integer not null,
    sqm real not null,
    priority int default 0 not null,
    price real not null,
    owner_id uuid not null,
    created_at timestamp default current_timestamp not null,
    location_id integer not null references locations (id),
    property_type_id integer not null references property_types (id),
    disposition_type_id integer not null references disposition_types (id)
);

INSERT INTO properties (
    id, 
    title, 
    img_path,
    description, 
    n_rooms, 
    n_bathrooms, 
    sqm, 
    priority, 
    price, 
    owner_id, 
    created_at, 
    location_id, 
    property_type_id, 
    disposition_type_id
) VALUES
    ('550e8400-e29b-41d4-a716-446655440000', 'Cozy Downtown Loft', 'http://localhost:12006/images/img1.jpg', 'Modern loft with city views', 2, 1, 75.5, 3, 250000.00, '6ba7b810-9dad-11d1-80b4-00c04fd430c8', '2024-01-15 10:30:00', 1, 1, 1),
    ('f47ac10b-58cc-4372-a567-0e02b2c3d479', 'Spacious Family House', 'http://localhost:12006/images/img2.jpg', 'Large home with garden', 4, 3, 180.0, 1, 450000.00, '7c4f14b8-9e9e-4e9b-8d8c-5e5f5e5f5e5f', '2024-02-20 14:15:00', 2, 2, 2),
    ('6b86b273-ff34-435d-a4d0-5e5e5e5e5e5e', 'Luxury Penthouse', 'http://localhost:12006/images/img3.jpg', 'Top-floor suite with terrace', 3, 2, 120.0, 5, 750000.00, '8f1c4b9d-4e9e-4e9b-8d8c-6f6f6f6f6f6f', '2024-03-10 09:00:00', 3, 1, 1),
    ('d1a5e4f8-9e9e-4e9b-8d8c-7e7e7e7e7e7e', 'Suburban Bungalow', 'http://localhost:12006/images/img4.jpg', 'Single-story home with garage', 3, 2, 110.5, 2, 320000.00, 'bf5e789f-d580-4c8d-98c7-7d2ced703ede', '2024-04-05 16:45:00', 4, 2, 2),
    ('de196703-f444-4f26-a7e2-72a7aba737b0', 'Studio Apartment', 'http://localhost:12006/images/img5.jpg', 'Compact living space', 1, 1, 45.0, 0, 150000.00, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '2024-05-12 11:20:00', 5, 1, 1),
    ('123e4567-e89b-12d3-a456-426614174000', 'Rustic Cottage', 'http://localhost:12006/images/img6.jpg', 'Charming retreat near lake', 2, 1, 80.0, 4, 200000.00, 'b1a5e4f8-9e9e-4e9b-8d8c-1e1e1e1e1e1e', '2024-06-18 13:10:00', 6, 2, 2),
    ('987fc10b-58cc-4372-a567-0e02b2c3d479', 'Urban Condo', 'http://localhost:12006/images/img7.jpg', 'Stylish condo in city center', 2, 2, 90.0, 2, 300000.00, 'c2d6e4f8-9e9e-4e9b-8d8c-2f2f2f2f2f2f', '2024-07-22 15:30:00', 7, 1, 1),
    ('3c9e4f8d-9e9e-4e9b-8d8c-3e3e3e3e3e3e', 'Countryside Villa', 'http://localhost:12006/images/img8.jpg', 'Sprawling estate with pool', 5, 4, 250.0, 6, 900000.00, 'd3e7f8a9-9e9e-4e9b-8d8c-3f3f3f3f3f3f', '2024-08-30 08:50:00', 8, 2, 2),
    ('4d9e4f8e-9e9e-4e9b-8d8c-4e4e4e4e4e4e', 'Modern Townhouse', 'http://localhost:12006/images/img9.jpg', 'Multi-level home with patio', 3, 3, 130.0, 3, 400000.00, 'e4f8a9b0-9e9e-4e9b-8d8c-4f4f4f4f4f4f', '2024-09-14 12:00:00', 9, 2, 1),
    ('5e9e4f8f-9e9e-4e9b-8d8c-5e5e5e5e5e5e', 'Beachfront Apartment', 'http://localhost:12006/images/img10.jpg', 'Ocean-view living', 2, 1, 70.0, 5, 350000.00, 'f5f8a9b1-9e9e-4e9b-8d8c-5f5f5f5f5f5f', '2024-10-25 17:25:00', 10, 1, 2);