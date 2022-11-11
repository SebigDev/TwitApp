-- Your SQL goes here
CREATE TABLE likes(
    id serial NOT NULL,
    created_at timestamp NOT NULL,
    tweet_id text NOT NULL,
    Constrain likes_pkey PRIMARY KEY(id)
);