-- oapth UP

CREATE TABLE author (
  id INT NOT NULL PRIMARY KEY,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL,
  email VARCHAR(100) NOT NULL
);

-- oapth DOWN

DROP TABLE author;
