-- Your SQL goes here
CREATE TABLE review_tags (
  id INT AUTO_INCREMENT,
  review_id INT NOT NULL,
  name VARCHAR(100) NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY(id),
  INDEX review_tag_name (review_id, name),
  CONSTRAINT review_id_name UNIQUE (review_id, name)
);
