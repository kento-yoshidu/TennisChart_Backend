CREATE TABLE users (
  id integer,
  name varchar(30)
);

INSERT INTO users VALUES (1, 'Roger Federer');

CREATE TABLE player (
  id  integer,
  name TEXT
);

INSERT INTO player VALUES (1, 'roger_federer');
INSERT INTO player VALUES (2, 'rafael_nadal');
INSERT INTO player VALUES (3, 'novak_dokovic');
INSERT INTO player VALUES (4, 'andy_murray');

CREATE TABLE ao (
  id  integer,
  year  integer,
  winner TEXT,
  running_up  TEXT
);

INSERT INTO ao VALUES (1, 2003, 'andre_agassi', 'test');
INSERT INTO ao VALUES (2, 2004, 'roger_federer', 'test');
INSERT INTO ao VALUES (3, 2005, 'marat_safin', 'test');
