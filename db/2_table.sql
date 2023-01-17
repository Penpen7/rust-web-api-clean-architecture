\c db

CREATE TABLE  db.user (
  mail_address VARCHAR(256),
  user_name VARCHAR(30),
  PRIMARY KEY (mail_address)
);

INSERT INTO db.user VALUES('hoge@example.com', 'hoge');
INSERT INTO db.user VALUES('fuga@example.com', 'fuga');
INSERT INTO db.user VALUES('test@example.com', 'test');
