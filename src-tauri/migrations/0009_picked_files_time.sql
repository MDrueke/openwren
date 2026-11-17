-- Version 9: files chosen for attaching are remembered with the time they were chosen,
-- so they can be forgotten once no draft needs them.

ALTER TABLE picked_files ADD COLUMN picked_at INTEGER NOT NULL DEFAULT 0;
