create table images (
    id uuid primary key,
    name varchar(255) not null,
    path varchar(255) not null,
    created_at timestamp default NOW () not null,
    property_id uuid not null references properties (id)
);

INSERT INTO
    images (id, name, path, property_id)
VALUES
    (
        '71f3a414-9c1e-4588-a524-8f6a45628a3e',
        'Main image',
        'http://localhost:12000/imf-files/images/img1.jpg',
        '550e8400-e29b-41d4-a716-446655440000'
    ),
    (
        '83b5c45f-1d6b-42a2-94c5-dc3c23c53a2f',
        'Main image',
        'http://localhost:12000/imf-files/images/img2.jpg',
        'f47ac10b-58cc-4372-a567-0e02b2c3d479'
    ),
    (
        '96e7d12c-8a4f-4b37-8367-1f729d7f3de5',
        'Main image',
        'http://localhost:12000/imf-files/images/img3.jpg',
        '6b86b273-ff34-435d-a4d0-5e5e5e5e5e5e'
    ),
    (
        'a9f8e21b-7c6d-4e5f-9384-2b1c76543a0d',
        'Main image',
        'http://localhost:12000/imf-files/images/img4.jpg',
        'd1a5e4f8-9e9e-4e9b-8d8c-7e7e7e7e7e7e'
    ),
    (
        'b2a1f34c-9d5e-4c7b-8a96-3e4f12d78b9c',
        'Main image',
        'http://localhost:12000/imf-files/images/img5.jpg',
        'de196703-f444-4f26-a7e2-72a7aba737b0'
    ),
    (
        'c4d5e6f7-8a9b-4c7d-9e8f-1a2b3c4d5e6f',
        'Main image',
        'http://localhost:12000/imf-files/images/img6.jpg',
        '123e4567-e89b-12d3-a456-426614174000'
    ),
    (
        'd7e8f9a0-b1c2-4d3e-5f6a-7b8c9d0e1f2a',
        'Main image',
        'http://localhost:12000/imf-files/images/img7.jpg',
        '987fc10b-58cc-4372-a567-0e02b2c3d479'
    ),
    (
        'e1f2a3b4-c5d6-7e8f-9a0b-1c2d3e4f5a6b',
        'Main image',
        'http://localhost:12000/imf-files/images/img8.jpg',
        '3c9e4f8d-9e9e-4e9b-8d8c-3e3e3e3e3e3e'
    ),
    (
        'f5e6d7c8-b9a0-1f2e-3d4c-5b6a7c8d9e0f',
        'Main image',
        'http://localhost:12000/imf-files/images/img9.jpg',
        '4d9e4f8e-9e9e-4e9b-8d8c-4e4e4e4e4e4e'
    ),
    (
        '11223344-5566-7788-99aa-bbccddeeff00',
        'Main image',
        'http://localhost:12000/imf-files/images/img10.jpg',
        '5e9e4f8f-9e9e-4e9b-8d8c-5e5e5e5e5e5e'
    );
