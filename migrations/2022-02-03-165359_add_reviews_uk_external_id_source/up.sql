-- Your SQL goes here
ALTER TABLE reviews ADD CONSTRAINT source_external_id UNIQUE (source, external_id);
