-- Your SQL goes here
--migrations/TIMESTAMP_invitations/up.sql
CREATE TABLE invitations (
                             id UUID NOT NULL PRIMARY KEY,
                             email VARCHAR(100) NOT NULL,
                             expires_at TIMESTAMP NOT NULL
);

--migrations/TIMESTAMP_invitations/down.sql
DROP TABLE invitations;