-- Your SQL goes here
CREATE TABLE todo (
  id INT AUTO_INCREMENT NOT NULL,
  description TEXT NOT NULL,
  completed BOOL DEFAULT FALSE NOT NULL,
  PRIMARY KEY(id)
);