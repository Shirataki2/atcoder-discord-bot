-- Add migration script here
ALTER TABLE submission RENAME COLUMN content_id TO contest_id;
