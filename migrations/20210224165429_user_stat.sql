-- Add migration script here
CREATE TABLE user_stat
(
    atcoder_id TEXT NOT NULL,
    streak INT DEFAULT 0 NOT NULL,
    problem_count INT DEFAULT 0 NOT NULL,
    point_sum DOUBLE PRECISION DEFAULT 0.0 NOT NULL,
    CONSTRAINT user_stat_pk PRIMARY KEY (atcoder_id)
);
