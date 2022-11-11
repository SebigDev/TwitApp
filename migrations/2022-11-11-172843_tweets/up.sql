-- Your SQL goes here
CREATE TABLE tweets(
    id serial NOT NULL,
    created_at Timestamp NOT NULL,
    message text NOT NULL,
    Constrain tweets_pkey PRIMARY KEY(id)
);