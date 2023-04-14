-- Your SQL goes here

CREATE TABLE test_summaries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    errors INTEGER NOT NULL,
    failures INTEGER NOT NULL,
    skipped INTEGER NOT NULL,
    tests INTEGER NOT NULL,
    time DECIMAL NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL
)