ALTER TABLE fluke_user 
ADD COLUMN username TEXT;

UPDATE fluke_user
SET username = split_part(email, '@', 1)
WHERE username IS NULL;

ALTER TABLE fluke_user 
ADD CONSTRAINT unique_username UNIQUE (username);

ALTER TABLE fluke_user 
ALTER COLUMN username SET NOT NULL;
