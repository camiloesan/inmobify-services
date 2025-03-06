CREATE TYPE operation_enum AS ENUM ('create', 'update', 'delete');

CREATE TABLE operation_logs (
    id SERIAL PRIMARY KEY,
    "service" VARCHAR(36) NOT NULL, 
    "operation" operation_enum NOT NULL DEFAULT 'create',
    element_id UUID NOT NULL,
    ip INET NOT NULL,
    "user" VARCHAR(128) NOT NULL,
    "date" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
