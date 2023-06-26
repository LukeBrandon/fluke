ALTER TABLE fluke_user
ADD COLUMN username TEXT;

UPDATE fluke_user
SET username = 'user_' || id;

ALTER TABLE fluke_user
ALTER COLUMN username SET NOT NULL;

ALTER TABLE fluke_user
ADD CONSTRAINT username_unique UNIQUE (username);
