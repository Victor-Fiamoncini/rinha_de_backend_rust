CREATE EXTENSION pg_trgm;

CREATE OR REPLACE FUNCTION generate_search_terms(nickname VARCHAR, name VARCHAR, stack VARCHAR[]) RETURNS TEXT AS $$
BEGIN
    RETURN (nickname || ' ' || name || ' ' || ARRAY_TO_STRING(stack, ' '));
END;
$$ LANGUAGE plpgsql IMMUTABLE;

CREATE TABLE IF NOT EXISTS persons (
    id UUID PRIMARY KEY NOT NULL,
    nickname VARCHAR(32) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    birth VARCHAR(10) NOT NULL,
    stack VARCHAR(32)[],
    search_terms TEXT GENERATED ALWAYS AS (generate_search_terms(nickname, name, stack)) STORED
);

CREATE INDEX persons_search_terms_idx ON persons USING GIST (search_terms gist_trgm_ops);
