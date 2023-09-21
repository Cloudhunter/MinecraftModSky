CREATE TABLE IF NOT EXISTS Profile {
    id INT GENERATED ALWAYS AS IDENTITY,
    first_seen_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    did TEXT UNIQUE,
    has_been_processed BOOLEAN DEFAULT FALSE,
    likely_country_of_living varchar(2) NULL DEFAULT NULL
}

CREATE TABLE IF NOT EXISTS Post (
    id INT GENERATED ALWAYS AS IDENTITY,
    indexed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    cid TEXT UNIQUE,
    uri TEXT UNIQUE,
    author_did TEXT REFERENCES Profile(did)
);

CREATE TABLE IF NOT EXISTS SubscriptionState (
    id INT GENERATED ALWAYS AS IDENTITY,
    service TEXT UNIQUE,
    cursor INT
);
