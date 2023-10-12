CREATE TABLE IF NOT EXISTS persons (
    id       UUID PRIMARY KEY NOT NULL,
    nickname VARCHAR(32) UNIQUE NOT NULL,
    name     VARCHAR(100) NOT NULL,
    birth    VARCHAR(10) NOT NULL,
    stack    TEXT
);
