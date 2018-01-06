-- Your SQL goes here
CREATE TABLE todo (
  id SERIAL NOT NULL PRIMARY KEY,
  description TEXT NOT NULL,
  completed BOOL DEFAULT FALSE NOT NULL
);