-- Add migration script here
CREATE TABLE IF NOT EXISTS articles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    author_id UUID NOT NULL,
    title VARCHAR(100) NOT NULL UNIQUE,
    text TEXT NOT NULL,
    tags TEXT [],
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_author
        FOREIGN KEY (author_id)
            REFERENCES users(id)

)
