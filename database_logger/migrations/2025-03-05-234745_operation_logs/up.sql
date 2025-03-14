CREATE TABLE operation_logs (
    id SERIAL PRIMARY KEY,
    "service" VARCHAR(36) NOT NULL,
    "operation" INTEGER REFERENCES operations (id) NOT NULL,
    affected_table VARCHAR(36) NOT NULL,
    element_id UUID NOT NULL,
    ip INET NOT NULL,
    "user" VARCHAR(128) NOT NULL,
    "date" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
