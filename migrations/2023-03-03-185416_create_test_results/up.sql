-- Your SQL goes here

CREATE TABLE test_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    duration NUMERIC NOT NULL,
    run_at TIMESTAMP WITH TIME ZONE NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    error_message TEXT
)