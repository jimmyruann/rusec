-- Add up migration script here
CREATE TABLE
    agents (
        id UUID PRIMARY KEY DEFAULT GEN_RANDOM_UUID (),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );