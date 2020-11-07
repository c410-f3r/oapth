-- oapth UP

CREATE TABLE post (
  id INT NOT NULL PRIMARY KEY,
  author_id INT NOT NULL,
  title VARCHAR(255) NOT NULL,
  description VARCHAR(500) NOT NULL,
  content TEXT NOT NULL,
  DATE date NOT NULL
);

-- oapth DOWN

DROP TABLE post;
