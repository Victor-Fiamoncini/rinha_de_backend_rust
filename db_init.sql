CREATE TABLE IF NOT EXISTS persons (
    id       UUID PRIMARY KEY NOT NULL,
    nickname TEXT NOT NULL,
    name     TEXT NOT NULL,
    birth    TEXT NOT NULL,
    stack    TEXT
);
