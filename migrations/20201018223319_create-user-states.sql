-- Add migration script here
CREATE TABLE user_states (
    user_id BIGINT NOT NULL PRIMARY KEY,
    points   BIGINT NOT NULL
)
