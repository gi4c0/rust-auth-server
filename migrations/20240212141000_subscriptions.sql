-- Add migration script here
CREATE TABLE IF NOT EXISTS subscriptions (
  subscriber_id UUID NOT NULL,
  author_id UUID NOT NULL,

  PRIMARY KEY(subscriber_id, author_id),

  CONSTRAINT fk_subscriber_id
      FOREIGN KEY (subscriber_id)
          REFERENCES users(id),

  CONSTRAINT fk_author_id
      FOREIGN KEY (author_id)
          REFERENCES users(id)
)
