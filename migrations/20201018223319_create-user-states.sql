-- Add migration script here
CREATE TABLE user_states (
    user_id BIGINT NOT NULL PRIMARY KEY,
    score   BIGINT NOT NULL
)
