-- Your SQL goes here
ALTER TABLE review_tags ADD CONSTRAINT review_id_name UNIQUE (review_id, name);
