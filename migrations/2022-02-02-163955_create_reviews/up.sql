-- Your SQL goes here
CREATE TABLE reviews (
  id INT AUTO_INCREMENT,
  external_author_id VARCHAR(50) NOT NULL,
  external_id VARCHAR(50) NOT NULL,
  source VARCHAR(20) NOT NULL,
  content TEXT,
  tags JSON,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY(id),
  INDEX (external_author_id)
);
