-- Your SQL goes here
--migrations/TIMESTAMP_users/up.sql
CREATE TABLE users (
                       email VARCHAR(100) NOT NULL PRIMARY KEY,
                       hash VARCHAR(122) NOT NULL, --argon hash
                       created_at TIMESTAMP NOT NULL
);

--migrations/TIMESTAMP_users/down.sql
DROP TABLE users;

