-- Add migration script here
CREATE TABLE submission
(
    id INTEGER NOT NULL,
    epoch_second TIMESTAMP NOT NULL,
    problem_id TEXT NOT NULL,
    content_id TEXT NOT NULL,
    result TEXT NOT NULL,
    atcoder_id TEXT NOT NULL,
    language TEXT NOT NULL,
    point INT NOT NULL,
    length INT NOT NULL,
    execution_time INT NOT NULL,
    account_id BIGINT REFERENCES account(id) ON DELETE CASCADE,
    CONSTRAINT submission_pkey PRIMARY KEY (id)
);
