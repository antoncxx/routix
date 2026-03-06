-- Your SQL goes here

CREATE TABLE users (
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(100) NOT NULL UNIQUE,
    password    VARCHAR(255) NOT NULL,
    role        VARCHAR(20)  NOT NULL DEFAULT 'user',
    scopes      TEXT[]       NOT NULL DEFAULT ARRAY[]::TEXT[],
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);