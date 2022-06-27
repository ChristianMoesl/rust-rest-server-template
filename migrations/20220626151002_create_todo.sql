-- Add migration script here
CREATE TABLE todo (
    id serial primary key,
    text text not null,
    completed boolean not null
);