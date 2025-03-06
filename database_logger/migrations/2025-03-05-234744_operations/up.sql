CREATE TABLE operations (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(36) NOT NULL
);

INSERT INTO operations (name) VALUES 
    ('create'),
    ('update'),
    ('delete'),
    ('logical_delete');
