-- oapth UP

CREATE TABLE apple (
  id INT NOT NULL PRIMARY KEY,
  weight INT NOT NULL
);

CREATE TABLE coffee (
  id INT NOT NULL PRIMARY KEY
);

-- oapth DOWN

DROP TABLE coffee;
DROP TABLE apple;